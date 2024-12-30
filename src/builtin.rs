use std::fmt::{Debug, Display};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub};
use std::sync::mpsc;
use std::time::Duration;

use crate::apply::Apply;
use crate::stack::{Cat, Stack};

pub fn i<S, Q>((s, q): (S, Q)) -> Q::Output
where
    S: Stack,
    Q: Apply<S>,
{
    q.apply(s)
}

pub fn k<S, Q>((s, q): (S, Q)) -> (S, Q::Output)
where
    S: Stack,
    Q: Apply<()>,
{
    (s, q.apply(()))
}

pub fn p<S, Q1, Q2>(((s, q1), q2): ((S, Q1), Q2)) -> ((S, Q1::Output), Q2::Output)
where
    S: Stack,
    Q1: 'static + Send + Apply<(), Output: 'static + Send>,
    Q2: 'static + Send + Apply<(), Output: 'static + Send>,
{
    let handle1 = std::thread::spawn(move || q1.apply(()));
    let handle2 = std::thread::spawn(move || q2.apply(()));
    let s1 = handle1.join().unwrap();
    let s2 = handle2.join().unwrap();
    ((s, s1), s2)
}

pub fn dip<S, I, Q>(((s, i), q): ((S, I), Q)) -> (Q::Output, I)
where
    S: Stack,
    Q: Apply<S>,
{
    (q.apply(s), i)
}

/// `$S ($@ -> $@ 'o1) ($@ -> $@ 'o2) => $S 'o1 'o2`
pub fn app0<S, Q1, Q2, O1, O2>(((s, q1), q2): ((S, Q1), Q2)) -> ((S, O1), O2)
where
    S: Stack,
    Q1: Apply<(), Output = ((), O1)>,
    Q2: Apply<(), Output = ((), O2)>,
{
    let ((), o1) = q1.apply(());
    let ((), o2) = q2.apply(());
    ((s, o1), o2)
}

/// `$S 'i ($@ 'i -> $@ 'o1) ($@ 'i -> $@ 'o2) => $S 'o1 'o2`
pub fn app1<S, I, Q1, Q2, O1, O2>((((s, i), q1), q2): (((S, I), Q1), Q2)) -> ((S, O1), O2)
where
    S: Stack,
    I: Clone,
    Q1: Apply<((), I), Output = ((), O1)>,
    Q2: Apply<((), I), Output = ((), O2)>,
{
    let ((), o1) = q1.apply(((), i.clone()));
    let ((), o2) = q2.apply(((), i));
    ((s, o1), o2)
}

pub fn dup<S, I>((s, i): (S, I)) -> ((S, I), I)
where
    S: Stack,
    I: Clone,
{
    ((s, i.clone()), i)
}

pub fn pop<S, I>((s, _): (S, I)) -> S
where
    S: Stack,
{
    s
}

pub fn swap<S, L, R>(((s, l), r): ((S, L), R)) -> ((S, R), L)
where
    S: Stack,
{
    ((s, r), l)
}

pub fn not<S, I>((s, i): (S, I)) -> (S, I::Output)
where
    S: Stack,
    I: Not,
{
    (s, i.not())
}

pub fn and<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: BitAnd<R>,
{
    (s, l & r)
}

pub fn or<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: BitOr<R>,
{
    (s, l | r)
}

pub fn xor<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: BitXor<R>,
{
    (s, l ^ r)
}

pub fn neg<S, I>((s, i): (S, I)) -> (S, I::Output)
where
    S: Stack,
    I: Neg,
{
    (s, i.neg())
}

pub fn add<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: Add<R>,
{
    (s, l + r)
}

pub fn sub<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: Sub<R>,
{
    (s, l - r)
}

pub fn mul<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: Mul<R>,
{
    (s, l * r)
}

pub fn div<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: Div<R>,
{
    (s, l / r)
}

pub fn rem<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: Rem<R>,
{
    (s, l % r)
}

pub fn incr<S>((s, i): (S, i64)) -> (S, i64)
where
    S: Stack,
{
    (s, i + 1)
}

pub fn decr<S>((s, i): (S, i64)) -> (S, i64)
where
    S: Stack,
{
    (s, i - 1)
}

pub fn eq<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Stack,
    I: PartialEq,
{
    (s, l == r)
}

pub fn ne<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Stack,
    I: PartialEq,
{
    (s, l != r)
}

pub fn lt<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Stack,
    I: PartialOrd,
{
    (s, l < r)
}

pub fn le<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Stack,
    I: PartialOrd,
{
    (s, l <= r)
}

pub fn gt<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Stack,
    I: PartialOrd,
{
    (s, l > r)
}

pub fn ge<S, I>(((s, l), r): ((S, I), I)) -> (S, bool)
where
    S: Stack,
    I: PartialOrd,
{
    (s, l >= r)
}

pub fn print<S, I>((s, i): (S, I)) -> S
where
    S: Stack,
    I: Display,
{
    print!("{i}");
    s
}

pub fn display<S, I>((s, i): (S, I)) -> S
where
    S: Stack,
    I: Display,
{
    println!("{i}");
    s
}

pub fn debug<S, I>((s, i): (S, I)) -> S
where
    S: Stack,
    I: Debug,
{
    println!("{i:?}");
    s
}

pub fn r#if<S, Q>(((s, c), q): ((S, bool), Q)) -> S
where
    S: Stack,
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
    S: Stack,
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
    S: Stack,
    Z: Stack,
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
    S: Stack,
{
    if c {
        (s, t)
    } else {
        (s, f)
    }
}

pub fn times<S, Q>(((mut s, q), n): ((S, Q), i64)) -> S
where
    S: Stack,
    Q: Clone + Apply<S, Output = S>,
{
    for _ in 0..n {
        s = q.clone().apply(s);
    }
    s
}

pub fn r#loop<S, Q>((mut s, q): (S, Q)) -> S
where
    S: Stack,
    Q: Clone + Apply<S, Output = S>,
{
    loop {
        s = q.clone().apply(s);
    }
}

pub fn r#while<S, Qc, Qa>(((mut s, qc), qa): ((S, Qc), Qa)) -> S
where
    S: Stack,
    Qc: Clone + Apply<S, Output = (S, bool)>,
    Qa: Clone + Apply<S, Output = S>,
{
    loop {
        let (z, c) = qc.clone().apply(s);
        s = z;

        if !c {
            return s;
        }

        s = qa.clone().apply(s);
    }
}

pub fn cat<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: Cat<R>,
    R: Stack,
{
    (s, l.cat(r))
}

pub fn stack<S>(s: S) -> ((), S)
where
    S: Stack,
{
    ((), s)
}

pub fn unstack<S, Z>((s, z): (S, Z)) -> S::Output
where
    S: Cat<Z>,
    Z: Stack,
{
    s.cat(z)
}

pub fn linrec<S, I, Qc, Ql, Qs, Qm>(
    (((((s, i), qc), ql), qs), qm): (((((S, I), Qc), Ql), Qs), Qm),
) -> (S, I)
where
    S: Stack,
    I: Clone,
    Qc: Clone + Apply<((), I), Output = ((), bool)>,
    Ql: Apply<((), I), Output = ((), I)>,
    Qs: Clone + Apply<((), I), Output = (((), I), I)>,
    Qm: Clone + Apply<(((), I), I), Output = ((), I)>,
{
    let ((), c) = qc.clone().apply(((), i.clone()));
    if c {
        let ((), i) = ql.apply(((), i));
        (s, i)
    } else {
        let (((), i1), i2) = qs.clone().apply(((), i));
        let ((), i2) = linrec(((((((), i2), qc), ql), qs), qm.clone()));
        let ((), i) = qm.apply((((), i1), i2));
        (s, i)
    }
}

pub fn binrec<S, I, Qc, Ql, Qs, Qm>(
    (((((s, i), qc), ql), qs), qm): (((((S, I), Qc), Ql), Qs), Qm),
) -> (S, I)
where
    S: Stack,
    I: Clone,
    Qc: Clone + Apply<((), I), Output = ((), bool)>,
    Ql: Clone + Apply<((), I), Output = ((), I)>,
    Qs: Clone + Apply<((), I), Output = (((), I), I)>,
    Qm: Clone + Apply<(((), I), I), Output = ((), I)>,
{
    let ((), c) = qc.clone().apply(((), i.clone()));
    if c {
        let ((), i) = ql.apply(((), i));
        (s, i)
    } else {
        let (((), i1), i2) = qs.clone().apply(((), i));
        let ((), i1) = binrec((
            (((((), i1), qc.clone()), ql.clone()), qs.clone()),
            qm.clone(),
        ));
        let ((), i2) = binrec(((((((), i2), qc), ql), qs), qm.clone()));
        let ((), i) = qm.apply((((), i1), i2));
        (s, i)
    }
}

pub fn sleep<S>((s, millis): (S, i64)) -> S
where
    S: Stack,
{
    std::thread::sleep(Duration::from_millis(i64::max(millis, 0) as u64));
    s
}

/// `$S ($@ -> $@ 'o1) ($@ -> $@ 'o2) => $S 'o1 'o2`
pub fn parapp0<S, Q1, Q2, O1, O2>(((s, q1), q2): ((S, Q1), Q2)) -> ((S, O1), O2)
where
    S: Stack,
    Q1: 'static + Send + Apply<(), Output = ((), O1)>,
    Q2: 'static + Send + Apply<(), Output = ((), O2)>,
    O1: 'static + Send,
    O2: 'static + Send,
{
    let handle1 = std::thread::spawn(move || q1.apply(()));
    let handle2 = std::thread::spawn(move || q2.apply(()));
    let ((), o1) = handle1.join().unwrap();
    let ((), o2) = handle2.join().unwrap();
    ((s, o1), o2)
}

/// `$S 'i ($@ 'i -> $@ 'o1) ($@ 'i -> $@ 'o2) => $S 'o1 'o2`
pub fn parapp1<S, I, Q1, Q2, O1, O2>((((s, i), q1), q2): (((S, I), Q1), Q2)) -> ((S, O1), O2)
where
    S: Stack,
    I: 'static + Clone + Send,
    Q1: 'static + Send + Apply<((), I), Output = ((), O1)>,
    Q2: 'static + Send + Apply<((), I), Output = ((), O2)>,
    O1: 'static + Send,
    O2: 'static + Send,
{
    let (i1, i2) = (i.clone(), i);
    let handle1 = std::thread::spawn(move || q1.apply(((), i1)));
    let handle2 = std::thread::spawn(move || q2.apply(((), i2)));
    let ((), o1) = handle1.join().unwrap();
    let ((), o2) = handle2.join().unwrap();
    ((s, o1), o2)
}

/// `$S ($@ -> $X) ($S -> $Z) => $Z $X`
pub fn climb0<S, Qx, Qz>(((s, qx), qz): ((S, Qx), Qz)) -> (Qz::Output, Qx::Output)
where
    S: Stack,
    Qx: 'static + Send + Apply<(), Output: 'static + Send>,
    Qz: Apply<S>,
{
    let handle = std::thread::spawn(|| qx.apply(()));
    let z = qz.apply(s);
    let x = handle.join().unwrap();
    (z, x)
}

/// `$S 'i ($@ 'i -> $X) ($S -> $Z) => $Z $X`
pub fn climb1<S, I, Qx, Qz>((((s, i), qx), qz): (((S, I), Qx), Qz)) -> (Qz::Output, Qx::Output)
where
    S: Stack,
    I: 'static + Send,
    Qx: 'static + Send + Apply<((), I), Output: 'static + Send>,
    Qz: Apply<S>,
{
    let handle = std::thread::spawn(|| qx.apply(((), i)));
    let z = qz.apply(s);
    let x = handle.join().unwrap();
    (z, x)
}

/// `$S ($@ -> $Z1) ($@ -> $Z2) => $S $Z1 $Z2`
pub fn fork0<S, Q1, Q2>(((s, q1), q2): ((S, Q1), Q2)) -> ((S, Q1::Output), Q2::Output)
where
    S: Stack,
    Q1: 'static + Send + Apply<(), Output: 'static + Send>,
    Q2: 'static + Send + Apply<(), Output: 'static + Send>,
{
    let handle1 = std::thread::spawn(|| q1.apply(()));
    let handle2 = std::thread::spawn(|| q2.apply(()));
    let z1 = handle1.join().unwrap();
    let z2 = handle2.join().unwrap();
    ((s, z1), z2)
}

/// `$S 'i ($@ 'i -> $Z1) ($@ 'i -> $Z2) => $S $Z1 $Z2`
pub fn fork1<S, I, Q1, Q2>((((s, i), q1), q2): (((S, I), Q1), Q2)) -> ((S, Q1::Output), Q2::Output)
where
    S: Stack,
    I: 'static + Send + Clone,
    Q1: 'static + Send + Apply<((), I), Output: 'static + Send>,
    Q2: 'static + Send + Apply<((), I), Output: 'static + Send>,
{
    let (i1, i2) = (i.clone(), i);
    let handle1 = std::thread::spawn(|| q1.apply(((), i1)));
    let handle2 = std::thread::spawn(|| q2.apply(((), i2)));
    let z1 = handle1.join().unwrap();
    let z2 = handle2.join().unwrap();
    ((s, z1), z2)
}

pub fn send<S, T, H, Z>(((s, (t, h)), z): ((S, (T, H)), Z)) -> ((S, T), <((), H) as Cat<Z>>::Output)
where
    S: Stack,
    T: Stack,
    Z: Stack,
    ((), H): Cat<Z>,
{
    ((s, t), ((), h).cat(z))
}

pub fn receive<S, Z, T, H>(
    ((s, z), (t, h)): ((S, Z), (T, H)),
) -> ((S, <((), H) as Cat<Z>>::Output), T)
where
    S: Stack,
    Z: Stack,
    T: Stack,
    ((), H): Cat<Z>,
{
    ((s, ((), h).cat(z)), t)
}

pub fn reply<S, S1, Q1, Q2, Q3, Qm, O1, Om>(
    ((s, ((((), q1), q2), q3)), qm): ((S, ((((), Q1), Q2), Q3)), Qm),
) -> (S, Q3::Output)
where
    S: Stack,
    S1: 'static + Send + Stack,
    Q1: 'static + Send + Apply<(), Output = (S1, O1)>,
    Q2: 'static + Send + Apply<S1, Output: 'static + Send>,
    Q3: 'static + Send + Apply<(Q2::Output, Om), Output: 'static + Send>,
    Qm: 'static + Send + Apply<((), O1), Output = ((), Om)>,
    O1: 'static + Send,
    Om: 'static + Send,
{
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let (s1, o1) = q1.apply(());
        out_sender.send(o1).unwrap();

        let s2 = q2.apply(s1);

        let om = in_receiver.recv().unwrap();
        q3.apply((s2, om))
    });

    std::thread::spawn(move || {
        let o1 = out_receiver.recv().unwrap();
        let ((), om) = qm.apply(((), o1));
        in_sender.send(om).unwrap();
    })
    .join()
    .unwrap();

    let o3 = handle.join().unwrap();
    (s, o3)
}
