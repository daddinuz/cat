use cat::apply::Apply;
use cat::quote::quote;
use cat::stack::stack;
use cat::verb::*;

fn main() {
    let client = quote![
        quote!["syn"],
        quote!["[client] trying to connect...", display],
        quote![
            "ack",
            eq,
            quote!["[client] connection succeded"],
            quote!["[client] connection failed"],
            if_else,
            display
        ],
    ];

    let server = quote![
        quote!["[server] waiting for incoming connections...", display],
        quote!["syn", eq, quote!["ack"], quote!["KO"], if_else],
        quote!["[server] connection accepted", display]
    ];

    let program = stack![client, server, reply];
    program.apply(stack![]);
}
