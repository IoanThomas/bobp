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

    let indices1 = vertex_key(tokens[0])?;
    let indices2 = vertex_key(tokens[1])?;
    let indices3 = vertex_key(tokens[2])?;

    Ok([indices1, indices2, indices3])
}

fn vertex_key(token: Token) -> result::Result<VertexKey> {
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

    #[test]
    fn parse_valid_texture_coordinates() {
        let texture_coordinates = texture_coordinates(&["0.4", "0.6"])
            .expect("failed to parse valid texture coordinates");

        assert_eq!(texture_coordinates[0], 0.4);
        assert_eq!(texture_coordinates[1], 0.6);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_texture_coordinates() {
        texture_coordinates(&["0.0", "invalid"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_few_texture_coordinates_tokens() {
        texture_coordinates(&["1.0"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_texture_coordinates_tokens() {
        texture_coordinates(&["0.05", "0.2", "0.1"]).unwrap();
    }

    #[test]
    fn parse_valid_normal() {
        let normal = normal(&["1.0", "0.0", "0.0"]).expect("failed to parse valid normal");

        assert_eq!(normal[0], 1.0);
        assert_eq!(normal[1], 0.0);
        assert_eq!(normal[2], 0.0);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_normal() {
        normal(&["0.0", "", "1.0"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_few_normal_tokens() {
        normal(&["0.5"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_normal_tokens() {
        normal(&["0.5", "0.5", "0.5", "0.5"]).unwrap();
    }

    #[test]
    fn parse_valid_face() {
        let face = face(&["1/2/3", "4/5/6", "7/8/9"]).expect("failed to parse valid face");

        assert_eq!(face[0], [1, 2, 3]);
        assert_eq!(face[1], [4, 5, 6]);
        assert_eq!(face[2], [7, 8, 9]);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_face() {
        face(&["1/2/3", "abc", "7/8/9"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_few_face_tokens() {
        face(&["2/4/6", "8,10,12"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_face_tokens() {
        face(&["0/1/1", "2/3/5", "8/13/21", "34/55/89"]).unwrap();
    }

    #[test]
    fn parse_valid_vertex_key() {
        let vertex_key = vertex_key("1/2/3").expect("failed to parse vertex key");

        assert_eq!(vertex_key[0], 1);
        assert_eq!(vertex_key[1], 2);
        assert_eq!(vertex_key[2], 3);
    }

    #[test]
    #[should_panic]
    fn parse_invalid_vertex_key() {
        vertex_key("3/6/a").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_few_vertex_key_tokens() {
        vertex_key("4/8").unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_too_many_vertex_key_tokens() {
        vertex_key("1/3/5/7").unwrap();
    }
}
