use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let carol = flow![
        Quote(flow!["cinema", eq]),
        Quote(flow!["disco", eq]),
        app1,
        or,
    ];

    let bob = flow![
        Quote(carol), // Bob asks Carol
        Quote(flow![
            Quote(flow!["cinema", eq]),
            Quote(flow!["restaurant", eq]),
            app1,
            or,
        ]), // In the meanwhile Bob applies his logic
        parapp1,      // Parallel application of the quotations above
        and,
    ];

    let alice = flow![flow!["cinema"], flow![], flow![display]];

    let program = flow![Quote(alice), Quote(bob), reply];
    program.apply(());
}
