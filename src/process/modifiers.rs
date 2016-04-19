use std::rc::Rc;
use std::cell::RefCell;

use shader::Context;
use process::{Process, ParseError, Setting};
use utils::*;

pub enum Type {
    Sobel,
    FreiChen,
}

pub struct EdgeDetect {
    threshold: f32,
    edtype: Type,
}

impl EdgeDetect {
    pub fn new(threshold: f32, edtype: Type) -> Rc<RefCell<Process>> {
        Rc::new(RefCell::new(EdgeDetect {
            threshold: threshold,
            edtype: edtype,
        }))
    }
}

impl Process for EdgeDetect {
    fn settings(&mut self) -> Vec<(String, Setting)> {
        vec![
        ("threshold".into(), Setting::Float(&mut self.threshold)),
        // ("type".into(), Setting::EdgeDetectType(&mut self.edtype))
        ]
    }
    fn max_in(&self) -> u32 {1}
    fn max_out(&self) -> u32 {1}
    fn shader(&self, ctx: &mut Context) -> String {
        let threshold = self.threshold;
        // TODO: Edge detection using first order methods requires evaluation of parents in 9 different places.
        format!("vec4 {} = vec4({}, {}, {}, {});\n", ctx.output(0), threshold, threshold, threshold, threshold,)
    }
}
