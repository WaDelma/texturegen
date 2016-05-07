use shader::Context;
use process::{Process, Setting, SettingMut};

pub enum Type {
    Sobel,
    FreiChen,
}

pub struct EdgeDetect {
    threshold: f32,
    edtype: Type,
}

impl EdgeDetect {
    pub fn new(threshold: f32, edtype: Type) -> Box<Process + Sized> {
        Box::new(EdgeDetect {
            threshold: threshold,
            edtype: edtype,
        })
    }
}

impl Process for EdgeDetect {
    fn setting(&self, key: &str) -> Setting {
        use process::Setting::*;
        match key {
            "threshold" => Float(&self.threshold),
            _ => panic!(),
        }
    }
    fn setting_mut(&mut self, key: &str) -> SettingMut {
        use process::SettingMut::*;
        match key {
            "threshold" => Float(&mut self.threshold),
            _ => panic!(),
        }
    }
    fn settings(&self) -> Vec<&'static str> {
        vec!["threshold",
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
