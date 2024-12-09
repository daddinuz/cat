use crate::quote::Quote;
use crate::sequence::Sequence;

pub trait Apply<I: Sequence> {
    type Output: Sequence;

    fn apply(self, input: I) -> Self::Output;
}

impl<I, O, F> Apply<I> for F
where
    I: Sequence,
    O: Sequence,
    F: FnOnce(I) -> O,
{
    type Output = O;

    fn apply(self, input: I) -> Self::Output {
        (self)(input)
    }
}

impl<I: Sequence> Apply<I> for () {
    type Output = I;

    fn apply(self, input: I) -> Self::Output {
        input
    }
}

impl<I, T, H> Apply<I> for (T, H)
where
    I: Sequence,
    T: Sequence + Apply<I>,
    H: Apply<T::Output>,
{
    type Output = H::Output;

    fn apply(self, input: I) -> Self::Output {
        let (tail, head) = self;
        head.apply(tail.apply(input))
    }
}

impl<S: Sequence> Apply<S> for bool {
    type Output = (S, bool);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Sequence> Apply<S> for char {
    type Output = (S, char);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Sequence> Apply<S> for i64 {
    type Output = (S, i64);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Sequence> Apply<S> for f64 {
    type Output = (S, f64);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<U, S: Sequence, const N: usize> Apply<S> for [U; N] {
    type Output = (S, [U; N]);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<U, S: Sequence> Apply<S> for Vec<U> {
    type Output = (S, Vec<U>);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<'a, S: Sequence> Apply<S> for &'a str {
    type Output = (S, &'a str);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Sequence> Apply<S> for String {
    type Output = (S, String);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<U, S: Sequence> Apply<S> for Quote<U> {
    type Output = (S, U);

    fn apply(self, input: S) -> Self::Output {
        (input, self.into_inner())
    }
}
