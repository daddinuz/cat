use cat::apply::Apply;
use cat::builtin::*;
use cat::quote::Quote;
use cat::flow;

fn main() {
    let abs = flow![dup, 0, lt, Quote(flow![neg]), r#if];
    let program = flow![-42, abs, display];
    program.apply(());
}
