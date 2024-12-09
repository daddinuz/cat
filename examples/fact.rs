use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, rpn};

fn main() {
    let fact = rpn![
        quote![dup, 1, le],
        quote![pop, 1],
        quote![dup, decr],
        quote![mul],
        linrec
    ];
    let program = rpn![5, fact, display];
    program.apply(());
}
