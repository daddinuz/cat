use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, rpn};

fn main() {
    let program = rpn![
        "Alice",
        quote!["Bob says hi to ", print, display, "Bob"], // Bob
        prompt,
        "Alice says hi to ",
        print,
        display
    ];
    program.apply(());
}
