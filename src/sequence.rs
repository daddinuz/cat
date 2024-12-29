use crate::sailed::Sailed;

pub trait Sequence: Sailed {}

impl Sequence for () {}

impl<T, H> Sequence for (T, H) where T: Sequence {}

pub trait Cat<S>: Sequence
where
    S: Sequence,
{
    type Output: Sequence;

    fn cat(self, other: S) -> Self::Output;
}

impl<S> Cat<()> for S
where
    S: Sequence,
{
    type Output = S;

    fn cat(self, _: ()) -> Self::Output {
        self
    }
}

impl<T, H, S> Cat<(T, H)> for S
where
    T: Sequence,
    S: Sequence + Cat<T>,
{
    type Output = (S::Output, H);

    fn cat(self, (tail, head): (T, H)) -> Self::Output {
        (self.cat(tail), head)
    }
}
