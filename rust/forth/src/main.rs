#![allow(unused_must_use)]

extern crate forth;

use forth::{Forth};

fn main() {
    let mut f = Forth::new();
    f.eval(": € 220371 ; €");
    assert_eq!("220371", f.format_stack());
}
