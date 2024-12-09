pub trait Sailed {}

impl Sailed for () {}

impl<T, H> Sailed for (T, H) {}
