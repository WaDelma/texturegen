use std::fmt::{self, Formatter, Display};
use std::collections::hash_map::{self, HashMap};

use palette::rgb::Rgba;

use Col;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub vertex: String,
    pub fragment: String,
}

pub struct Shader {
    vertex_snippets: Vec<String>,
    fragment_snippets: Vec<String>,
}

impl Shader {
    pub fn new() -> Shader {
        Shader {
            vertex_snippets: vec![],
            fragment_snippets: vec![],
        }
    }

    pub fn add_vertex<S: Into<String>>(&mut self, snippet: S) {
        self.vertex_snippets.push(snippet.into());
    }

    pub fn add_fragment<S: Into<String>>(&mut self, snippet: S) {
        self.fragment_snippets.push(snippet.into());
    }

    pub fn build(self) -> Source {
        let mut vertex = String::new();
        vertex.push_str("#version 140\n");
        vertex.push_str("in vec2 position;\n");
        vertex.push_str("in vec2 tex_coords;\n");
        vertex.push_str("uniform mat4 matrix;\n");
        vertex.push_str("out vec2 v_tex_coords;\n");
        vertex.push_str("void main() {\n");
        vertex.push_str("v_tex_coords = tex_coords;\n");
        for snippet in self.vertex_snippets {
            vertex.push_str(&snippet);
        }
        vertex.push_str("}");

        let mut fragment = String::new();
        fragment.push_str("#version 140\n");
        fragment.push_str("in vec2 v_tex_coords;\n");
        fragment.push_str("out vec4 color;\n");
        fragment.push_str(r#"
        //  <https://www.shadertoy.com/view/Xd23Dh>
        //  by inigo quilez <http://iquilezles.org/www/articles/voronoise/voronoise.htm>
        vec3 hash3(vec2 p) {
            vec3 q = vec3(dot(p, vec2(127.1, 311.7)),
                          dot(p, vec2(269.5, 183.3)),
                          dot(p, vec2(419.2, 371.9)));
            return fract(sin(q) * 43758.5453);
        }

        float iqnoise(in vec2 x, float u, float v) {
            vec2 floor = floor(x);
            vec2 fract = fract(x);

            float k = 1. + 63. * pow(1. - v, 4.);

            float va = 0.;
            float wt = 0.;
            for(int x = -2; x <= 2; x++) {
                for(int y = -2; y <= 2; y++) {
                    vec2 offset = vec2(float(x), float(y));
                    vec3 o = hash3(floor + offset) * vec3(u, u, 1.);
                    vec2 r = offset - fract + o.xy;
                    float d = dot(r, r);
                    float ww = pow(1. - smoothstep(0., 1.414, sqrt(d)), k);
                    va += o.z * ww;
                    wt += ww;
                }
            }
            return va / wt;
        }

        //
        // Description : Array and textureless GLSL 2D simplex noise function.
        //      Author : Ian McEwan, Ashima Arts.
        //  Maintainer : ijm
        //     Lastmod : 20110822 (ijm)
        //     License : Copyright (C) 2011 Ashima Arts. All rights reserved.
        //               Distributed under the MIT License. See LICENSE file.
        //               https://github.com/ashima/webgl-noise
        //
        vec3 mod289(vec3 x) {
            return x - floor(x * (1. / 289.)) * 289.;
        }

        vec2 mod289(vec2 x) {
            return x - floor(x * (1. / 289.)) * 289.;
        }

        vec3 permute(vec3 x) {
            return mod289(((x * 34.) + 1.) * x);
        }

        float snoise(float seed, vec2 v) {
            const vec4 C = vec4(0.211324865405187,  // (3.-sqrt(3.))/6.
                                0.366025403784439,  // 0.5*(sqrt(3.)-1.)
                                -0.577350269189626,  // -1. + 2. * C.x
                                0.024390243902439); // 1. / 41.
            // First corner
            vec2 i  = floor(v + dot(v, C.yy));
            vec2 x0 = v - i + dot(i, C.xx);

            // Other corners
            vec2 i1;
            //i1.x = step(x0.y, x0.x); // x0.x > x0.y ? 1. : 0.
            //i1.y = 1. - i1.x;
            i1 = (x0.x > x0.y) ? vec2(1., 0.) : vec2(0., 1.);
            // x0 = x0 - 0. + 0. * C.xx ;
            // x1 = x0 - i1 + 1. * C.xx ;
            // x2 = x0 - 1. + 2. * C.xx ;
            vec4 x12 = x0.xyxy + C.xxzz;
            x12.xy -= i1;

            // Permutations
            i = mod289(i); // Avoid truncation effects in permutation
            vec3 p = permute(permute(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
            p = permute(p + vec3(seed));

            vec3 m = max(0.5 - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), 0.);
            m = m * m;
            m = m * m;

            // Gradients: 41 points uniformly over a line, mapped onto a diamond.
            // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
            vec3 x = 2. * fract(p * C.www) - 1.;
            vec3 h = abs(x) - 0.5;
            vec3 ox = floor(x + 0.5);
            vec3 a0 = x - ox;

            // Normalise gradients implicitly by scaling m
            // Approximation of: m *= inversesqrt(a0 * a0 + h * h);
            m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

            // Compute final noise value at P
            vec3 g;
            g.x = a0.x * x0.x + h.x * x0.y;
            g.yz = a0.yz * x12.xz + h.yz * x12.yw;
            return 130. * dot(m, g) * 0.5 + 0.5;
        }
        "#);
        fragment.push_str("void main() {\n");
        for snippet in self.fragment_snippets {
            fragment.push_str(&snippet);
        }
        fragment.push_str("}");
        // println!("{}", fragment);
        Source {vertex: vertex, fragment: fragment}
    }
}

pub struct Context {
    id: usize,
    inputs: HashMap<u32, Identifier>,
    outputs: HashMap<u32, Identifier>,
    temps: u32,
}

pub struct Inputs<'a>(hash_map::Iter<'a, u32, Identifier>);

impl<'a> Iterator for Inputs<'a> {
    type Item = (u32, Identifier);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(a, b)| (a.clone(), b.clone()))
    }
}

impl Context {
    pub fn new<I: IntoIterator<Item=u32>>(id: usize, inputs: I, outputs: u32) -> Context {
        Context {
            id: id,
            inputs: inputs.into_iter().map(|i| (i, Identifier{id: id, itype: Type::Input, index: i})).collect(),
            outputs: (0..outputs).map(|i| (i as u32, Identifier{id: id, itype: Type::Output, index: i as u32})).collect(),
            temps: 0,
        }
    }

    pub fn input(&self, index: u32) -> Option<Identifier> {
        self.inputs.get(&index).map(Clone::clone)
    }

    pub fn first_input(&self) -> Identifier {
        self.inputs().next().expect("There wasn't any inputs to take first of").1
    }

    pub fn inputs(&self) -> Inputs {
        Inputs(self.inputs.iter())
    }

    pub fn input_len(&self) -> usize {
        self.inputs.len()
    }

    pub fn output(&self, index: u32) -> Identifier {
        *self.outputs.get(&index).expect(&format!("There wasn't output for index: {}", index))
    }

    pub fn temporary(&mut self) -> Identifier {
        let identifier = Identifier {
            id: self.id,
            itype: Type::Temporary,
            index: self.temps,
        };
        self.temps += 1;
        identifier
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Type {
    Input,
    Temporary,
    Output,
}

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        use self::Type::*;
        match *self {
            Input => "in",
            Temporary => "tmp",
            Output => "out",
        }.fmt(fmt)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Identifier {
    id: usize,
    itype: Type,
    index: u32,
}

impl Display for Identifier {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}_{}_{}", self.itype, self.id, self.index)
    }
}

pub fn col(c: Col) -> String {
    let c: Rgba = c.into();
    format!("vec4({}, {}, {}, {})", c.red, c.green, c.blue, c.alpha)
}
