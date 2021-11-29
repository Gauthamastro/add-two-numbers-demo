use crate::ffi::GoInt;

mod ffi;

fn main() {
    println!("Hello, world!");
    let result = unsafe { ffi::Add(1 as GoInt, 2 as GoInt) };
    println!("Result: {:?}",result);
}
