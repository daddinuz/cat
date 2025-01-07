use cat::apply::Apply;
use cat::quote::quote;
use cat::stack::stack;
use cat::verb::*;

fn main() {
    let alice = quote![
        quote!["Alice"],
        quote![],
        quote!["Alice says hi to ", print, display],
    ];

    let bob = quote![
        quote![],
        quote!["Bob says hi to ", print, display, "Bob"],
        quote![]
    ];

    let program = stack![alice, bob, reply];
    program.apply(stack![]);
}
