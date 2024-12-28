#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal<A>(pub A);

impl<A> Literal<A> {
    pub fn into_inner(self) -> A {
        let Self(inner) = self;
        inner
    }
}

impl<A> AsRef<A> for Literal<A> {
    fn as_ref(&self) -> &A {
        let Self(inner) = self;
        inner
    }
}

impl<A> AsMut<A> for Literal<A> {
    fn as_mut(&mut self) -> &mut A {
        let Self(inner) = self;
        inner
    }
}
