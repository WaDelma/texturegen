use Col;
use shader::Context;

pub mod inputs;
pub mod combiners;
pub mod modifiers;

pub use self::inputs::{Constant, Stripes, VoronoiNoise, Noise};
pub use self::combiners::Blend;
pub use self::combiners::Type as BlendType;
pub use self::modifiers::{EdgeDetect, Select, Invert};
pub use self::modifiers::Type as EdgeDetectType;

pub enum Setting<'a> {
    Text(&'a String),
    Integer(&'a u32),
    Boolean(&'a bool),
    Float(&'a f32),
    Color(&'a Col),
    Blend(&'a BlendType),
}

pub enum SettingMut<'a> {
    Text(&'a mut String),
    Integer(&'a mut u32),
    Boolean(&'a mut bool),
    Float(&'a mut f32),
    Color(&'a mut Col),
    Blend(&'a mut BlendType),
}

impl<'a> ToString for Setting<'a> {
    fn to_string(&self) -> String {
        use self::Setting::*;
        match *self {
            Text(ref t) => (*t).clone(),
            Integer(ref i) => format!("{}", i),
            Boolean(ref b) => format!("{}", if **b {1} else {0}),
            Float(ref f) => format!("{}", f),
            Color(ref c) => format!("{},{},{},{}", c.red, c.green, c.blue, c.alpha),
            Blend(ref b) => format!("{:?}", b),
        }
    }
}

pub trait Process: ProcessClone {
    fn setting(&self, &str) -> Setting;
    fn setting_mut(&mut self, &str) -> SettingMut;
    fn settings(&self) -> Vec<&'static str>;
    fn max_in(&self) -> u32;
    fn max_out(&self) -> u32;
    fn shader(&self, context: &mut Context) -> String;
}

pub trait ProcessClone {
    fn clone_box(&self) -> Box<Process + Sized + 'static>;
}

impl<T> ProcessClone for T where T: 'static + Process + Clone {
    fn clone_box(&self) -> Box<Process + Sized + 'static> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Process + Sized> {
    fn clone(&self) -> Box<Process + Sized> {
        self.clone_box()
    }
}
