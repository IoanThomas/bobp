use crate::{result, Error};

pub type Token<'a> = &'a str;
pub type Position = [f32; 3];
pub type TextureCoordinates = [f32; 2];
pub type Normal = [f32; 3];

pub fn position(positions: &mut Vec<Position>, tokens: &[Token]) -> result::Result<()> {
    parse_attribute(positions, tokens)
}

pub fn texture_coordinates(
    texture_coordinates: &mut Vec<TextureCoordinates>,
    tokens: &[Token],
) -> result::Result<()> {
    parse_attribute(texture_coordinates, tokens)
}

pub fn normal(normals: &mut Vec<Normal>, tokens: &[Token]) -> result::Result<()> {
    parse_attribute(normals, tokens)
}

fn parse_attribute<const NUM_COMPONENTS: usize>(
    attributes: &mut Vec<[f32; NUM_COMPONENTS]>,
    tokens: &[Token],
) -> result::Result<()> {
    let components = tokens
        .iter()
        .map(parse_float)
        .collect::<result::Result<Vec<_>>>()?;

    if components.len() != NUM_COMPONENTS {
        return Err(Error::InvalidFormat);
    }

    let attribute = components.try_into().map_err(|_| Error::InvalidFormat)?;
    attributes.push(attribute);

    Ok(())
}

fn parse_float(token: impl AsRef<str>) -> result::Result<f32> {
    Ok(token.as_ref().parse::<f32>()?)
}
