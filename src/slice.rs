use crate::{result, sub, Error};

pub fn parse_integer(tokens: &[&str], index: usize) -> result::Result<usize> {
    let token = get(tokens, index)?;
    let value = token.parse::<usize>()?;

    Ok(value)
}

pub fn split_vertex_index<'a>(tokens: &[&'a str], index: usize) -> result::Result<Vec<&'a str>> {
    let vertex_index = get(tokens, index)?;
    let components = vertex_index.split('/').collect();

    Ok(components)
}

pub fn get_component<const N: usize>(
    components: &[[f32; N]],
    component_indices: [usize; 3],
    index: usize,
) -> result::Result<&[f32; N]> {
    let component_index = sub(component_indices[index], 1)?;

    get(components, component_index)
}

pub fn get<T>(slice: &[T], index: usize) -> result::Result<&T> {
    slice.get(index).ok_or(Error::InvalidFormat)
}
