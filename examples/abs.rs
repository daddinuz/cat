use cat::apply::Apply;
use cat::quote::quote;
use cat::stack::stack;
use cat::verb::*;

fn main() {
    let abs = stack![dup, 0, lt, quote![neg], r#if];
    let program = stack![-42, abs, display];
    program.apply(stack![]);
}
