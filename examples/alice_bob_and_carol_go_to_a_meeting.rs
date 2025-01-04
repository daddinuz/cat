use cat::apply::Apply;
use cat::builtin::*;
use cat::flow;
use cat::quote::Quote;

fn main() {
    let carol = flow![
        Quote(flow!["cinema", eq]),
        Quote(flow!["disco", eq]),
        y,
        fork,
        unstack2,
        or,
    ];

    let bob = flow![
        flow![],
        flow![
            Quote(carol),
            Quote(flow![
                Quote(flow!["cinema", eq]),
                Quote(flow!["restaurant", eq]),
                y,
                fork,
                unstack2,
                or,
            ]),
            y,
            fork,
            unstack2,
            and,
        ],
        flow![]
    ];

    let alice = flow![flow!["cinema"], flow![], flow![display]];

    let program = flow![Quote(alice), Quote(bob), reply];
    program.apply(());
}
