use cat::apply::Apply;
use cat::builtin::*;
use cat::{quote, stack};

fn main() {
    let program = stack![
        "Alice",
        quote!["Bob says hi to ", print, display, "Bob"], // Bob
        prompt,
        "Alice says hi to ",
        print,
        display
    ];
    program.apply(());
}
