extern crate daggy;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

use std::slice;
use std::collections::HashSet;
use std::ops::Deref;

use daggy::{PetGraph, NodeIndex};
use daggy::petgraph::graph;
use daggy::petgraph::Bfs;

use dag::PortNumbered;
use process::Process;
use shader::{Context, Shader};

pub use shader::Source;
pub use dag::{Edge, Port, port};

pub mod process;
mod dag;
mod shader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Added,
    Changed,
    Removed,
}

pub struct Generator<T> {
    dag: PortNumbered<Node<T>>
}

pub struct GeneratorView<'a, T: 'a> (&'a Generator<T>);

impl<'a, T: 'a> GeneratorView<'a, T> {
    pub fn get(&self, node: NodeIndex) -> Option<(&(Process + Sized), &T)> {
        self.0.dag.node_weight(node).map(|n| (&*n.process, &n.data))
    }
}

impl<'a, T: 'a> Deref for GeneratorView<'a, T> {
    type Target = Generator<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Generator<T> {
    pub fn new() -> Generator<T> {
        Generator {
            dag: PortNumbered::new(),
        }
    }

    pub fn view<F>(&mut self, mut fun: F) -> GeneratorView<T>
        where F: FnMut(Source, &mut T, &Process),
    {
        for node in 0..self.dag.node_count() {
            let node = NodeIndex::new(node);
            if self.dag.node_weight_mut(node).map(|n| n.dirty).unwrap_or(false) {
                self.update_dag(node);
                let program = build_shader(&self.dag, node);
                if let Some(n) = self.dag.node_weight_mut(node) {
                    fun(program, &mut n.data, &*n.process);
                    n.dirty = false;
                }
            }
        }
        GeneratorView(&*self)
    }

    pub fn get(&self, node: NodeIndex) -> Option<(&Box<Process + Sized>, &T)> {
        self.dag.node_weight(node).map(|n| (&n.process, &n.data))
    }

    pub fn get_mut(&mut self, node: NodeIndex) -> Option<(&mut Box<Process + Sized>, &mut T)> {
        self.dirtify(node); // TODO: This doesn't necessary mutate...
        self.dag.node_weight_mut(node).map(|n| (&mut n.process, &mut n.data))
    }

    pub fn add(&mut self, node: Box<Process + Sized>, data: T) -> NodeIndex {
        let n = self.dag.add_node(Node::new(node, data));
        self.dirtify(n);
        n
    }

    pub fn remove(&mut self, node: &NodeIndex) -> Option<(Box<Process + Sized>, T)> {
        let children = self.dag.children(*node).map(|n| n.1.node).collect::<Vec<_>>();
        self.dag.remove_outgoing_edges(*node);
        for c in children {
            self.dirtify(c);
        }
        if let Some(n) = self.dag.remove_node(*node) {
            Some((n.process, n.data))
        } else {
            None
        }
    }

    pub fn connect(&mut self, from: Port<u32>, to: Port<u32>) -> bool {
        if self.dag.update_edge(from, to).is_ok() {
            self.dirtify(to.node);
            true
        } else {
            false
        }
    }

    pub fn disconnect(&mut self, to: Port<u32>) -> Option<Port<u32>> {
        if let Some(from) = self.dag.remove_edge_to_port(to) {
            self.dirtify(to.node);
            Some(from)
        } else {
            None
        }
    }

    pub fn graph(&self) -> &PetGraph<Node<T>, ::dag::Edge, u32> {
        self.dag.graph()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.dag.raw_nodes().iter())
    }

    pub fn iter_connections(&self) -> ::dag::Edges<u32> {
        self.dag.edges()
    }

    pub fn connections(&self) -> usize {
        self.dag.edge_count()
    }

    fn dirtify(&mut self, node: NodeIndex) {
        let mut bfs = Bfs::new(self.graph(), node);
        while let Some(n) = bfs.next(self.graph()) {
            if let Some(n) = self.dag.node_weight_mut(n) {
                n.dirty = true;
            }
        }
    }

    //TODO: Switch this to use topological order and cache more inteligently.
    fn update_dag(&mut self, node: NodeIndex) {
        let mut stack = vec![node];
        while let Some(node) = stack.pop() {
            self.dag.node_weight_mut(node)
                .unwrap().program = Some(build_shader(&self.dag, node));
            stack.extend(self.dag.children(node).map(|n| n.1.node));
        }
    }
}

fn build_shader<T>(dag: &PortNumbered<Node<T>>, node: NodeIndex) -> Source {
    let mut result = Shader::new();
    result.add_vertex("gl_Position = matrix * vec4(position, 0, 1);\n");
    result.add_fragment("vec4 one = vec4(1);\n");
    gather_shader(dag, &mut result, node, &mut HashSet::new());
    result.add_fragment(format!("color = out_{}_0;\n", node.index()));
    result.build()
}

fn gather_shader<T>(dag: &PortNumbered<Node<T>>, shader: &mut Shader, node: NodeIndex, visited: &mut HashSet<NodeIndex>) {
    if visited.contains(&node) {
        return;
    }
    visited.insert(node);
    let process = &dag.node_weight(node).expect("Node or it's parent didn't exist.").process;
    for s in 0..process.max_in() {
        shader.add_fragment(format!("vec4 in_{}_{} = vec4(0);\n", node.index(), s));
    }
    let mut inputs = HashSet::new();
    for (parent, target) in dag.parents(node) {
        inputs.insert(target);
        gather_shader(dag, shader, parent.node, visited);
        shader.add_fragment(format!("in_{}_{} = out_{}_{};\n", node.index(), target, parent.node.index(), parent.port));
    }
    let mut context = Context::new(node.index(), inputs, process.max_out());
    shader.add_fragment(process.shader(&mut context));
}

pub struct Node<T> {
    data: T,
    process: Box<Process + Sized>,
    program: Option<Source>,
    dirty: bool,
}

impl<T> Node<T> {
    fn new(process: Box<Process + Sized>, data: T) -> Node<T> {
        Node {
            data: data,
            process: process,
            program: None,
            dirty: true,
        }
    }
}

pub struct Iter<'a, T: 'a>(slice::Iter<'a, graph::Node<Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a (Process + Sized), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|n| (&*n.weight.process, &n.weight.data))
    }
}
