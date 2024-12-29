use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let client = flow![
        flow!["syn"],
        flow![],
        flow!["ack", eq, "connected", "aborted", choose, display],
    ];

    let server = flow!["syn", eq, "ack", "KO", choose];

    let program = flow![Quote(client), Quote(server), reply];
    program.apply(());
}
