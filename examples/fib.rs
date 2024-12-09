use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, rpn};

fn main() {
    let fib = rpn![
        quote![dup, 3, lt],
        quote![pop, 1],
        quote![decr, dup, decr],
        quote![add],
        binrec
    ];
    let program = rpn![30, fib, display];
    program.apply(());
}
