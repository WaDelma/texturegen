use std::rc::Rc;
use std::cell::RefCell;

use shader::Context;
use process::{Process, ParseError, Setting};
use utils::*;

pub struct Constant {
    color: [f32; 4],
}

impl Constant {
    pub fn new(color: [f32; 4]) -> Rc<RefCell<Process>> {
        Rc::new(RefCell::new(Constant {
            color: color,
        }))
    }
}

impl Process for Constant {
    fn settings(&mut self) -> Vec<(String, Setting)> {
        vec![("color".into(), Setting::Color(&mut self.color))]
    }
    fn max_in(&self) -> u32 {0}
    fn max_out(&self) -> u32 {1}
    fn shader(&self, ctx: &mut Context) -> String {
        let c = self.color;
        format!("vec4 {} = vec4({}, {}, {}, {});\n", ctx.output(0), c[0], c[1], c[2], c[3])
    }
}

pub struct Stripes {
    ver: u32,
    hor: u32,
    even_col: [f32; 4],
    odd_col: [f32; 4],
}

impl Stripes {
    pub fn new(ver: u32, hor: u32, even_col: [f32; 4], odd_col: [f32; 4]) -> Rc<RefCell<Process>> {
        Rc::new(RefCell::new(Stripes {
            ver: ver,
            hor: hor,
            even_col: even_col,
            odd_col: odd_col,
        }))
    }
}

impl Process for Stripes {
    fn settings(&mut self) -> Vec<(String, Setting)> {
        vec![
        ("horizontal".into(), Setting::Integer(&mut self.hor)),
        ("vertical".into(), Setting::Integer(&mut self.ver)),
        ("even color".into(), Setting::Color(&mut self.even_col)),
        ("odd color".into(), Setting::Color(&mut self.odd_col))]
    }
    fn max_in(&self) -> u32 {0}
    fn max_out(&self) -> u32 {1}
    fn shader(&self, ctx: &mut Context) -> String {
        let mut result = String::new();
        let ec = self.even_col;
        let oc = self.odd_col;
        let hor = 1. / self.hor as f64;
        let ver = 1. / self.ver as f64;
        result.push_str(&format!("vec4 {};\n", ctx.output(0)));
        result.push_str(&format!("if(mod(v_tex_coords.x, {}) < {} != mod(v_tex_coords.y, {}) < {}) {{\n", 2. * ver, ver, 2. * hor, hor));
        result.push_str(&format!("{} = vec4({}, {}, {}, {});\n", ctx.output(0), oc[0], oc[1], oc[2], oc[3]));
        result.push_str("} else {\n");
        result.push_str(&format!("{} = vec4({}, {}, {}, {});\n", ctx.output(0), ec[0], ec[1], ec[2], ec[3]));
        result.push_str("}\n");
        result
    }
}
