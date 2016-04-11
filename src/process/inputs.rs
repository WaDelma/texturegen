use std::rc::Rc;
use std::cell::RefCell;

use shader::Context;
use process::{Process, ParseError};
use utils::*;

pub struct Constant {
    constant: [f32; 4],
}

impl Constant {
    pub fn new(constant: [f32; 4]) -> Rc<RefCell<Process>> {
        Rc::new(RefCell::new(Constant {
            constant: constant,
        }))
    }
}

impl Process for Constant {
    fn modify(&mut self, key: usize, value: String) -> Result<(), ParseError> {
        match key {
            0 => self.constant = try!(decode_color(&value)),
            k => panic!("Unknown option: {}", k),
        }
        Ok(())
    }
    fn setting(&self, key: usize) -> String {
        match key {
            0 => encode_color(self.constant),
            k => panic!("Unknown option: {}", k),
        }
    }
    fn settings(&self) -> Vec<String> {
        vec!["color".into()]
    }
    fn max_in(&self) -> u32 {0}
    fn max_out(&self) -> u32 {1}
    fn shader(&self, ctx: &mut Context) -> String {
        let c = self.constant;
        format!("vec4 {} = vec4({}, {}, {}, {});\n", ctx.output(0), c[0], c[1], c[2], c[3])
    }
}

pub struct Stripes {
    ver: f32,
    hor: f32,
    even_col: [f32; 4],
    odd_col: [f32; 4],
}

impl Stripes {
    pub fn new(ver: f32, hor: f32, even_col: [f32; 4], odd_col: [f32; 4]) -> Rc<RefCell<Process>> {
        Rc::new(RefCell::new(Stripes {
            ver: ver,
            hor: hor,
            even_col: even_col,
            odd_col: odd_col,
        }))
    }
}

impl Process for Stripes {
    fn modify(&mut self, key: usize, value: String) -> Result<(), ParseError> {
        match key {
            0 => self.hor = try!(value.trim().parse()),
            1 => self.ver = try!(value.trim().parse()),
            2 => self.even_col = try!(decode_color(&value)),
            3 => self.odd_col = try!(decode_color(&value)),
            k => panic!("Unknown option: {}", k),
        }
        Ok(())
    }
    fn setting(&self, key: usize) -> String {
        match key {
            0 => format!("{}", self.hor),
            1 => format!("{}", self.ver),
            2 => encode_color(self.even_col),
            3 => encode_color(self.odd_col),
            k => panic!("Unknown option: {}", k),
        }
    }
    fn settings(&self) -> Vec<String> {
        vec!["horizontal".into(), "vertical".into(), "even color".into(), "odd color".into()]
    }
    fn max_in(&self) -> u32 {0}
    fn max_out(&self) -> u32 {1}
    fn shader(&self, ctx: &mut Context) -> String {
        let mut result = String::new();
        let ec = self.even_col;
        let oc = self.odd_col;
        result.push_str(&format!("vec4 {};\n", ctx.output(0)));
        result.push_str(&format!("if(mod(v_tex_coords.x, {}) < {} != mod(v_tex_coords.y, {}) < {}) {{\n", self.hor * 2., self.hor, self.ver * 2., self.ver));
        result.push_str(&format!("{} = vec4({}, {}, {}, {});\n", ctx.output(0), oc[0], oc[1], oc[2], oc[3]));
        result.push_str("} else {\n");
        result.push_str(&format!("{} = vec4({}, {}, {}, {});\n", ctx.output(0), ec[0], ec[1], ec[2], ec[3]));
        result.push_str("}\n");
        result
    }
}
