use cat::apply::Apply;
use cat::quote::quote;
use cat::stack::stack;
use cat::verb::*;

fn main() {
    let fact = stack![
        quote![1, le],
        quote![pop, 1],
        quote![dup, decr],
        quote![mul],
        linrec
    ];
    let program = stack![5, fact, display];
    program.apply(stack![]);
}
