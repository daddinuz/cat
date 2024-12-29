use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let client = quote![
        stack!["syn"],
        stack![],
        stack!["ack", eq, "connected", "aborted", choose, display]
    ];

    let server = quote!["syn", eq, "ack", "KO", choose];

    let program = stack![client, server, reply];
    program.apply(());
}
