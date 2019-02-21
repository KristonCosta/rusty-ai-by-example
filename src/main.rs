mod chapter1;
mod lib;

use chapter1::part_one;
// use chapter1::part_two;
use chapter1::part_three;

enum Chapter {
    OneOne,
    OneTwo,
    OneThree,
}

fn main() {
    let val = Chapter::OneTwo;
    match val {
        Chapter::OneOne => {
            part_one::run::main()
        },
        Chapter::OneTwo => {
          //  part_two::run::main()
        },
        Chapter::OneThree => {
            part_three::run::main()
        },
    }
}