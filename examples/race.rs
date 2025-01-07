use cat::apply::Apply;
use cat::quote::quote;
use cat::stack::stack;
use cat::verb::*;

fn main() {
    let program = stack![
        quote!["foo"],
        quote!["bar"],
        quote![String::from("spam ")],
        race,
        unquote,
        concat,
        display
    ];
    program.apply(stack![]);
}
