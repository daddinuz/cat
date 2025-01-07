use crate::stack::Stack;

pub trait Apply<I: Stack> {
    type Output: Stack;

    fn apply(self, input: I) -> Self::Output;
}

impl<I, O, F> Apply<I> for F
where
    I: Stack,
    O: Stack,
    F: FnOnce(I) -> O,
{
    type Output = O;

    fn apply(self, input: I) -> Self::Output {
        (self)(input)
    }
}

impl<S: Stack> Apply<S> for bool {
    type Output = (S, bool);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Stack> Apply<S> for char {
    type Output = (S, char);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Stack> Apply<S> for f64 {
    type Output = (S, f64);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Stack> Apply<S> for i64 {
    type Output = (S, i64);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Stack, U> Apply<S> for Vec<U> {
    type Output = (S, Vec<U>);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Stack, U, const N: usize> Apply<S> for [U; N] {
    type Output = (S, [U; N]);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<'a, S: Stack, U> Apply<S> for &'a [U] {
    type Output = (S, &'a [U]);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<S: Stack> Apply<S> for String {
    type Output = (S, String);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}

impl<'a, S: Stack> Apply<S> for &'a str {
    type Output = (S, &'a str);

    fn apply(self, input: S) -> Self::Output {
        (input, self)
    }
}
