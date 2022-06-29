use crate::{result, slice};

pub fn position(positions: &mut Vec<[f32; 3]>, tokens: &[&str]) -> result::Result<()> {
    let value1 = slice::parse_float(tokens, 0)?;
    let value2 = slice::parse_float(tokens, 1)?;
    let value3 = slice::parse_float(tokens, 2)?;

    positions.push([value1, value2, value3]);

    Ok(())
}

pub fn texture_coordinates(
    texture_coordinates: &mut Vec<[f32; 2]>,
    tokens: &[&str],
) -> result::Result<()> {
    let value1 = slice::parse_float(tokens, 0)?;
    let value2 = slice::parse_float(tokens, 1)?;

    texture_coordinates.push([value1, value2]);

    Ok(())
}

pub fn normal(normals: &mut Vec<[f32; 3]>, tokens: &[&str]) -> result::Result<()> {
    let value1 = slice::parse_float(tokens, 0)?;
    let value2 = slice::parse_float(tokens, 1)?;
    let value3 = slice::parse_float(tokens, 2)?;

    normals.push([value1, value2, value3]);

    Ok(())
}
