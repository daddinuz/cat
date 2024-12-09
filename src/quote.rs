use std::fmt::{Debug, Formatter, Result as FmtResult};

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

impl<A> Debug for Quote<A>
where
    A: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("Quote").field(self.as_ref()).finish()
    }
}

impl<A> Copy for Quote<A> where A: Copy {}

impl<A> Clone for Quote<A>
where
    A: Clone,
{
    fn clone(&self) -> Self {
        Quote(self.as_ref().clone())
    }
}
