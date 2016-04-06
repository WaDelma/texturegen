use std::num::ParseFloatError;

use shader::Context;

pub mod inputs;
pub mod combiners;

pub use self::inputs::Constant;
pub use self::combiners::{Blend, BlendType};

pub enum ParseError {
    Internal,
    Unknown(String),
    ParseFloatError(ParseFloatError),
}

impl From<ParseFloatError> for ParseError {
    fn from(error: ParseFloatError) -> ParseError {
        ParseError::ParseFloatError(error)
    }
}

pub trait Process {
    fn modify(&mut self, key: usize, value: String) -> Result<(), ParseError>;
    fn setting(&self, usize) -> String;
    fn settings(&self) -> Vec<String>;
    fn max_in(&self) -> u32;
    fn max_out(&self) -> u32;
    fn shader(&self, context: &mut Context) -> String;
}
