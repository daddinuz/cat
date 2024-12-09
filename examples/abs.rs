use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, rpn};

fn main() {
    let abs = rpn![dup, 0, lt, quote![neg], r#if];
    let program = rpn![-42, abs, display];
    program.apply(());
}
