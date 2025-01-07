use std::fmt::{Debug, Display};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub};
use std::sync::mpsc;
use std::time::Duration;

use crate::apply::Apply;
use crate::quote::{quote, Quote};
use crate::stack::{stack, Stack, Unpack};
use crate::utils::{Concat, Contains, IsEmpty, Len};

pub fn i<S, A>(stack: Stack![..S, Quote![..A]]) -> Stack![..A::Output]
where
    S: Stack,
    A: Stack + Apply<S>,
{
    let Unpack![..s, Quote(a)] = stack;
    a.apply(s)
}

pub fn dip<S, I, A>(stack: Stack![..S, I, Quote![..A]]) -> Stack![..A::Output, I]
where
    S: Stack,
    A: Stack + Apply<S>,
{
    let Unpack![..s, i, Quote(a)] = stack;
    stack![..a.apply(s), i]
}

pub fn apply<S, A>(stack: Stack![..S, A]) -> Stack![..A::Output]
where
    S: Stack,
    A: Apply<S>,
{
    let Unpack![..s, a] = stack;
    a.apply(s)
}

pub fn quote<S, U>(stack: Stack![..S, U]) -> Stack![..S, Quote![U]]
where
    S: Stack,
{
    let Unpack![..s, u] = stack;
    stack![..s, quote![u]]
}

pub fn unquote<S, U>(stack: Stack![..S, Quote![..U]]) -> Stack![..S::Output]
where
    S: Stack + Concat<U, Output: Stack>,
    U: Stack,
{
    let Unpack![..s, Quote(u)] = stack;
    s.concat(u)
}

pub fn unquote2<Si, S1, S2, So>(stack: Stack![..Si, Quote![..S1], Quote![..S2]]) -> Stack![..So]
where
    Si: Stack + Concat<S1, Output: Stack + Concat<S2, Output = So>>,
    S1: Stack,
    S2: Stack,
    So: Stack,
{
    let Unpack![..s, Quote(z1), Quote(z2)] = stack;
    s.concat(z1).concat(z2)
}

pub fn dup<S, I>(stack: Stack![..S, I]) -> Stack![..S, I, I]
where
    S: Stack,
    I: Clone,
{
    let Unpack![..s, i] = stack;
    stack![..s, i.clone(), i]
}

pub fn pop<S, I>(stack: Stack![..S, I]) -> Stack![..S]
where
    S: Stack,
{
    let Unpack![..s, _] = stack;
    s
}

pub fn swap<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, R, L]
where
    S: Stack,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, r, l]
}

pub fn not<S, I>(stack: Stack![..S, I]) -> Stack![..S, I::Output]
where
    S: Stack,
    I: Not,
{
    let Unpack![..s, i] = stack;
    stack![..s, !i]
}

pub fn and<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: BitAnd<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l & r]
}

pub fn or<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: BitOr<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l | r]
}

pub fn xor<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: BitXor<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l ^ r]
}

pub fn neg<S, I>(stack: Stack![..S, I]) -> Stack![..S, I::Output]
where
    S: Stack,
    I: Neg,
{
    let Unpack![..s, i] = stack;
    stack![..s, -i]
}

pub fn add<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: Add<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l + r]
}

pub fn sub<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: Sub<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l - r]
}

pub fn mul<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: Mul<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l * r]
}

pub fn div<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: Div<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l / r]
}

pub fn rem<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: Rem<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l % r]
}

pub fn incr<S>(stack: Stack![..S, i64]) -> Stack![..S, i64]
where
    S: Stack,
{
    let Unpack![..s, n] = stack;
    stack![..s, n + 1]
}

pub fn decr<S>(stack: Stack![..S, i64]) -> Stack![..S, i64]
where
    S: Stack,
{
    let Unpack![..s, n] = stack;
    stack![..s, n - 1]
}

pub fn eq<S, I>(stack: Stack![..S, I, I]) -> Stack![..S, bool]
where
    S: Stack,
    I: PartialEq,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l == r]
}

pub fn ne<S, I>(stack: Stack![..S, I, I]) -> Stack![..S, bool]
where
    S: Stack,
    I: PartialEq,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l != r]
}

pub fn lt<S, I>(stack: Stack![..S, I, I]) -> Stack![..S, bool]
where
    S: Stack,
    I: PartialOrd,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l < r]
}

pub fn le<S, I>(stack: Stack![..S, I, I]) -> Stack![..S, bool]
where
    S: Stack,
    I: PartialOrd,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l <= r]
}

pub fn gt<S, I>(stack: Stack![..S, I, I]) -> Stack![..S, bool]
where
    S: Stack,
    I: PartialOrd,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l > r]
}

pub fn ge<S, I>(stack: Stack![..S, I, I]) -> Stack![..S, bool]
where
    S: Stack,
    I: PartialOrd,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l >= r]
}

pub fn print<S, I>(stack: Stack![..S, I]) -> Stack![..S]
where
    S: Stack,
    I: Display,
{
    let Unpack![..s, i] = stack;
    print!("{i}");
    s
}

pub fn display<S, I>(stack: Stack![..S, I]) -> Stack![..S]
where
    S: Stack,
    I: Display,
{
    let Unpack![..s, i] = stack;
    println!("{i}");
    s
}

pub fn debug<S, I>(stack: Stack![..S, I]) -> Stack![..S]
where
    S: Stack,
    I: Debug,
{
    let Unpack![..s, i] = stack;
    println!("{i:?}");
    s
}

pub fn r#if<S, A>(stack: Stack![..S, bool, Quote![..A]]) -> Stack![..S]
where
    S: Stack,
    A: Stack + Apply<S, Output = S>,
{
    let Unpack![..s, c, Quote(a)] = stack;
    if c {
        a.apply(s)
    } else {
        s
    }
}

pub fn r#else<S, A>(stack: Stack![..S, bool, Quote![..A]]) -> Stack![..S]
where
    S: Stack,
    A: Stack + Apply<S, Output = S>,
{
    let Unpack![..s, c, Quote(a)] = stack;
    if c {
        s
    } else {
        a.apply(s)
    }
}

pub fn if_else<S, T, F, Z>(stack: Stack![..S, bool, Quote![..T], Quote![..F]]) -> Stack![..Z]
where
    S: Stack,
    T: Stack + Apply<S, Output = Z>,
    F: Stack + Apply<S, Output = Z>,
{
    let Unpack![..s, c, Quote(t), Quote(f)] = stack;
    if c {
        t.apply(s)
    } else {
        f.apply(s)
    }
}

pub fn times<S, A>(stack: Stack![..S, Quote![..A], i64]) -> Stack![..S]
where
    S: Stack,
    A: Stack + Clone + Apply<S, Output = S>,
{
    let Unpack![..mut s, Quote(a), n] = stack;
    for _ in 0..n {
        s = a.clone().apply(s);
    }
    s
}

pub fn r#loop<S, A>(stack: Stack![..S, Quote![..A]]) -> Stack![]
where
    S: Stack,
    A: Stack + Clone + Apply<S, Output = S>,
{
    let Unpack![..mut s, Quote(a)] = stack;
    loop {
        s = a.clone().apply(s);
    }
}

pub fn r#while<S, C, A>(stack: Stack![..S, Quote![..C], Quote![..A]]) -> Stack![..S]
where
    S: Stack,
    C: Stack + Clone + Apply<S, Output = Stack![..S, bool]>,
    A: Stack + Clone + Apply<S, Output = S>,
{
    let Unpack![..mut s, Quote(qc), Quote(qa)] = stack;
    loop {
        let Unpack![..s_tmp, c] = qc.clone().apply(s);
        s = s_tmp;

        if !c {
            return s;
        }

        s = qa.clone().apply(s);
    }
}

pub fn concat<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, L::Output]
where
    S: Stack,
    L: Concat<R>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, l.concat(r)]
}

pub fn extend<S, T, I>(stack: Stack![..S, T, I]) -> Stack![..S, T]
where
    S: Stack,
    T: Extend<I::Item>,
    I: IntoIterator,
{
    let Unpack![..s, mut t, i] = stack;
    t.extend(i);
    stack![..s, t]
}

pub fn stack<S>(stack: Stack![..S]) -> Stack![Quote![..S]]
where
    S: Stack,
{
    stack![quote![..stack]]
}

pub fn compose<S, L, R>(stack: Stack![..S, L, R]) -> Stack![..S, R::Output]
where
    S: Stack,
    R: Concat<L>,
{
    let Unpack![..s, l, r] = stack;
    stack![..s, r.concat(l)]
}

pub fn contains<S, U, V>(stack: Stack![..S, U, V]) -> Stack![..S, bool]
where
    S: Stack,
    U: Contains<V>,
{
    let Unpack![..s, u, v] = stack;
    stack![..s, u.contains(v)]
}

pub fn is_empty<S, U>(stack: Stack![..S, U]) -> Stack![..S, bool]
where
    S: Stack,
    U: IsEmpty,
{
    let Unpack![..s, u] = stack;
    stack![..s, u.is_empty()]
}

pub fn len<S, U>(stack: Stack![..S, U]) -> Stack![..S, i64]
where
    S: Stack,
    U: Len,
{
    let Unpack![..s, u] = stack;
    let len = i64::try_from(u.len()).unwrap();
    stack![..s, len]
}

pub fn linrec<S, I, Qc, Ql, Qs, Qm>(
    stack: Stack![
        ..S,
        I,
        Quote![..Qc],
        Quote![..Ql],
        Quote![..Qs],
        Quote![..Qm]
    ],
) -> Stack![..S, I]
where
    S: Stack,
    I: Clone,
    Qc: Stack + Clone + Apply<Stack![I], Output = Stack![bool]>,
    Ql: Stack + Apply<Stack![I], Output = Stack![I]>,
    Qs: Stack + Clone + Apply<Stack![I], Output = Stack![I, I]>,
    Qm: Stack + Clone + Apply<Stack![I, I], Output = Stack![I]>,
{
    let Unpack![..s, i, Quote(qc), Quote(ql), Quote(qs), Quote(qm)] = stack;
    let Unpack![c] = qc.clone().apply(stack![i.clone()]);
    if c {
        let Unpack![i] = ql.apply(stack![i]);
        stack![..s, i]
    } else {
        let Unpack![i1, i2] = qs.clone().apply(stack![i]);
        let Unpack![i2] = linrec(stack![
            i2,
            quote![..qc],
            quote![..ql],
            quote![..qs],
            quote![..qm.clone()]
        ]);
        let Unpack![i] = qm.apply(stack![i1, i2]);
        stack![..s, i]
    }
}

pub fn binrec<S, I, Qc, Ql, Qs, Qm>(
    stack: Stack![
        ..S,
        I,
        Quote![..Qc],
        Quote![..Ql],
        Quote![..Qs],
        Quote![..Qm]
    ],
) -> Stack![..S, I]
where
    S: Stack,
    I: Clone,
    Qc: Stack + Clone + Apply<Stack![I], Output = Stack![bool]>,
    Ql: Stack + Clone + Apply<Stack![I], Output = Stack![I]>,
    Qs: Stack + Clone + Apply<Stack![I], Output = Stack![I, I]>,
    Qm: Stack + Clone + Apply<Stack![I, I], Output = Stack![I]>,
{
    let Unpack![..s, i, Quote(qc), Quote(ql), Quote(qs), Quote(qm)] = stack;
    let Unpack![c] = qc.clone().apply(stack![i.clone()]);
    if c {
        let Unpack![i] = ql.apply(stack![i]);
        stack![..s, i]
    } else {
        let Unpack![i1, i2] = qs.clone().apply(stack![i]);
        let Unpack![i1] = binrec(stack![
            i1,
            quote![..qc.clone()],
            quote![..ql.clone()],
            quote![..qs.clone()],
            quote![..qm.clone()]
        ]);
        let Unpack![i2] = binrec(stack![
            i2,
            quote![..qc],
            quote![..ql],
            quote![..qs],
            quote![..qm.clone()]
        ]);
        let Unpack![i] = qm.apply(stack![i1, i2]);
        stack![..s, i]
    }
}

pub fn parbinrec<S, I, Qc, Ql, Qs, Qm>(
    stack: Stack![
        ..S,
        I,
        Quote![..Qc],
        Quote![..Ql],
        Quote![..Qs],
        Quote![..Qm]
    ],
) -> Stack![..S, I]
where
    S: Stack,
    I: 'static + Send + Clone,
    Qc: 'static + Send + Stack + Clone + Apply<Stack![I], Output = Stack![bool]>,
    Ql: 'static + Send + Stack + Clone + Apply<Stack![I], Output = Stack![I]>,
    Qs: 'static + Send + Stack + Clone + Apply<Stack![I], Output = Stack![I, I]>,
    Qm: 'static + Send + Stack + Clone + Apply<Stack![I, I], Output = Stack![I]>,
{
    pub fn rec<S, I, Qc, Ql, Qs, Qm>(
        stack: Stack![
            ..S,
            I,
            Quote![..Qc],
            Quote![..Ql],
            Quote![..Qs],
            Quote![..Qm]
        ],
        mut pool_size: usize,
    ) -> Stack![..S, I]
    where
        S: Stack,
        I: 'static + Send + Clone,
        Qc: 'static + Send + Stack + Clone + Apply<Stack![I], Output = Stack![bool]>,
        Ql: 'static + Send + Stack + Clone + Apply<Stack![I], Output = Stack![I]>,
        Qs: 'static + Send + Stack + Clone + Apply<Stack![I], Output = Stack![I, I]>,
        Qm: 'static + Send + Stack + Clone + Apply<Stack![I, I], Output = Stack![I]>,
    {
        let Unpack![..s, i, Quote(qc), Quote(ql), Quote(qs), Quote(qm)] = stack;
        let Unpack![c] = qc.clone().apply(stack![i.clone()]);

        if c {
            let Unpack![i] = ql.apply(stack![i]);
            stack![..s, i]
        } else {
            let Unpack![i1, i2] = qs.clone().apply(stack![i]);

            let (i1, i2) = if pool_size > 0 {
                pool_size >>= 1;

                let handle = {
                    let (qc, ql, qs, qm) = (qc.clone(), ql.clone(), qs.clone(), qm.clone());
                    std::thread::spawn(move || {
                        rec(
                            stack![
                                i1,
                                quote![..qc.clone()],
                                quote![..ql.clone()],
                                quote![..qs.clone()],
                                quote![..qm.clone()]
                            ],
                            pool_size,
                        )
                    })
                };

                let Unpack![i2] = rec(
                    stack![
                        i2,
                        quote![..qc],
                        quote![..ql],
                        quote![..qs],
                        quote![..qm.clone()]
                    ],
                    pool_size,
                );
                let Unpack![i1] = handle.join().unwrap();
                (i1, i2)
            } else {
                let Unpack![i1] = binrec(stack![
                    i1,
                    quote![..qc.clone()],
                    quote![..ql.clone()],
                    quote![..qs.clone()],
                    quote![..qm.clone()]
                ]);
                let Unpack![i2] = binrec(stack![
                    i2,
                    quote![..qc],
                    quote![..ql],
                    quote![..qs],
                    quote![..qm.clone()]
                ]);
                (i1, i2)
            };

            let Unpack![i] = qm.apply(stack![i1, i2]);
            stack![..s, i]
        }
    }

    let pool_size = std::thread::available_parallelism()
        .map(|n| usize::from(n) >> 2)
        .unwrap_or(0);

    let Unpack![..s, i, qc, ql, qs, qm] = stack;
    rec(stack![..s, i, qc, ql, qs, qm], pool_size)
}

pub fn sleep<S>(stack: Stack![..S, i64]) -> Stack![..S]
where
    S: Stack,
{
    let Unpack![..s, millis] = stack;
    std::thread::sleep(Duration::from_millis(i64::max(millis, 0) as u64));
    s
}

pub fn y<S, U, Qa, Qb>(
    stack: Stack![..S, U, Quote![..Qa], Quote![..Qb]],
) -> Stack![
    ..S,
    Quote![..<Stack![U] as Concat<Qa>>::Output],
    Quote![..<Stack![U] as Concat<Qb>>::Output]
]
where
    S: Stack,
    U: Clone,
    Stack![U]: Concat<Qa, Output: Stack> + Concat<Qb, Output: Stack>,
    Qa: Stack,
    Qb: Stack,
{
    let Unpack![..s, u, Quote(qa), Quote(qb)] = stack;
    stack![..s, quote![u.clone(), ..qa], quote![u, ..qb]]
}

//           Qa
// A:      *----*
//        /      \
// X: ---*        *--->
//        \      /
// B:      *----*
//           Qb
pub fn fork<S, Qa, Qb>(
    stack: Stack![..S, Quote![..Qa], Quote![..Qb]],
) -> Stack![..S, Quote![..Qa::Output], Quote![..Qb::Output]]
where
    S: Stack,
    Qa: 'static + Send + Stack + Apply<Stack![], Output: 'static + Send>,
    Qb: Stack + Apply<Stack![]>,
{
    let Unpack![..s, Quote(qa), Quote(qb)] = stack;
    let handle = std::thread::spawn(move || qa.apply(stack![]));
    let zb = qb.apply(stack![]);
    let za = handle.join().unwrap();
    stack![..s, quote![..za], quote![..zb]]
}

//           Qa
// A:      *----*
//        /      \
// X: ---*--------*--->
//           Qx
pub fn climb<S, Qz, Qs>(
    stack: Stack![..S, Quote![..Qz], Quote![..Qs]],
) -> Stack![..Qs::Output, Quote![..Qz::Output]]
where
    S: Stack,
    Qz: 'static + Send + Stack + Apply<Stack![], Output: 'static + Send + Stack>,
    Qs: Stack + Apply<Stack![..S], Output: Stack>,
{
    let Unpack![..s, Quote(qz), Quote(qs)] = stack;
    let handle = std::thread::spawn(move || qz.apply(stack![]));
    let s = qs.apply(s);
    let z = handle.join().unwrap();
    stack![..s, quote![..z]]
}

//          Qa1 Qa2
// A:      *---*---*
//        /    |    \
// X: ---*    Oa     *--->
//        \    ↓    /
// B:      *---*---*
//          Qb1 Qb2
pub fn send<S, Za, Qa1, Qa2, Oa, Zb, Qb1, Qb2>(
    stack: Stack![
        ..S,
        Quote![Quote![..Qa1], Quote![..Qa2]],
        Quote![Quote![..Qb1], Quote![..Qb2]],
    ],
) -> Stack![..S, Quote![..Qa2::Output], Quote![..Qb2::Output]]
where
    S: Stack,
    Za: Stack,
    Qa1: 'static + Send + Stack + Apply<Stack![], Output = Stack![..Za, Oa]>,
    Qa2: 'static + Send + Stack + Apply<Stack![..Za], Output: 'static + Send>,
    Oa: 'static + Send,
    Zb: Stack,
    Qb1: 'static + Send + Stack + Apply<Stack![], Output = Stack![..Zb]>,
    Qb2: 'static + Send + Stack + Apply<Stack![..Zb, Oa]>,
{
    let Unpack![
        ..s,
        Quote(Unpack![Quote(qa1), Quote(qa2)]),
        Quote(Unpack![Quote(qb1), Quote(qb2)]),
    ] = stack;

    let (sender, receiver) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let Unpack![..za, oa] = qa1.apply(stack![]);
        sender.send(oa).unwrap();
        qa2.apply(za)
    });

    let zb = qb1.apply(stack![]);
    let oa = receiver.recv().unwrap();
    let zb = qb2.apply(stack![..zb, oa]);

    let za = handle.join().unwrap();
    stack![..s, quote![..za], quote![..zb]]
}

//          Qa1 Qa2 Qa3
// A:      *---*---*---*
//        /    |   ↑    \
// X: ---*    Oa  Ob     *--->
//        \    ↓   |    /
// B:      *---*---*---*
//          Qb1 Qb2 Qb3
pub fn reply<S, Za1, Za2, Qa1, Qa2, Qa3, Oa, Zb1, Zb2, Qb1, Qb2, Qb3, Ob>(
    stack: Stack![
        ..S,
        Quote![Quote![..Qa1], Quote![..Qa2], Quote![..Qa3]],
        Quote![Quote![..Qb1], Quote![..Qb2], Quote![..Qb3]],
    ],
) -> Stack![..S, Quote![..Qa3::Output], Quote![..Qb3::Output]]
where
    S: Stack,
    Za1: Stack,
    Za2: Stack,
    Qa1: 'static + Send + Stack + Apply<Stack![], Output = Stack![..Za1, Oa]>,
    Qa2: 'static + Send + Stack + Apply<Za1, Output = Za2>,
    Qa3: 'static + Send + Stack + Apply<Stack![..Za2, Ob], Output: 'static + Send>,
    Oa: 'static + Send,
    Zb1: Stack,
    Zb2: Stack,
    Qb1: 'static + Send + Stack + Apply<Stack![], Output = Zb1>,
    Qb2: 'static + Send + Stack + Apply<Stack![..Zb1, Oa], Output = Stack![..Zb2, Ob]>,
    Qb3: 'static + Send + Stack + Apply<Zb2, Output: 'static + Send>,
    Ob: 'static + Send,
{
    let Unpack![
        ..s,
        Quote(Unpack![Quote(qa1), Quote(qa2), Quote(qa3)]),
        Quote(Unpack![Quote(qb1), Quote(qb2), Quote(qb3)]),
    ] = stack;

    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let Unpack![..za1, oa] = qa1.apply(stack![]);
        out_sender.send(oa).unwrap();

        let za2 = qa2.apply(za1);

        let ob = in_receiver.recv().unwrap();
        qa3.apply(stack![..za2, ob])
    });

    let zb1 = qb1.apply(stack![]);

    let oa = out_receiver.recv().unwrap();
    let Unpack![..zb2, ob] = qb2.apply(stack![..zb1, oa]);

    in_sender.send(ob).unwrap();

    let b = qb3.apply(zb2);
    let a = handle.join().unwrap();
    stack![..s, quote![..a], quote![..b]]
}

//            Qa1 Qa2 Qa3
// A:        *---*---*---*
//          /    |   ↑    \
//         /    Oa  Ob     \
//        /       \ /       \
// X: ---*         *         *--->
//        \       / \       /
//         \    Ob  Oa     /
//          \    |   ↓    /
// B:        *---*---*---*
//            Qb1 Qb2 Qb3
pub fn exchange<S, Za, Qa1, Qa2, Qa3, Oa, Zb, Qb1, Qb2, Qb3, Ob>(
    stack: Stack![
        ..S,
        Quote![Quote![..Qa1], Quote![..Qa2], Quote![..Qa3]],
        Quote![Quote![..Qb1], Quote![..Qb2], Quote![..Qb3]],
    ],
) -> Stack![..S, Quote![..Qa3::Output], Quote![..Qb3::Output]]
where
    S: Stack,
    Za: Stack,
    Qa1: 'static + Send + Stack + Apply<Stack![], Output = Stack![..Za, Oa]>,
    Qa2: 'static + Send + Stack + Apply<Za>,
    Qa3: 'static + Send + Stack + Apply<Stack![..Qa2::Output, Ob], Output: 'static + Send>,
    Oa: 'static + Send,
    Zb: Stack,
    Qb1: 'static + Send + Stack + Apply<Stack![], Output = Stack![..Zb, Ob]>,
    Qb2: 'static + Send + Stack + Apply<Zb>,
    Qb3: 'static + Send + Stack + Apply<Stack![..Qb2::Output, Oa], Output: 'static + Send>,
    Ob: 'static + Send,
{
    let Unpack![
        ..s,
        Quote(Unpack![Quote(qa1), Quote(qa2), Quote(qa3)]),
        Quote(Unpack![Quote(qb1), Quote(qb2), Quote(qb3)]),
    ] = stack;

    let (sender_ab, receiver_ab) = mpsc::channel();
    let (sender_ba, receiver_ba) = mpsc::channel();

    let handle = std::thread::spawn(move || {
        let Unpack![..za, oa] = qa1.apply(stack![]);
        sender_ba.send(oa).unwrap();

        let za = qa2.apply(za);

        let ob = receiver_ab.recv().unwrap();
        qa3.apply(stack![..za, ob])
    });

    let Unpack![..zb, ob] = qb1.apply(stack![]);
    sender_ab.send(ob).unwrap();

    let zb = qb2.apply(zb);

    let oa = receiver_ba.recv().unwrap();

    let b = qb3.apply(stack![..zb, oa]);
    let a = handle.join().unwrap();
    stack![..s, quote![..a], quote![..b]]
}

//             Qa
// A:        *----*
//          /     |
//         /      Z (if Qa ends before Qb)
//        /  Qs   ↓
// X: ---*--------*--->
//        \       ↑
//         \      Z (if Qb ends before Qa)
//          \     |
// B:        *----*
//             Qb
pub fn race<S, Qa, Qb, Qs, Z>(
    stack: Stack![..S, Quote![..Qa], Quote![..Qb], Quote![..Qs]],
) -> Stack![..Qs::Output, Quote![..Z]]
where
    S: Stack,
    Qa: 'static + Send + Stack + Apply<Stack![], Output = Z>,
    Qb: 'static + Send + Stack + Apply<Stack![], Output = Z>,
    Qs: Stack + Apply<S>,
    Z: 'static + Send + Stack,
{
    let Unpack![..s, Quote(qa), Quote(qb), Quote(qs)] = stack;
    let (sender, receiver) = mpsc::channel();
    let (sender_a, sender_b) = (sender.clone(), sender);

    std::thread::spawn(move || sender_a.send(qa.apply(stack![])));
    std::thread::spawn(move || sender_b.send(qb.apply(stack![])));

    let s = qs.apply(s);
    let z = receiver.recv().unwrap();
    stack![..s, quote![..z]]
}
