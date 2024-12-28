use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let fact = stack![
        quote![dup, 1, le],
        quote![pop, 1],
        quote![dup, decr],
        quote![mul],
        linrec
    ];
    let program = stack![5, fact, display];
    program.apply(());
}
