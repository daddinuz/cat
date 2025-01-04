use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let client = flow![
        flow!["syn"],
        flow!["[client] trying to connect...", display],
        flow![
            "ack",
            eq,
            "[client] connection succeded",
            "[client] connection failed",
            choose,
            display
        ],
    ];

    let server = flow![
        flow!["[server] waiting for incoming connections...", display],
        flow!["syn", eq, "ack", "KO", choose],
        flow!["[server] connection accepted", display]
    ];

    let program = flow![Quote(client), Quote(server), reply];
    program.apply(());
}
