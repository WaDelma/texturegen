use std::num::{ParseFloatError, ParseIntError};

use shader::Context;

pub mod inputs;
pub mod combiners;
pub mod modifiers;

pub use self::inputs::{Constant, Stripes};
pub use self::combiners::Blend;
pub use self::combiners::Type as BlendType;
pub use self::modifiers::EdgeDetect;
pub use self::modifiers::Type as EdgeDetectType;

pub enum ParseError {
    Internal,
    Unknown(String),
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
}

impl From<ParseFloatError> for ParseError {
    fn from(error: ParseFloatError) -> ParseError {
        ParseError::ParseFloatError(error)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> ParseError {
        ParseError::ParseIntError(error)
    }
}

pub enum Setting<'a> {
    Text(&'a mut String),
    Integer(&'a mut u32),
    Float(&'a mut f32),
    Color(&'a mut [f32; 4]),
    Blend(&'a mut BlendType),
}

impl<'a> ToString for Setting<'a> {
    fn to_string(&self) -> String {
        use self::Setting::*;
        match *self {
            Text(ref t) => (*t).clone(),
            Integer(ref i) => format!("{}", i),
            Float(ref f) => format!("{}", f),
            Color(ref c) => format!("{},{},{},{}", c[0], c[1], c[2], c[3]),
            Blend(ref b) => format!("{:?}", b),
        }
    }
}

pub trait Process {
    fn settings(&mut self) -> Vec<(String, Setting)>;
    fn max_in(&self) -> u32;
    fn max_out(&self) -> u32;
    fn shader(&self, context: &mut Context) -> String;
}
