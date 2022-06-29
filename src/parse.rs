use crate::{result, Error};
use std::ops::Deref;

pub fn position(positions: &mut Vec<[f32; 3]>, tokens: &[&str]) -> result::Result<()> {
    parse_value(positions, tokens)
}

pub fn texture_coordinates(
    texture_coordinates: &mut Vec<[f32; 2]>,
    tokens: &[&str],
) -> result::Result<()> {
    parse_value(texture_coordinates, tokens)
}

pub fn normal(normals: &mut Vec<[f32; 3]>, tokens: &[&str]) -> result::Result<()> {
    parse_value(normals, tokens)
}

fn parse_value<const NUM_COMPONENTS: usize>(
    components: &mut Vec<[f32; NUM_COMPONENTS]>,
    tokens: &[&str],
) -> result::Result<()> {
    let values = tokens
        .iter()
        .map(Deref::deref)
        .map(parse_float)
        .collect::<result::Result<Vec<_>>>()?;

    if values.len() != NUM_COMPONENTS {
        return Err(Error::InvalidFormat);
    }

    components.push(values.try_into().map_err(|_| Error::InvalidFormat)?);

    Ok(())
}

pub fn parse_float(token: &str) -> result::Result<f32> {
    Ok(token.parse::<f32>()?)
}
