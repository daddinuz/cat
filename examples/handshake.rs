use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, rpn};

fn main() {
    let program = rpn![
        quote!["syn"],                                     // client1
        quote!["syn", eq, "ack", "KO", choose],            // server1
        quote!["ack", eq, "connected", "aborted", choose], // client2
        reply,
        display
    ];
    program.apply(());
}
