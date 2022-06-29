use crate::error::Error;

pub mod error;
mod parse;
pub mod result;
mod slice;

pub fn parse(input: impl AsRef<str>) -> result::Result<(Vec<[f32; 8]>, Vec<usize>)> {
    let mut indices = vec![];

    let mut positions = vec![];
    let mut texture_coordinates = vec![];
    let mut normals = vec![];

    let mut component_indices = vec![];

    for line in input.as_ref().to_lowercase().lines() {
        let tokens = line.split_whitespace().collect::<Vec<_>>();

        if tokens.is_empty() {
            return Err(Error::InvalidFormat);
        }

        let key = tokens[0];
        let tokens = &tokens[1..];

        match key {
            "v" => positions.push(parse::position(tokens)?),
            "vt" => texture_coordinates.push(parse::texture_coordinates(tokens)?),
            "vn" => normals.push(parse::normal(tokens)?),
            "f" => parse_face(tokens, &mut component_indices, &mut indices)?,
            _ => {}
        }
    }

    let vertices = component_indices
        .into_iter()
        .map(|component_indices| {
            component_indices_to_vertex(
                component_indices,
                &positions,
                &texture_coordinates,
                &normals,
            )
        })
        .collect::<result::Result<Vec<_>>>()?;

    Ok((vertices, indices))
}

fn component_indices_to_vertex(
    component_indices: [usize; 3],
    positions: &[[f32; 3]],
    texture_coordinates: &[[f32; 2]],
    normals: &[[f32; 3]],
) -> result::Result<[f32; 8]> {
    let position = slice::get_component(positions, component_indices, 0)?;
    let texture_coordinates = slice::get_component(texture_coordinates, component_indices, 1)?;
    let normal = slice::get_component(normals, component_indices, 2)?;

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

fn sub(value: usize, sub_value: usize) -> result::Result<usize> {
    value.checked_sub(sub_value).ok_or(Error::InvalidFormat)
}

fn parse_face(
    tokens: &[&str],
    component_indices: &mut Vec<[usize; 3]>,
    indices: &mut Vec<usize>,
) -> result::Result<()> {
    // TODO: Check number of vertices
    let vertex_index1 = slice::split_vertex_index(tokens, 0)?;
    let vertex_index2 = slice::split_vertex_index(tokens, 1)?;
    let vertex_index3 = slice::split_vertex_index(tokens, 2)?;

    parse_vertex(&vertex_index1, component_indices, indices)?;
    parse_vertex(&vertex_index2, component_indices, indices)?;
    parse_vertex(&vertex_index3, component_indices, indices)?;

    Ok(())
}

fn parse_vertex(
    tokens: &[&str],
    component_indices: &mut Vec<[usize; 3]>,
    indices: &mut Vec<usize>,
) -> result::Result<()> {
    let component_index1 = slice::parse_integer(tokens, 0)?;
    let component_index2 = slice::parse_integer(tokens, 1)?;
    let component_index3 = slice::parse_integer(tokens, 2)?;

    let vertex_index = [component_index1, component_index2, component_index3];

    match component_indices
        .iter()
        .enumerate()
        .find(|(_, x)| **x == vertex_index)
    {
        Some((index, _vertex_index)) => {
            indices.push(index);
        }
        None => {
            component_indices.push(vertex_index);
            let index = component_indices.len() - 1;

            indices.push(index);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
