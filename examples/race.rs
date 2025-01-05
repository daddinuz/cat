use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let foo = flow!["foo"];
    let bar = flow!["bar"];

    let program = flow![
        Quote(foo),
        Quote(bar),
        Quote(flow![String::from("spam ")]),
        race,
        unstack,
        concat,
        display,
    ];
    program.apply(())
}
