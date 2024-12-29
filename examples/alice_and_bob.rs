use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let alice = quote![
        stack!["Alice"],
        stack![],
        stack!["Alice says hi to ", print, display]
    ];

    let bob = quote!["Bob says hi to ", print, display, "Bob"];

    let program = stack![alice, bob, reply];
    program.apply(());
}
