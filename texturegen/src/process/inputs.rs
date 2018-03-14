use Col;
use shader::{col, Context};
use process::{Process, Setting, SettingMut};

#[derive(Clone, Debug)]
pub struct Constant {
    color: Col,
}

impl Constant {
    pub fn new(color: Col) -> Box<Process> {
        Box::new(Constant { color: color })
    }
}

impl Process for Constant {
    fn setting(&self, key: &str) -> Setting {
        use process::Setting::*;
        match key {
            "color" => Color(&self.color),
            _ => panic!(),
        }
    }
    fn setting_mut(&mut self, key: &str) -> SettingMut {
        use process::SettingMut::*;
        match key {
            "color" => Color(&mut self.color),
            _ => panic!(),
        }
    }
    fn settings(&self) -> Vec<&'static str> {
        vec!["color"]
    }
    fn max_in(&self) -> u32 {
        0
    }
    fn max_out(&self) -> u32 {
        1
    }
    fn shader(&self, ctx: &mut Context) -> String {
        format!("vec4 {} = {};\n", ctx.output(0), col(self.color))
    }
}

#[derive(Clone, Debug)]
pub struct Stripes {
    ver: u32,
    hor: u32,
    even_col: Col,
    odd_col: Col,
}

impl Stripes {
    pub fn new(ver: u32, hor: u32, even_col: Col, odd_col: Col) -> Box<Process> {
        Box::new(Stripes {
            ver: ver,
            hor: hor,
            even_col: even_col,
            odd_col: odd_col,
        })
    }
}

impl Process for Stripes {
    fn setting(&self, key: &str) -> Setting {
        use process::Setting::*;
        match key {
            "horizontal" => Integer(&self.hor),
            "vertical" => Integer(&self.ver),
            "even color" => Color(&self.even_col),
            "odd color" => Color(&self.odd_col),
            _ => panic!(),
        }
    }
    fn setting_mut(&mut self, key: &str) -> SettingMut {
        use process::SettingMut::*;
        match key {
            "horizontal" => Integer(&mut self.hor),
            "vertical" => Integer(&mut self.ver),
            "even color" => Color(&mut self.even_col),
            "odd color" => Color(&mut self.odd_col),
            _ => panic!(),
        }
    }
    fn settings(&self) -> Vec<&'static str> {
        vec!["horizontal", "vertical", "even color", "odd color"]
    }
    fn max_in(&self) -> u32 {
        0
    }
    fn max_out(&self) -> u32 {
        1
    }
    fn shader(&self, ctx: &mut Context) -> String {
        let mut result = String::new();
        let hor = 1. / self.hor as f64;
        let ver = 1. / self.ver as f64;
        result.push_str(&format!("vec4 {};\n", ctx.output(0)));
        result.push_str(&format!(
            "if(mod(v_tex_coords.x, {}) < {} != mod(v_tex_coords.y, {}) < {}) {{\n",
            2. * ver,
            ver,
            2. * hor,
            hor
        ));
        result.push_str(&format!("{} = {};\n", ctx.output(0), col(self.odd_col)));
        result.push_str("} else {\n");
        result.push_str(&format!("{} = {};\n", ctx.output(0), col(self.even_col)));
        result.push_str("}\n");
        result
    }
}

#[derive(Clone, Debug)]
pub struct VoronoiNoise {
    ver: u32,
    hor: u32,
    seed: u32,
    grid: f32,
    control: f32,
}

impl VoronoiNoise {
    pub fn new(seed: u32, ver: u32, hor: u32, grid: f32, control: f32) -> Box<Process> {
        Box::new(VoronoiNoise {
            seed: seed,
            ver: ver,
            hor: hor,
            grid: grid,
            control: control,
        })
    }
}

impl Process for VoronoiNoise {
    fn setting(&self, key: &str) -> Setting {
        use process::Setting::*;
        match key {
            "horizontal" => Integer(&self.hor),
            "vertical" => Integer(&self.ver),
            "seed" => Integer(&self.seed),
            "grid" => Float(&self.grid),
            "control" => Float(&self.control),
            _ => panic!(),
        }
    }
    fn setting_mut(&mut self, key: &str) -> SettingMut {
        use process::SettingMut::*;
        match key {
            "horizontal" => Integer(&mut self.hor),
            "vertical" => Integer(&mut self.ver),
            "seed" => Integer(&mut self.seed),
            "grid" => Float(&mut self.grid),
            "control" => Float(&mut self.control),
            _ => panic!(),
        }
    }
    fn settings(&self) -> Vec<&'static str> {
        vec!["seed", "horizontal", "vertical", "grid", "control"]
    }
    fn max_in(&self) -> u32 {
        0
    }
    fn max_out(&self) -> u32 {
        1
    }
    fn shader(&self, ctx: &mut Context) -> String {
        let mut result = String::new();
        let temp = ctx.temporary();
        let hor = 1. / self.hor as f32;
        let ver = 1. / self.ver as f32;
        result.push_str(&format!(
            "float {} = iqnoise(v_tex_coords / vec2({}, {}), {}, {});\n",
            temp, hor, ver, self.grid, self.control
        ));
        result.push_str(&format!(
            "vec4 {} = vec4({c}, {c}, {c}, 1.);\n",
            ctx.output(0),
            c = temp
        ));
        result
    }
}

#[derive(Clone, Debug)]
pub struct Noise {
    ver: u32,
    hor: u32,
    seed: u32,
}

impl Noise {
    pub fn new(seed: u32, ver: u32, hor: u32) -> Box<Process> {
        Box::new(Noise {
            seed: seed,
            ver: ver,
            hor: hor,
        })
    }
}

impl Process for Noise {
    fn setting(&self, key: &str) -> Setting {
        use process::Setting::*;
        match key {
            "horizontal" => Integer(&self.hor),
            "vertical" => Integer(&self.ver),
            "seed" => Integer(&self.seed),
            _ => panic!(),
        }
    }
    fn setting_mut(&mut self, key: &str) -> SettingMut {
        use process::SettingMut::*;
        match key {
            "horizontal" => Integer(&mut self.hor),
            "vertical" => Integer(&mut self.ver),
            "seed" => Integer(&mut self.seed),
            _ => panic!(),
        }
    }
    fn settings(&self) -> Vec<&'static str> {
        vec!["seed", "horizontal", "vertical"]
    }
    fn max_in(&self) -> u32 {
        0
    }
    fn max_out(&self) -> u32 {
        1
    }
    fn shader(&self, ctx: &mut Context) -> String {
        let mut result = String::new();
        let temp = ctx.temporary();
        let hor = 1. / self.hor as f32;
        let ver = 1. / self.ver as f32;
        result.push_str(&format!(
            "float {} = snoise({}, v_tex_coords / vec2({}, {}));\n",
            temp, self.seed, hor, ver
        ));
        result.push_str(&format!(
            "vec4 {} = vec4({c}, {c}, {c}, 1.);\n",
            ctx.output(0),
            c = temp
        ));
        result
    }
}
