use crate::error::Error;
use crate::types::{Normal, Position, TextureCoordinates, Token, Vertex, VertexKey};

pub mod error;
mod parse;
pub mod result;
mod slice;
mod types;

pub fn parse_obj(input: impl AsRef<str>) -> result::Result<(Vec<Vertex>, Vec<usize>)> {
    let mut positions = vec![];
    let mut texture_coordinates = vec![];
    let mut normals = vec![];
    let mut vertex_keys = vec![];

    for line in input.as_ref().to_lowercase().lines() {
        let tokens = line.split_whitespace().collect::<Vec<Token>>();

        if tokens.is_empty() {
            return Err(Error::InvalidFormat);
        }

        let key = tokens[0];
        let tokens = &tokens[1..];

        match key {
            "v" => positions.push(parse::position(tokens)?),
            "vt" => texture_coordinates.push(parse::texture_coordinates(tokens)?),
            "vn" => normals.push(parse::normal(tokens)?),
            "f" => vertex_keys.extend(parse::face(tokens)?),
            _ => {}
        }
    }

    let unique_vertex_keys = unique_vertex_keys(&vertex_keys);

    let indices = get_vertex_key_indices(&vertex_keys, &unique_vertex_keys)?;
    let vertices = create_vertices(
        &unique_vertex_keys,
        &positions,
        &texture_coordinates,
        &normals,
    )?;

    Ok((vertices, indices))
}

fn get_vertex_key_indices(
    vertex_keys: &[VertexKey],
    unique_vertex_keys: &[VertexKey],
) -> result::Result<Vec<usize>> {
    vertex_keys
        .iter()
        .map(|key| find_index(unique_vertex_keys, *key))
        .collect()
}

fn find_index(unique_vertex_keys: &[VertexKey], vertex_key: VertexKey) -> result::Result<usize> {
    unique_vertex_keys
        .iter()
        .position(|key| vertex_key == *key)
        .ok_or(Error::InvalidFormat)
}

fn unique_vertex_keys(vertex_keys: &[VertexKey]) -> Vec<VertexKey> {
    let mut unique_vertex_keys = vec![];

    for vertex_key in vertex_keys.iter() {
        match unique_vertex_keys.iter().find(|key| *key == vertex_key) {
            Some(_) => {}
            None => {
                unique_vertex_keys.push(*vertex_key);
            }
        }
    }

    unique_vertex_keys
}

fn create_vertices(
    vertex_keys: &[VertexKey],
    positions: &[Position],
    texture_coordinates: &[TextureCoordinates],
    normals: &[Normal],
) -> result::Result<Vec<Vertex>> {
    vertex_keys
        .iter()
        .map(|vertex_key| create_vertex(*vertex_key, positions, texture_coordinates, normals))
        .collect()
}

fn create_vertex(
    vertex_key: VertexKey,
    positions: &[Position],
    texture_coordinates: &[TextureCoordinates],
    normals: &[Normal],
) -> result::Result<Vertex> {
    let position = get_attribute(positions, vertex_key[0])?;
    let texture_coordinates = get_attribute(texture_coordinates, vertex_key[1])?;
    let normal = get_attribute(normals, vertex_key[2])?;

    Ok([
        position[0],
        position[1],
        position[2],
        texture_coordinates[0],
        texture_coordinates[1],
        normal[0],
        normal[1],
        normal[2],
    ])
}

fn get_attribute<const N: usize>(
    attributes: &[[f32; N]],
    index: usize,
) -> result::Result<&[f32; N]> {
    let index = index.checked_sub(1).ok_or(Error::InvalidFormat)?;

    slice::get(attributes, index)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
