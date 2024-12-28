use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let abs = stack![dup, 0, lt, quote![neg], r#if];
    let program = stack![-42, abs, display];
    program.apply(());
}
