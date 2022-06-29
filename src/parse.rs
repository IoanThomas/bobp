use crate::{result, slice};

pub type Token<'a> = &'a str;
pub type Position = [f32; 3];
pub type TextureCoordinates = [f32; 2];
pub type Normal = [f32; 3];

pub fn position(tokens: &[Token]) -> result::Result<Position> {
    let x = slice::parse_i(tokens, 0)?;
    let y = slice::parse_i(tokens, 0)?;
    let z = slice::parse_i(tokens, 0)?;

    Ok([x, y, z])
}

pub fn texture_coordinates(tokens: &[Token]) -> result::Result<TextureCoordinates> {
    let u = slice::parse_i(tokens, 0)?;
    let v = slice::parse_i(tokens, 0)?;

    Ok([u, v])
}

pub fn normal(tokens: &[Token]) -> result::Result<Normal> {
    let x = slice::parse_i(tokens, 0)?;
    let y = slice::parse_i(tokens, 0)?;
    let z = slice::parse_i(tokens, 0)?;

    Ok([x, y, z])
}
