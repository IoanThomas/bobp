use crate::{result, Error};

pub fn get<T>(slice: &[T], index: usize) -> result::Result<&T> {
    slice.get(index).ok_or(Error::InvalidFormat)
}
