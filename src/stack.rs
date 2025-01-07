use crate::apply::Apply;
use crate::sailed::Sailed;
use crate::utils::{Concat, Contains, Debug, Display, IsEmpty, Len};

pub trait Stack: Sailed {}

impl Stack for () {}

impl<T, H> Stack for (T, H) where T: Stack {}

impl<I: Stack> Apply<I> for ()
where
    Self: Stack,
{
    type Output = I;

    fn apply(self, input: I) -> Self::Output {
        input
    }
}

impl<I, T, H> Apply<I> for (T, H)
where
    Self: Stack,
    I: Stack,
    T: Stack + Apply<I>,
    H: Apply<T::Output>,
{
    type Output = H::Output;

    fn apply(self, input: I) -> Self::Output {
        let (tail, head) = self;
        head.apply(tail.apply(input))
    }
}

impl<S> Concat<()> for S
where
    S: Stack,
{
    type Output = S;

    fn concat(self, _: ()) -> Self::Output {
        self
    }
}

impl<T, H, S> Concat<(T, H)> for S
where
    T: Stack,
    S: Stack + Concat<T>,
{
    type Output = (S::Output, H);

    fn concat(self, (tail, head): (T, H)) -> Self::Output {
        (self.concat(tail), head)
    }
}

impl<T> Contains<T> for ()
where
    Self: Stack,
{
    fn contains(&self, _: T) -> bool {
        false
    }
}

impl<T, H> Contains<H> for (T, H)
where
    Self: Stack,
    H: PartialEq,
    T: Contains<H>,
{
    fn contains(&self, value: H) -> bool {
        let (tail, head) = self;
        &value == head || tail.contains(value)
    }
}

impl Len for ()
where
    Self: Stack,
{
    fn len(&self) -> usize {
        0
    }
}

impl<T, H> Len for (T, H)
where
    Self: Stack,
    T: Stack + Len,
{
    fn len(&self) -> usize {
        let (tail, _) = self;
        tail.len() + 1
    }
}

impl IsEmpty for ()
where
    Self: Stack,
{
    fn is_empty(&self) -> bool {
        true
    }
}

impl<T, H> IsEmpty for (T, H)
where
    Self: Stack,
    T: Stack,
{
    fn is_empty(&self) -> bool {
        false
    }
}

impl Display for ()
where
    Self: Stack,
{
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<T, H> Display for (T, H)
where
    Self: Stack,
    T: Stack + IsEmpty + Display,
    H: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (tail, head) = self;

        if tail.is_empty() {
            write!(f, "{head}")
        } else {
            tail.fmt(f)?;
            write!(f, " {head}")
        }
    }
}

impl Debug for ()
where
    Self: Stack,
{
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl<T, H> Debug for (T, H)
where
    Self: Stack,
    T: Stack + IsEmpty + Debug,
    H: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (tail, head) = self;

        if tail.is_empty() {
            write!(f, "{head:?}")
        } else {
            tail.fmt(f)?;
            write!(f, " {head:?}")
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __x_stack_x__ {
    (..$s:expr, $($k:tt)+) => {
        $crate::utils::Concat::concat($s, $crate::stack::stack!($($k)+))
    };
    (..$s:expr $(,)?) => {
        $crate::utils::Concat::concat($s, ())
    };
    ($e:expr, $($k:tt)+) => {
        $crate::utils::Concat::concat(((), $e), $crate::stack::stack!($($k)+))
    };
    ($e:expr $(,)?) => {
        ((), $e)
    };
    () => {
        ()
    };
}

pub use __x_stack_x__ as stack;

#[doc(hidden)]
#[macro_export]
macro_rules! __x_Stack_x__ {
    (..$bottom:ty, $($rest:tt)+) => {
        $crate::stack::Stack![__extend__ $bottom = $($rest)+]
    };
    ($bottom:ty, $($rest:tt)+) => {
        $crate::stack::Stack![__extend__ ((), $bottom) = $($rest)+]
    };
    (..$bottom:ty $(,)?) => {
        $bottom
    };
    ($bottom:ty $(,)?) => {
        ((), $bottom)
    };
    () => {
        ()
    };

    (__extend__ $accumulator:ty = $bottom:ty, $($rest:tt)+) => {
        $crate::stack::Stack![__extend__ ($accumulator, $bottom) = $($rest)+]
    };
    (__extend__ $accumulator:ty = $bottom:ty $(,)?) => {
        ($accumulator, $bottom)
    };
}

pub use __x_Stack_x__ as Stack;

#[doc(hidden)]
#[macro_export]
macro_rules! __x_Unpack_x__ {
    (..$bottom:pat, $($rest:tt)+) => {
        $crate::stack::Unpack![__extend__ $bottom = $($rest)+]
    };
    ($bottom:pat, $($rest:tt)+) => {
        $crate::stack::Unpack![__extend__ ((), $bottom) = $($rest)+]
    };
    (..$bottom:pat $(,)?) => {
        $bottom
    };
    ($bottom:pat $(,)?) => {
        ((), $bottom)
    };
    () => {
        ()
    };

    (__extend__ $accumulator:pat = $bottom:pat, $($rest:tt)+) => {
        $crate::stack::Unpack![__extend__ ($accumulator, $bottom) = $($rest)+]
    };
    (__extend__ $accumulator:pat = $bottom:pat $(,)?) => {
        ($accumulator, $bottom)
    };
}

pub use __x_Unpack_x__ as Unpack;
