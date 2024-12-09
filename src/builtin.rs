use std::fmt::{Debug, Display};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub};

use crate::apply::Apply;
use crate::sequence::{Join, Sequence};

pub fn i<S, Q>((s, q): (S, Q)) -> Q::Output
where
    S: Sequence,
    Q: Apply<S>,
{
    q.apply(s)
}

pub fn dip<S, P, Q>(((s, p), q): ((S, P), Q)) -> (Q::Output, P)
where
    S: Sequence,
    Q: Apply<S>,
{
    (q.apply(s), p)
}

pub fn dup<S, I>((s, i): (S, I)) -> ((S, I), I)
where
    S: Sequence,
    I: Clone,
{
    ((s, i.clone()), i)
}

pub fn pop<S, I>((s, _): (S, I)) -> S
where
    S: Sequence,
{
    s
}

pub fn swap<S, L, R>(((s, l), r): ((S, L), R)) -> ((S, R), L)
where
    S: Sequence,
{
    ((s, r), l)
}

pub fn not<S, I>((s, i): (S, I)) -> (S, I::Output)
where
    S: Sequence,
    I: Not,
{
    (s, i.not())
}

pub fn and<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: BitAnd<R>,
{
    (s, l & r)
}

pub fn or<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: BitOr<R>,
{
    (s, l | r)
}

pub fn xor<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: BitXor<R>,
{
    (s, l ^ r)
}

pub fn neg<S, I>((s, i): (S, I)) -> (S, I::Output)
where
    S: Sequence,
    I: Neg,
{
    (s, i.neg())
}

pub fn add<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: Add<R>,
{
    (s, l + r)
}

pub fn sub<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: Sub<R>,
{
    (s, l - r)
}

pub fn mul<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: Mul<R>,
{
    (s, l * r)
}

pub fn div<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: Div<R>,
{
    (s, l / r)
}

pub fn rem<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Sequence,
    L: Rem<R>,
{
    (s, l % r)
}

pub fn incr<S>((s, i): (S, i64)) -> (S, i64)
where
    S: Sequence,
{
    (s, i + 1)
}

pub fn decr<S>((s, i): (S, i64)) -> (S, i64)
where
    S: Sequence,
{
    (s, i - 1)
}

pub fn eq<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Sequence,
    I: Ord,
{
    (s, l == r)
}

pub fn ne<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Sequence,
    I: Ord,
{
    (s, l != r)
}

pub fn lt<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Sequence,
    I: Ord,
{
    (s, l < r)
}

pub fn le<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Sequence,
    I: Ord,
{
    (s, l <= r)
}

pub fn gt<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Sequence,
    I: Ord,
{
    (s, l > r)
}

pub fn ge<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Sequence,
    I: Ord,
{
    (s, l >= r)
}

pub fn display<S, I>((s, i): (S, I)) -> S
where
    S: Sequence,
    I: Display,
{
    println!("{i}");
    s
}

pub fn debug<S, I>((s, i): (S, I)) -> S
where
    S: Sequence,
    I: Debug,
{
    println!("{i:?}");
    s
}

pub fn r#if<S, Q>(((s, c), q): ((S, bool), Q)) -> S
where
    S: Sequence,
    Q: Apply<S, Output = S>,
{
    if c {
        q.apply(s)
    } else {
        s
    }
}

pub fn r#else<S, Q>(((s, c), q): ((S, bool), Q)) -> S
where
    S: Sequence,
    Q: Apply<S, Output = S>,
{
    if c {
        s
    } else {
        q.apply(s)
    }
}

pub fn if_else<S, Z, Qt, Qf>((((s, c), qt), qf): (((S, bool), Qt), Qf)) -> Z
where
    S: Sequence,
    Z: Sequence,
    Qt: Apply<S, Output = Z>,
    Qf: Apply<S, Output = Z>,
{
    if c {
        qt.apply(s)
    } else {
        qf.apply(s)
    }
}

pub fn r#loop<S, Q>((mut s, q): (S, Q))
where
    S: Sequence,
    Q: Clone + Apply<S, Output = S>,
{
    loop {
        s = q.clone().apply(s);
    }
}

pub fn r#while<S, Qc, Qa>(((mut s, qc), qa): ((S, Qc), Qa)) -> S
where
    S: Sequence,
    Qc: Clone + Apply<S, Output = (S, bool)>,
    Qa: Clone + Apply<S, Output = S>,
{
    loop {
        let (ss, c) = qc.clone().apply(s);
        s = ss;

        if !c {
            return s;
        }

        s = qa.clone().apply(s);
    }
}

pub fn stack<S: Sequence>(s: S) -> ((), S) {
    ((), s)
}

pub fn unstack<S, T>((s, t): (S, T)) -> S::Output
where
    S: Join<T>,
    T: Sequence,
{
    s.join(t)
}

pub fn linrec<S, T, Qc, Ql, Qs, Qm>(
    (((((s, t), qc), ql), qs), qm): (((((S, T), Qc), Ql), Qs), Qm),
) -> (S, T)
where
    S: Sequence,
    Qc: Clone + Apply<((), T), Output = (((), T), bool)>,
    Ql: Apply<((), T), Output = ((), T)>,
    Qs: Clone + Apply<((), T), Output = (((), T), T)>,
    Qm: Clone + Apply<(((), T), T), Output = ((), T)>,
{
    let (((), t), c) = qc.clone().apply(((), t));
    if c {
        let ((), t) = ql.apply(((), t));
        (s, t)
    } else {
        let (((), t1), t2) = qs.clone().apply(((), t));
        let ((), t2) = linrec(((((((), t2), qc), ql), qs), qm.clone()));
        let ((), t) = qm.apply((((), t1), t2));
        (s, t)
    }
}

pub fn binrec<S, T, Qc, Ql, Qs, Qm>(
    (((((s, t), qc), ql), qs), qm): (((((S, T), Qc), Ql), Qs), Qm),
) -> (S, T)
where
    S: Sequence,
    Qc: Clone + Apply<((), T), Output = (((), T), bool)>,
    Ql: Clone + Apply<((), T), Output = ((), T)>,
    Qs: Clone + Apply<((), T), Output = (((), T), T)>,
    Qm: Clone + Apply<(((), T), T), Output = ((), T)>,
{
    let (((), t), c) = qc.clone().apply(((), t));
    if c {
        let ((), t) = ql.apply(((), t));
        (s, t)
    } else {
        let (((), t1), t2) = qs.clone().apply(((), t));
        let ((), t1) = binrec((
            (((((), t1), qc.clone()), ql.clone()), qs.clone()),
            qm.clone(),
        ));
        let ((), t2) = binrec(((((((), t2), qc), ql), qs), qm.clone()));
        let ((), t) = qm.apply((((), t1), t2));
        (s, t)
    }
}
