use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let carol = quote![quote!["cinema", eq], quote!["disco", eq], app1, or];

    let bob = quote![
        carol,                                                            // Bob asks Carol
        quote![quote!["cinema", eq], quote!["restaurant", eq], app1, or], // In the meanwhile Bob applies his logic
        parapp1, // Parallel application of the quotations above
        and
    ];

    let alice = quote![stack!["cinema"], stack![], stack![display]];

    let program = stack![alice, bob, reply];
    program.apply(());
}
