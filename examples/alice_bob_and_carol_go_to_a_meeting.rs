use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, rpn};

fn main() {
    let carol = quote!["cinema", eq];

    let bob = quote![
        carol,        // Bob asks Carol
        quote![true], // In the meanwhile Bob applies his logic
        branch,       // Parallel application of the quotations above
        and
    ];

    // The definition below is equivalent
    // let bob = quote![
    //     quote![carol, prompt], // Bob asks Carol
    //     quote![pop, true],     // In the meanwhile Bob applies his logic
    //     parapp1,               // Parallel application of the quotations above (unary functions)
    //     and
    // ];

    // Alice's point of view
    let program = rpn!["cinema", bob, prompt, display];
    program.apply(());
}
