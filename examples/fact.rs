use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let fact = flow![
        Quote(flow![dup, 1, le]),
        Quote(flow![pop, 1]),
        Quote(flow![dup, decr]),
        Quote(flow![mul]),
        linrec
    ];
    let program = flow![5, fact, display];
    program.apply(());
}
