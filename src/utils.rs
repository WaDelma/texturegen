use process::{Process, ParseError};

pub fn decode_color(s: &str) -> Result<[f32; 3], ParseError> {
    let input = s.split(",").collect::<Vec<_>>();
    if input.len() < 3 {
        return Err(ParseError::Internal);
    }
    Ok([try!(input[0].trim().parse()),
        try!(input[1].trim().parse()),
        try!(input[2].trim().parse())])
}

pub fn encode_color(c: [f32; 3]) -> String {
    format!("{},{},{}", c[0], c[1], c[2])
}
