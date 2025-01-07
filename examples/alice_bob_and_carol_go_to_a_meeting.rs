use cat::apply::Apply;
use cat::quote::quote;
use cat::stack::stack;
use cat::verb::*;

fn main() {
    let carol = quote![
        quote!["cinema", eq],
        quote!["disco", eq],
        y,
        fork,
        unquote2,
        or,
    ];

    let bob = quote![
        quote![],
        quote![
            carol,
            quote![
                quote!["cinema", eq],
                quote!["restaurant", eq],
                y,
                fork,
                unquote2,
                or,
            ],
            y,
            fork,
            unquote2,
            and,
        ],
        quote![]
    ];

    let alice = quote![quote!["cinema"], quote![], quote![display]];

    let program = stack![alice, bob, reply];
    program.apply(stack![]);
}
