use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let program = stack![
        quote!["syn"],                                     // client1
        quote!["syn", eq, "ack", "KO", choose],            // server1
        quote!["ack", eq, "connected", "aborted", choose], // client2
        reply,
        display
    ];
    program.apply(());
}
