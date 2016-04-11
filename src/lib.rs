extern crate daggy;

use std::rc::Rc;
use std::slice;
use std::cell::RefCell;
use std::collections::HashSet;

use daggy::{PetGraph, NodeIndex};
use daggy::petgraph::graph;

use dag::PortNumbered;
use process::Process;
use shader::{Context, Shader, Source};

pub use dag::{Edge, Port, port};

pub mod process;
mod dag;
mod shader;
mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Added,
    Changed,
    Removed,
}

pub struct TextureGenerator<'a, T: 'a> {
    dag: PortNumbered<Node<T>>,
    listeners: Vec<Box<Fn(&TextureGenerator<'a, T>, NodeIndex, &Source, EventType) + 'a>>,
}

impl<'a, T> TextureGenerator<'a, T> {
    pub fn new() -> TextureGenerator<'a, T> {
        TextureGenerator {
            dag: PortNumbered::new(),
            listeners: vec![],
        }
    }

    pub fn get(&self, node: NodeIndex) -> Option<(Rc<RefCell<Process>>, &T)> {
        self.dag.node_weight(node).map(|n| (n.process.clone(), &n.data))
    }

    pub fn get_mut(&mut self, node: NodeIndex) -> Option<(Rc<RefCell<Process>>, &mut T)> {
        self.dag.node_weight_mut(node).map(|n| (n.process.clone(), &mut n.data))
    }

    pub fn add(&mut self, node: Rc<RefCell<Process>>, data: T) -> NodeIndex {
        let n = self.dag.add_node(Node::new(node, data));
        self.update_dag(n);
        n
    }

    pub fn modify(&self, node: NodeIndex, key: usize, value: String) -> bool {
        if let Some(weight) = self.dag.node_weight(node) {
            let mut process = weight.process.borrow_mut();
            if let Ok(_) = process.modify(key, value) {
                drop(process);
                self.update_dag(node);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn remove(&mut self, node: &NodeIndex) -> Option<Rc<RefCell<Process>>> {
        let children = self.dag.children(*node).map(|n| n.1.node).collect::<Vec<_>>();
        self.dag.remove_outgoing_edges(*node);
        for c in children {
            self.update_dag(c);
        }
        if let Some(n) = self.dag.remove_node(*node) {
            if let Some(program) = n.program.into_inner() {
                for listener in &self.listeners {
                    listener(self, *node, &program, EventType::Removed);
                }
            }
            Some(n.process)
        } else {
            None
        }
    }

    pub fn connect(&mut self, from: Port<u32>, to: Port<u32>) -> bool {
        if let Ok(_) = self.dag.update_edge(from, to) {
            self.update_dag(to.node);
            true
        } else {
            false
        }
    }

    pub fn disconnect(&mut self, to: Port<u32>) -> Option<Port<u32>> {
        if let Some(from) = self.dag.remove_edge_to_port(to) {
            self.update_dag(to.node);
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

    pub fn register_shader_listener<F>(&mut self, fun: F)
        where F: Fn(&TextureGenerator<'a, T>, NodeIndex, &Source, EventType) + 'a
    {
        self.listeners.push(Box::new(fun));
    }

    //TODO: Switch this to use topological order and cache more inteligently.
    fn update_dag(&self, node: NodeIndex) {
        let shader = self.build_shader(node);
        let mut program = self.dag.node_weight(node).unwrap().program.borrow_mut();
        let event = if program.is_some() {
            EventType::Changed
        } else {
            EventType::Added
        };
        for listener in &self.listeners {
            listener(self, node, &shader, event);
        }
        *program = Some(shader);
        for child in self.dag.children(node).map(|n| n.1.node) {
            self.update_dag(child);
        }
    }

    fn build_shader(&self, node: NodeIndex) -> Source {
        let mut result = Shader::new();
        result.add_vertex("gl_Position = matrix * vec4(position, 0, 1);\n");
        result.add_fragment("vec4 one = vec4(1);\n");
        self.gather_shader(&mut result, node, &mut HashSet::new());
        result.add_fragment(format!("color = out_{}_0;\n", node.index()));
        result.build()
    }

    fn gather_shader(&self, shader: &mut Shader, node: NodeIndex, visited: &mut HashSet<NodeIndex>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        let process = self.dag.node_weight(node).expect("Node or it's parent didn't exist.").process.borrow();
        for s in 0..process.max_in() {
            shader.add_fragment(format!("vec4 in_{}_{} = vec4(0);\n", node.index(), s));
        }
        let mut inputs = HashSet::new();
        for (parent, target) in self.dag.parents(node) {
            inputs.insert(target);
            self.gather_shader(shader, parent.node, visited);
            shader.add_fragment(format!("in_{}_{} = out_{}_{};\n", node.index(), target, parent.node.index(), parent.port));
        }
        let mut context = Context::new(node.index(), inputs, process.max_out());
        shader.add_fragment(process.shader(&mut context));
    }
}

pub struct Node<T> {
    data: T,
    process: Rc<RefCell<Process>>,
    program: RefCell<Option<Source>>,
}

impl<T> Node<T> {
    fn new(process: Rc<RefCell<Process>>, data: T) -> Node<T> {
        Node {
            data: data,
            process: process,
            program: RefCell::new(None),
        }
    }
}

pub struct Iter<'a, T: 'a>(slice::Iter<'a, graph::Node<Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Rc<RefCell<Process>>, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|n| (n.weight.process.clone(), &n.weight.data))
    }
}
