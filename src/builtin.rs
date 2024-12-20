use std::fmt::{Debug, Display};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub};
use std::time::Duration;

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

pub fn print<S, I>((s, i): (S, I)) -> S
where
    S: Sequence,
    I: Display,
{
    print!("{i}");
    s
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

pub fn choose<S, T>((((s, c), t), f): (((S, bool), T), T)) -> (S, T)
where
    S: Sequence,
{
    if c {
        (s, t)
    } else {
        (s, f)
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

pub fn sleep<S>((s, millis): (S, i64)) -> S
where
    S: Sequence,
{
    std::thread::sleep(Duration::from_millis(i64::max(millis, 0) as u64));
    s
}

// who joins the thread?((s,  q): (S
pub fn detach<S, Q>((s, q): (S, Q)) -> S
where
    S: Sequence,
    Q: 'static + Send + Apply<(), Output: Send>,
{
    std::thread::spawn(move || q.apply(()));
    s
}

// who joins the thread?
pub fn send<S, I, Q>(((s, i), q): ((S, I), Q)) -> S
where
    S: Sequence,
    I: 'static + Send,
    Q: 'static + Send + Apply<((), I), Output: Send>,
{
    std::thread::spawn(move || q.apply(((), i)));
    s
}

/// `$S ($@ -> 'o) => $S 'o`
pub fn receive<S, O, Q>((s, q): (S, Q)) -> (S, O)
where
    S: Sequence,
    O: 'static + Send,
    Q: 'static + Send + Apply<(), Output = ((), O)>,
{
    let ((), o) = std::thread::spawn(move || q.apply(())).join().unwrap();
    (s, o)
}

/// `$S 'i ($@ 'i -> $@ 'o) => $S 'o`
pub fn prompt<S, I, O, Q>(((s, i), q): ((S, I), Q)) -> (S, O)
where
    S: Sequence,
    I: 'static + Send,
    O: 'static + Send,
    Q: 'static + Send + Apply<((), I), Output = ((), O)>,
{
    let ((), o) = std::thread::spawn(move || q.apply(((), i))).join().unwrap();
    (s, o)
}

/// `$S 'i ($@ 'i -> $@ 'o) ($S -> $Z) => $Z 'o`
pub fn branch<S, Z, I, O, Qp, Ql>((((s, i), qp), ql): (((S, I), Qp), Ql)) -> (Z, O)
where
    S: Sequence,
    Z: Sequence,
    I: 'static + Send,
    O: 'static + Send,
    Qp: 'static + Send + Apply<((), I), Output = ((), O)>,
    Ql: Apply<S, Output = Z>,
{
    let handle = std::thread::spawn(|| qp.apply(((), i)));
    let z = ql.apply(s);
    let ((), o) = handle.join().unwrap();
    (z, o)
}

/// `$S1 ($@ -> $Z 'o1) ($S1 'o1 -> $S2 'i) ($Z 'i -> $@ 'o2) => $S2 'o2`
pub fn reply<S1, S2, Z, O1, I, O2, Qo1, Qx, Qo2>(
    (((s1, qo1), qi), qo2): (((S1, Qo1), Qx), Qo2),
) -> (S2, O2)
where
    S1: Sequence,
    S2: Sequence,
    Z: 'static + Send + Sequence,
    O1: 'static + Send,
    I: 'static + Send,
    O2: 'static + Send,
    Qo1: 'static + Send + Apply<(), Output = (Z, O1)>,
    Qx: Apply<(S1, O1), Output = (S2, I)>,
    Qo2: 'static + Send + Apply<(Z, I), Output = ((), O2)>,
{
    let (z, o1) = std::thread::spawn(move || qo1.apply(())).join().unwrap();
    let (s2, i) = qi.apply((s1, o1));
    let ((), o2) = std::thread::spawn(move || qo2.apply((z, i)))
        .join()
        .unwrap();
    (s2, o2)
}

/// `$S ($@ -> $@ 'o1) ($@ -> $@ 'o2) => $S 'o1 'o2`
pub fn parapp0<S, O1, O2, Q1, Q2>(((s, q1), q2): ((S, Q1), Q2)) -> ((S, O1), O2)
where
    S: Sequence,
    O1: 'static + Send,
    O2: 'static + Send,
    Q1: 'static + Send + Apply<(), Output = ((), O1)>,
    Q2: 'static + Send + Apply<(), Output = ((), O2)>,
{
    let (handle1, handle2) = (
        std::thread::spawn(|| q1.apply(())),
        std::thread::spawn(|| q2.apply(())),
    );
    let ((), o1) = handle1.join().unwrap();
    let ((), o2) = handle2.join().unwrap();
    ((s, o1), o2)
}

/// `$S 'i ($@ 'i -> $@ 'o1) ($@ 'i -> $@ 'o2) => $S 'o1 'o2`
pub fn parapp1<S, I, O1, O2, Q1, Q2>((((s, i), q1), q2): (((S, I), Q1), Q2)) -> ((S, O1), O2)
where
    S: Sequence,
    I: 'static + Clone + Send,
    O1: 'static + Send,
    O2: 'static + Send,
    Q1: 'static + Send + Apply<((), I), Output = ((), O1)>,
    Q2: 'static + Send + Apply<((), I), Output = ((), O2)>,
{
    let j = i.clone();
    let (handle1, handle2) = (
        std::thread::spawn(|| q1.apply(((), j))),
        std::thread::spawn(|| q2.apply(((), i))),
    );
    let ((), o1) = handle1.join().unwrap();
    let ((), o2) = handle2.join().unwrap();
    ((s, o1), o2)
}

pub fn cross<S, Z1, Z2, X1, X2, O1, O2, M1, M2, Q1, Q2>(
    ((((s, m1), m2), q1), q2): ((((S, M1), M2), Q1), Q2),
) -> ((S, O1), O2)
where
    S: Sequence,
    Z1: 'static + Send + Sequence,
    Z2: 'static + Send + Sequence,
    X1: 'static + Send,
    X2: 'static + Send,
    O1: 'static + Send,
    O2: 'static + Send,
    M1: 'static + Send + Apply<(), Output = (Z1, X1)>,
    M2: 'static + Send + Apply<(), Output = (Z2, X2)>,
    Q1: 'static + Send + Apply<(Z1, X1), Output = ((), O1)>,
    Q2: 'static + Send + Apply<(Z2, X2), Output = ((), O2)>,
{
    let (handle1, handle2) = (
        std::thread::spawn(|| m1.apply(())),
        std::thread::spawn(|| m2.apply(())),
    );

    let (z1, x1) = handle1.join().unwrap();
    let (z2, x2) = handle2.join().unwrap();

    let (handle1, handle2) = (
        std::thread::spawn(|| q1.apply((z1, x1))),
        std::thread::spawn(|| q2.apply((z2, x2))),
    );

    let ((), o1) = handle1.join().unwrap();
    let ((), o2) = handle2.join().unwrap();

    ((s, o1), o2)
}
