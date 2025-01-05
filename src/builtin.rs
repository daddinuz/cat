use std::fmt::{Debug, Display};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub};
use std::sync::mpsc;
use std::time::Duration;

use crate::apply::Apply;
use crate::flow;
use crate::misc::{Concat, Contains, IsEmpty, Len};
use crate::stack::Stack;

pub fn apply<S, Q>((s, q): (S, Q)) -> Q::Output
where
    S: Stack,
    Q: Apply<S>,
{
    q.apply(s)
}

pub fn dip<S, I, Q>(((s, i), q): ((S, I), Q)) -> (Q::Output, I)
where
    S: Stack,
    Q: Apply<S>,
{
    (q.apply(s), i)
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

pub fn r#loop<S, Q>((mut s, q): (S, Q)) -> ()
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
        let (s_tmp, c) = qc.clone().apply(s);
        s = s_tmp;

        if !c {
            return s;
        }

        s = qa.clone().apply(s);
    }
}

pub fn concat<S, L, R>(((s, l), r): ((S, L), R)) -> (S, L::Output)
where
    S: Stack,
    L: Concat<R>,
{
    (s, l.concat(r))
}

pub fn extend<S, T, I>(((s, mut t), i): ((S, T), I)) -> (S, T)
where
    S: Stack,
    T: Extend<I::Item>,
    I: IntoIterator,
{
    t.extend(i);
    (s, t)
}

pub fn stack<S>(s: S) -> ((), S)
where
    S: Stack,
{
    ((), s)
}

pub fn unstack<S, Z>((s, z): (S, Z)) -> S::Output
where
    S: Concat<Z>,
    Z: Stack,
{
    s.concat(z)
}

pub fn unstack2<S, Z1, Z2, X, Y>(((s, z1), z2): ((S, Z1), Z2)) -> Y
where
    S: Concat<Z1, Output = X>,
    X: Concat<Z2, Output = Y>,
    Z1: Stack,
    Z2: Stack,
{
    s.concat(z1).concat(z2)
}

pub fn quote<S, U>((s, u): (S, U)) -> (S, ((), U))
where
    S: Stack,
{
    (s, ((), u))
}

pub fn compose<S, Qf, Qg>(((s, qf), qg): ((S, Qf), Qg)) -> (S, (((), Qg), Qf))
where
    S: Stack,
    Qf: Apply<Qg::Output>,
    Qg: Apply<S>,
    (((), Qg), Qf): Apply<S, Output = Qf::Output>,
{
    (s, flow![qg, qf])
}

pub fn contains<S, U, V>(((s, u), v): ((S, U), V)) -> (S, bool)
where
    S: Stack,
    U: Contains<V>,
{
    (s, u.contains(v))
}

pub fn is_empty<S, U>((s, u): (S, U)) -> ((S, U), bool)
where
    S: Stack,
    U: IsEmpty,
{
    let is_empty = u.is_empty();
    ((s, u), is_empty)
}

pub fn len<S, U>((s, u): (S, U)) -> ((S, U), i64)
where
    S: Stack,
    U: Len,
{
    let len = i64::try_from(u.len()).unwrap();
    ((s, u), len)
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
    let ((), c) = qc.clone().apply(flow![i.clone()]);
    if c {
        let ((), i) = ql.apply(flow![i]);
        (s, i)
    } else {
        let (((), i1), i2) = qs.clone().apply(flow![i]);
        let ((), i2) = linrec(flow![i2, qc, ql, qs, qm.clone()]);
        let ((), i) = qm.apply(flow![i1, i2]);
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
    let ((), c) = qc.clone().apply(flow![i.clone()]);
    if c {
        let ((), i) = ql.apply(flow![i]);
        (s, i)
    } else {
        let (((), i1), i2) = qs.clone().apply(flow![i]);
        let ((), i1) = binrec(flow![i1, qc.clone(), ql.clone(), qs.clone(), qm.clone()]);
        let ((), i2) = binrec(flow![i2, qc, ql, qs, qm.clone()]);
        let ((), i) = qm.apply(flow![i1, i2]);
        (s, i)
    }
}

pub fn parbinrec<S, I, Qc, Ql, Qs, Qm>(
    (((((s, i), qc), ql), qs), qm): (((((S, I), Qc), Ql), Qs), Qm),
) -> (S, I)
where
    S: Stack,
    I: 'static + Send + Clone,
    Qc: 'static + Send + Clone + Apply<((), I), Output = ((), bool)>,
    Ql: 'static + Send + Clone + Apply<((), I), Output = ((), I)>,
    Qs: 'static + Send + Clone + Apply<((), I), Output = (((), I), I)>,
    Qm: 'static + Send + Clone + Apply<(((), I), I), Output = ((), I)>,
{
    fn rec<S, I, Qc, Ql, Qs, Qm>(
        (((((s, i), qc), ql), qs), qm): (((((S, I), Qc), Ql), Qs), Qm),
        pool_size: usize,
    ) -> (S, I)
    where
        S: Stack,
        I: 'static + Send + Clone,
        Qc: 'static + Send + Clone + Apply<((), I), Output = ((), bool)>,
        Ql: 'static + Send + Clone + Apply<((), I), Output = ((), I)>,
        Qs: 'static + Send + Clone + Apply<((), I), Output = (((), I), I)>,
        Qm: 'static + Send + Clone + Apply<(((), I), I), Output = ((), I)>,
    {
        let ((), c) = qc.clone().apply(flow![i.clone()]);
        if c {
            let ((), i) = ql.apply(flow![i]);
            (s, i)
        } else {
            let (((), i1), i2) = qs.clone().apply(flow![i]);

            let (i1, i2) = if pool_size > 0 {
                let handle = {
                    let (qc, ql, qs, qm) = (qc.clone(), ql.clone(), qs.clone(), qm.clone());
                    std::thread::spawn(move || rec(flow![i1, qc, ql, qs, qm], pool_size >> 1))
                };

                let ((), i2) = rec(flow![i2, qc, ql, qs, qm.clone()], pool_size >> 1);
                let ((), i1) = handle.join().unwrap();

                (i1, i2)
            } else {
                let ((), i1) = binrec(flow![i1, qc.clone(), ql.clone(), qs.clone(), qm.clone()]);
                let ((), i2) = binrec(flow![i2, qc, ql, qs, qm.clone()]);
                (i1, i2)
            };

            let ((), i) = qm.apply(flow![i1, i2]);
            (s, i)
        }
    }

    let pool_size = std::thread::available_parallelism()
        .map(|n| usize::from(n) >> 2)
        .unwrap_or(0);

    rec((((((s, i), qc), ql), qs), qm), pool_size)
}

pub fn sleep<S>((s, millis): (S, i64)) -> S
where
    S: Stack,
{
    std::thread::sleep(Duration::from_millis(i64::max(millis, 0) as u64));
    s
}

pub fn y<S, U, Qa, Qb, Sa, Sb>(
    (((s, u), qa), qb): (((S, U), Qa), Qb),
) -> ((S, (((), U), Qa)), (((), U), Qb))
where
    U: Clone,
    (((), U), Qa): Apply<Sa>,
    (((), U), Qb): Apply<Sb>,
    Sa: Stack,
    Sb: Stack,
{
    ((s, (((), u.clone()), qa)), (((), u), qb))
}

//           Qa
// A:      *----*
//        /      \
// S: ---*        *--->
//        \      /
// B:      *----*
//           Qb
pub fn fork<S, Qa, Qb>(((s, qa), qb): ((S, Qa), Qb)) -> ((S, Qa::Output), Qb::Output)
where
    S: Stack,
    Qa: 'static + Send + Apply<(), Output: 'static + Send>,
    Qb: 'static + Send + Apply<(), Output: 'static + Send>,
{
    let handle = std::thread::spawn(move || qa.apply(()));
    let zb = qb.apply(());
    let za = handle.join().unwrap();
    ((s, za), zb)
}

//          Qa1 Qa2
// A:      *---+---*
//        /    |    \
// S: ---*    Oa     *--->
//        \    ↓    /
// B:      *---+---*
//          Qb1 Qb2
pub fn send<S, Za, Qa1, Qa2, Oa, Zb, Qb1, Qb2>(
    ((s, (((), qa1), qa2)), (((), qb1), qb2)): ((S, (((), Qa1), Qa2)), (((), Qb1), Qb2)),
) -> ((S, Qa2::Output), Qb2::Output)
where
    S: Stack,
    Za: Stack,
    Qa1: 'static + Send + Apply<(), Output = (Za, Oa)>,
    Qa2: 'static + Send + Apply<Za, Output: 'static + Send>,
    Oa: 'static + Send,
    Zb: Stack,
    Qb1: 'static + Send + Apply<(), Output = Zb>,
    Qb2: 'static + Send + Apply<(Zb, Oa)>,
{
    let (sender, receiver) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let (za, oa) = qa1.apply(());
        sender.send(oa).unwrap();
        qa2.apply(za)
    });

    let zb = qb1.apply(());
    let oa = receiver.recv().unwrap();
    let zb = qb2.apply((zb, oa));

    let za = handle.join().unwrap();
    ((s, za), zb)
}

//          Qa1 Qa2 Qa3
// A:      *---+---+---*
//        /    |   ↑    \
// S: ---*    Oa  Ob     *--->
//        \    ↓   |    /
// B:      *---+---+---*
//          Qb1 Qb2 Qb3
pub fn reply<S, Za1, Za2, Qa1, Qa2, Qa3, Oa, Zb1, Zb2, Qb1, Qb2, Qb3, Ob>(
    ((s, ((((), qa1), qa2), qa3)), ((((), qb1), qb2), qb3)): (
        (S, ((((), Qa1), Qa2), Qa3)),
        ((((), Qb1), Qb2), Qb3),
    ),
) -> ((S, Qa3::Output), Qb3::Output)
where
    S: Stack,
    Za1: Stack,
    Za2: Stack,
    Qa1: 'static + Send + Apply<(), Output = (Za1, Oa)>,
    Qa2: 'static + Send + Apply<Za1, Output = Za2>,
    Qa3: 'static + Send + Apply<(Za2, Ob), Output: 'static + Send>,
    Oa: 'static + Send,
    Zb1: Stack,
    Zb2: Stack,
    Qb1: 'static + Send + Apply<(), Output = Zb1>,
    Qb2: 'static + Send + Apply<(Zb1, Oa), Output = (Zb2, Ob)>,
    Qb3: 'static + Send + Apply<Zb2, Output: 'static + Send>,
    Ob: 'static + Send,
{
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let (za1, oa) = qa1.apply(());
        out_sender.send(oa).unwrap();

        let za2 = qa2.apply(za1);

        let ob = in_receiver.recv().unwrap();
        qa3.apply((za2, ob))
    });

    let zb1 = qb1.apply(());

    let oa = out_receiver.recv().unwrap();
    let (zb2, ob) = qb2.apply((zb1, oa));

    in_sender.send(ob).unwrap();

    let b = qb3.apply(zb2);
    let a = handle.join().unwrap();
    ((s, a), b)
}

//            Qa1 Qa2 Qa3
// A:        *---+---+---*
//          /    |   ↑    \
//         /    Oa  Ob     \
//        /       \ /       \
// S: ---*         *         *--->
//        \       / \       /
//         \    Ob  Oa     /
//          \    |   ↓    /
// B:        *---+---+---*
//            Qb1 Qb2 Qb3
pub fn exchange<S, Za, Qa1, Qa2, Qa3, Oa, Zb, Qb1, Qb2, Qb3, Ob>(
    ((s, ((((), qa1), qa2), qa3)), ((((), qb1), qb2), qb3)): (
        (S, ((((), Qa1), Qa2), Qa3)),
        ((((), Qb1), Qb2), Qb3),
    ),
) -> ((S, Qa3::Output), Qb3::Output)
where
    S: Stack,
    Za: Stack,
    Qa1: 'static + Send + Apply<(), Output = (Za, Oa)>,
    Qa2: 'static + Send + Apply<Za>,
    Qa3: 'static + Send + Apply<(Qa2::Output, Ob), Output: 'static + Send>,
    Oa: 'static + Send,
    Zb: Stack,
    Qb1: 'static + Send + Apply<(), Output = (Zb, Ob)>,
    Qb2: 'static + Send + Apply<Zb>,
    Qb3: 'static + Send + Apply<(Qb2::Output, Oa), Output: 'static + Send>,
    Ob: 'static + Send,
{
    let (sender_ab, receiver_ab) = mpsc::channel();
    let (sender_ba, receiver_ba) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let (za, oa) = qa1.apply(());
        sender_ba.send(oa).unwrap();

        let za = qa2.apply(za);

        let ob = receiver_ab.recv().unwrap();
        qa3.apply((za, ob))
    });

    let (zb, ob) = qb1.apply(());
    sender_ab.send(ob).unwrap();

    let zb = qb2.apply(zb);

    let oa = receiver_ba.recv().unwrap();

    let b = qb3.apply((zb, oa));
    let a = handle.join().unwrap();
    ((s, a), b)
}

//             Qa
// A:        *----+
//          /     |
//         /      Z (if Qa ends before Qb)
//        /  Qs   ↓
// S: ---*--------+--->
//        \       ↑
//         \      Z (if Qb ends before Qa)
//          \     |
// B:        *----+
//             Qb
pub fn race<S, Qa, Qb, Qs, Z>((((s, qa), qb), qs): (((S, Qa), Qb), Qs)) -> (Qs::Output, Z)
where
    S: Stack,
    Qa: 'static + Send + Apply<(), Output = Z>,
    Qb: 'static + Send + Apply<(), Output = Z>,
    Qs: Apply<S>,
    Z: 'static + Send + Stack,
{
    let (sender, receiver) = mpsc::channel();
    let (sender_a, sender_b) = (sender.clone(), sender);

    std::thread::spawn(move || sender_a.send(qa.apply(())));
    std::thread::spawn(move || sender_b.send(qb.apply(())));

    let s = qs.apply(s);
    let z = receiver.recv().unwrap();
    (s, z)
}
