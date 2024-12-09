use std::fmt::Display;

use crate::sequence::Sequence;

pub trait Show {
    fn show(&self);
}

impl Show for () {
    fn show(&self) {}
}

impl<T, H> Show for (T, H)
where
    T: Sequence + Show,
    H: Display,
{
    fn show(&self) {
        let (tail, head) = self;
        tail.show();
        print!(" {head}");
    }
}
