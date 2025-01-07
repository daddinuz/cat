use crate::apply::Apply;
use crate::stack::Stack;
use crate::utils::{Concat, Debug, Display, IsEmpty, Len};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Quote<S: Stack>(pub S);

impl<S: Stack> From<S> for Quote<S> {
    fn from(stack: S) -> Self {
        Self(stack)
    }
}

impl Default for Quote<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl Quote<()> {
    pub const fn new() -> Self {
        Self(())
    }
}

impl<S: Stack> Quote<S> {
    pub fn into_inner(self) -> S {
        let Quote(stack) = self;
        stack
    }
}

impl<I, S> Apply<I> for Quote<S>
where
    I: Stack,
    S: Stack,
{
    type Output = (I, Self);

    fn apply(self, input: I) -> Self::Output {
        (input, self)
    }
}

impl<L, R> Concat<Quote<R>> for Quote<L>
where
    L: Stack + Concat<R, Output: Stack>,
    R: Stack,
{
    type Output = Quote<L::Output>;

    fn concat(self, Quote(right): Quote<R>) -> Self::Output {
        let Quote(left) = self;
        Quote(left.concat(right))
    }
}

impl<S> Len for Quote<S>
where
    S: Stack + Len,
{
    fn len(&self) -> usize {
        let Quote(stack) = self;
        stack.len()
    }
}

impl<S> IsEmpty for Quote<S>
where
    S: Stack + IsEmpty,
{
    fn is_empty(&self) -> bool {
        let Quote(stack) = self;
        stack.is_empty()
    }
}

impl<S> std::fmt::Display for Quote<S>
where
    S: Stack + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Quote(stack) = self;
        write!(f, "[{}]", stack.display())
    }
}

impl<S> std::fmt::Debug for Quote<S>
where
    S: Stack + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Quote(stack) = self;
        write!(f, "[{:?}]", stack.debug())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __x_quote_x__ {
    ($($k:tt)*) => {
        $crate::quote::Quote($crate::stack::stack![$($k)*])
    };
}

pub use __x_quote_x__ as quote;

#[doc(hidden)]
#[macro_export]
macro_rules! __x_Quote_x__ {
    ($($k:tt)*) => {
        $crate::quote::Quote::<$crate::stack::Stack![$($k)*]>
    };
}

pub use __x_Quote_x__ as Quote;
