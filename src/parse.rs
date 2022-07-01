use crate::types::{Normal, Position, TextureCoordinates, Token, VertexKey};
use crate::{result, Error};

pub fn position(tokens: &[Token]) -> result::Result<Position> {
    if tokens.len() != 3 {
        return Err(Error::InvalidFormat);
    }

    let x = tokens[0].parse()?;
    let y = tokens[1].parse()?;
    let z = tokens[2].parse()?;

    Ok([x, y, z])
}

pub fn texture_coordinates(tokens: &[Token]) -> result::Result<TextureCoordinates> {
    if tokens.len() != 2 {
        return Err(Error::InvalidFormat);
    }

    let u = tokens[0].parse()?;
    let v = tokens[1].parse()?;

    Ok([u, v])
}

pub fn normal(tokens: &[Token]) -> result::Result<Normal> {
    if tokens.len() != 3 {
        return Err(Error::InvalidFormat);
    }

    let x = tokens[0].parse()?;
    let y = tokens[1].parse()?;
    let z = tokens[2].parse()?;

    Ok([x, y, z])
}

pub fn face(tokens: &[Token]) -> result::Result<[VertexKey; 3]> {
    if tokens.len() != 3 {
        return Err(Error::InvalidFormat);
    }

    let indices1 = parse_vertex_key(tokens[0])?;
    let indices2 = parse_vertex_key(tokens[1])?;
    let indices3 = parse_vertex_key(tokens[2])?;

    Ok([indices1, indices2, indices3])
}

fn parse_vertex_key(token: Token) -> result::Result<VertexKey> {
    let tokens = token.split('/').collect::<Vec<_>>();

    if tokens.len() != 3 {
        return Err(Error::InvalidFormat);
    }

    let position_index = tokens[0].parse()?;
    let texture_coordinates_index = tokens[1].parse()?;
    let normal_index = tokens[2].parse()?;

    Ok([position_index, texture_coordinates_index, normal_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_position() {
        let position = position(&["-2.4", "1.5", "0.6"]).expect("failed to parse valid position");

        assert_eq!(position[0], -2.4);
        assert_eq!(position[1], 1.5);
        assert_eq!(position[2], 0.6);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_position() {
        position(&["xyz", "-4.0", "0.5"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_few_position_tokens() {
        position(&["8.5", "-1.2"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_position_tokens() {
        position(&["0.05", "-78.0", "3", "5"]).unwrap();
    }
}
