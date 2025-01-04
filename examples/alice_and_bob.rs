use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let alice = flow![
        flow!["Alice"],
        flow![],
        flow!["Alice says hi to ", print, display],
    ];

    let bob = flow![
        flow![],
        flow!["Bob says hi to ", print, display, "Bob"],
        flow![]
    ];

    let program = flow![Quote(alice), Quote(bob), reply];
    program.apply(());
}
