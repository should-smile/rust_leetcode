#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use std::cell::RefCell;

fn tmp1() {
    let mut a = 10;
    let mut v = 3;
    let b: RefCell<&mut i32> = RefCell::from(&mut a);

    *b.borrow_mut() = &mut v;
    println!("{}", *b.borrow());
    println!("{}", a);
}

fn tmp2() {
    use std::cell::RefCell;

    let c = RefCell::new(5);

    let five = c.into_inner();

}

fn main() {
    tmp1();
}