#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quote<A>(pub A);

impl<A> Quote<A> {
    pub fn into_inner(self) -> A {
        let Self(inner) = self;
        inner
    }
}

impl<A> AsRef<A> for Quote<A> {
    fn as_ref(&self) -> &A {
        let Self(inner) = self;
        inner
    }
}

impl<A> AsMut<A> for Quote<A> {
    fn as_mut(&mut self) -> &mut A {
        let Self(inner) = self;
        inner
    }
}
