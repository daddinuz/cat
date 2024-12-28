use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let fib = stack![
        quote![dup, 3, lt],
        quote![pop, 1],
        quote![decr, dup, decr],
        quote![add],
        binrec
    ];
    let program = stack![30, fib, display];
    program.apply(());
}
