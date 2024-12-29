use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let fib = flow![
        Quote(flow![dup, 3, lt]),
        Quote(flow![pop, 1]),
        Quote(flow![decr, dup, decr]),
        Quote(flow![add]),
        binrec,
    ];
    let program = flow![30, fib, display];
    program.apply(());
}
