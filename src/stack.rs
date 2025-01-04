use crate::sailed::Sailed;

pub trait Stack: Sailed {}

impl Stack for () {}

impl<T, H> Stack for (T, H) where T: Stack {}
