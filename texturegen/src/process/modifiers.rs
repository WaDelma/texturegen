use shader::Context;
use process::{Process, Setting, SettingMut};

#[derive(Clone, Debug)]
pub enum Type {
    Sobel,
    FreiChen,
}

#[derive(Clone, Debug)]
pub struct EdgeDetect {
    threshold: f32,
    edtype: Type,
}

impl EdgeDetect {
    pub fn new(threshold: f32, edtype: Type) -> Box<Process> {
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
        vec![
            "threshold",
            // ("type".into(), Setting::EdgeDetectType(&mut self.edtype))
        ]
    }
    fn max_in(&self) -> u32 {
        1
    }
    fn max_out(&self) -> u32 {
        1
    }
    fn shader(&self, ctx: &mut Context) -> String {
        let threshold = self.threshold;
        // TODO: Edge detection using first order methods requires evaluation of parents in 9 different places.
        format!(
            "vec4 {} = vec4({}, {}, {}, {});\n",
            ctx.output(0),
            threshold,
            threshold,
            threshold,
            threshold,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Select {
    threshold: f32,
}

impl Select {
    pub fn new(threshold: f32) -> Box<Process> {
        Box::new(Select {
            threshold: threshold,
        })
    }
}

impl Process for Select {
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
        vec!["threshold"]
    }
    fn max_in(&self) -> u32 {
        3
    }
    fn max_out(&self) -> u32 {
        1
    }
    fn shader(&self, ctx: &mut Context) -> String {
        if let (Some(a), Some(t), Some(b)) = (ctx.input(0), ctx.input(1), ctx.input(2)) {
            let mut res = format!("vec4 {} = {};\n", ctx.output(0), a);
            res.push_str(&format!(
                "if(({t}.r * 0.33 + {t}.g * 0.33 + {t}.b * 0.33) > {}) {{\n",
                self.threshold,
                t = t
            ));
            res.push_str(&format!("  {} = {};\n", ctx.output(0), b));
            res.push_str(&format!("}}\n"));
            res
        } else {
            format!("vec4 {} = vec4(0);\n", ctx.output(0))
        }
    }
}

#[derive(Clone, Debug)]
pub struct Invert {
    alpha: bool,
}

impl Invert {
    pub fn new() -> Box<Process> {
        Box::new(Invert { alpha: false })
    }
}

impl Process for Invert {
    fn setting(&self, key: &str) -> Setting {
        use process::Setting::*;
        match key {
            "alpha" => Boolean(&self.alpha),
            _ => panic!(),
        }
    }
    fn setting_mut(&mut self, key: &str) -> SettingMut {
        use process::SettingMut::*;
        match key {
            "alpha" => Boolean(&mut self.alpha),
            _ => panic!(),
        }
    }
    fn settings(&self) -> Vec<&'static str> {
        vec!["alpha"]
    }
    fn max_in(&self) -> u32 {
        1
    }
    fn max_out(&self) -> u32 {
        1
    }
    fn shader(&self, ctx: &mut Context) -> String {
        if let Some(input) = ctx.input(0) {
            if self.alpha {
                format!(
                    "vec4 {} = vec4({i}.rgb, 1 - {i}.a);\n",
                    ctx.output(0),
                    i = input
                )
            } else {
                format!(
                    "vec4 {} = vec4(1 - {i}.r, 1 - {i}.g, 1 - {i}.b, {i}.a);\n",
                    ctx.output(0),
                    i = input
                )
            }
        } else {
            format!("vec4 {} = vec4(0);\n", ctx.output(0))
        }
    }
}
