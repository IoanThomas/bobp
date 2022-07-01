use crate::error::Error;
use crate::types::{Normal, Position, TextureCoordinates, Token, Vertex, VertexKey};

pub mod error;
mod parse;
pub mod result;
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

fn unique_vertex_keys(vertex_keys: &[VertexKey]) -> Vec<VertexKey> {
    let mut uniques = vertex_keys.to_vec();
    uniques.sort_unstable();
    uniques.dedup();

    uniques
}

fn get_vertex_key_indices(
    vertex_keys: &[VertexKey],
    unique_vertex_keys: &[VertexKey],
) -> result::Result<Vec<usize>> {
    vertex_keys
        .iter()
        .map(|vertex_key| unique_vertex_keys.iter().position(|key| vertex_key == key))
        .collect::<Option<_>>()
        .ok_or(Error::InvalidFormat)
}

fn create_vertices(
    vertex_keys: &[VertexKey],
    positions: &[Position],
    texture_coordinates: &[TextureCoordinates],
    normals: &[Normal],
) -> result::Result<Vec<Vertex>> {
    vertex_keys
        .iter()
        .map(|vertex_key| {
            let position = get_attribute(positions, vertex_key[0])?;
            let texture_coordinates = get_attribute(texture_coordinates, vertex_key[1])?;
            let normal = get_attribute(normals, vertex_key[2])?;

            let mut vertex = [0f32; 8];

            vertex[0..3].copy_from_slice(position);
            vertex[3..5].copy_from_slice(texture_coordinates);
            vertex[5..8].copy_from_slice(normal);

            Ok(vertex)
        })
        .collect()
}

fn get_attribute<const N: usize>(
    attributes: &[[f32; N]],
    index: usize,
) -> result::Result<&[f32; N]> {
    index
        .checked_sub(1)
        .and_then(|index| attributes.get(index))
        .ok_or(Error::InvalidFormat)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_vertices_with_valid_keys() {
        let vertex_keys = vec![[1, 2, 3]];
        let positions = vec![[3.0; 3]];
        let texture_coordinates = vec![[0.2; 2], [0.5; 2]];
        let normals = vec![[0.5; 3], [0.2; 3], [0.3; 3]];

        let vertices = create_vertices(&vertex_keys, &positions, &texture_coordinates, &normals)
            .expect("failed to create vertices with valid keys");

        assert_eq!(vertices.len(), 1);
        assert_eq!(vertices[0][0..3], [3.0; 3]);
        assert_eq!(vertices[0][3..5], [0.5; 2]);
        assert_eq!(vertices[0][5..8], [0.3; 3]);
    }

    #[test]
    #[should_panic]
    fn create_vertices_with_invalid_keys() {
        let vertex_keys = vec![[1, 5, 3]];
        let positions = vec![[3.0; 3]];
        let texture_coordinates = vec![[0.2; 2], [0.5; 2]];
        let normals = vec![[0.5; 3], [0.2; 3], [0.3; 3]];

        create_vertices(&vertex_keys, &positions, &texture_coordinates, &normals).unwrap();
    }

    #[test]
    fn get_attribute_with_valid_index() {
        let attribute = get_attribute(&[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]], 1)
            .expect("failed to get attribute with valid index");

        assert_eq!(attribute, &[1.0, 2.0]);
    }

    #[test]
    #[should_panic]
    fn get_attribute_with_invalid_index() {
        get_attribute(&[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]], 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_attribute_with_out_of_range_index() {
        get_attribute(&[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]], 4).unwrap();
    }
}
