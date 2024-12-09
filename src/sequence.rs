use crate::sailed::Sailed;

pub trait Sequence: Sailed {}

impl Sequence for () {}

impl<T, H> Sequence for (T, H) where T: Sequence {}

pub trait Push<U>: Sequence {
    type Output: Sequence;

    fn push(self, value: U) -> Self::Output;
}

impl<U> Push<U> for () {
    type Output = ((), U);

    fn push(self, value: U) -> Self::Output {
        (self, value)
    }
}

impl<T, H, U> Push<U> for (T, H)
where
    T: Sequence,
{
    type Output = ((T, H), U);

    fn push(self, value: U) -> Self::Output {
        (self, value)
    }
}

pub trait Join<S>: Sequence
where
    S: Sequence,
{
    type Output: Sequence;

    fn join(self, other: S) -> Self::Output;
}

impl<S> Join<()> for S
where
    S: Sequence,
{
    type Output = S;

    fn join(self, _: ()) -> Self::Output {
        self
    }
}

impl<T, H, S> Join<(T, H)> for S
where
    T: Sequence,
    S: Sequence + Join<T>,
    S::Output: Push<H>,
{
    type Output = <S::Output as Push<H>>::Output;

    fn join(self, (tail, head): (T, H)) -> Self::Output {
        self.join(tail).push(head)
    }
}
