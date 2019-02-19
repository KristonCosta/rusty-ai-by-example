extern crate enum_dispatch;

mod chapter1;
mod lib;

enum Chapter {
    OneOne,
    OneTwo,
    OneThree,
}

fn main() {
    let val = Chapter::OneOne;
    match val {
        Chapter::OneOne => {
            chapter1::run::main()
        },
        Chapter::OneTwo => {},
        Chapter::OneThree => {},
    }
}