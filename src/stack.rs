use crate::sailed::Sailed;

pub trait Stack: Sailed {}

impl Stack for () {}

impl<T, H> Stack for (T, H) where T: Stack {}

pub trait Cat<S>: Stack
where
    S: Stack,
{
    type Output: Stack;

    fn cat(self, other: S) -> Self::Output;
}

impl<S> Cat<()> for S
where
    S: Stack,
{
    type Output = S;

    fn cat(self, _: ()) -> Self::Output {
        self
    }
}

impl<T, H, S> Cat<(T, H)> for S
where
    T: Stack,
    S: Stack + Cat<T>,
{
    type Output = (S::Output, H);

    fn cat(self, (tail, head): (T, H)) -> Self::Output {
        (self.cat(tail), head)
    }
}
