use cat::apply::Apply;
use cat::quote::quote;
use cat::stack::stack;
use cat::verb::*;

fn main() {
    let fib = stack![
        quote![3, lt],
        quote![pop, 1],
        quote![decr, dup, decr],
        quote![add],
        parbinrec,
    ];
    let program = stack![30, fib, display];
    program.apply(stack![]);
}
