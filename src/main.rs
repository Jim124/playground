mod my_func;
mod other_funcs;
use crate::my_func::add_five;
use crate::other_funcs::minus_funcs::subtract_10;
fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);
    let y = add_five(x);
    println!("y is {}", y);
    let z = subtract_10(x);
    println!("z is {z}");
    x = 70;
    println!("x is {}", x);
    let my_floats :[f32;10] = [0.0;10];
    println!("{:?}",my_floats );
    let my_floats_new = my_floats.map(|n| n+2.0);
    println!("{:?}",my_floats_new );
}


fn _test(){
    let _x =5;
    let _y = 6;
}

#[allow(dead_code,unused_variables)]
fn test_1(){
    let x = 5;
    let y = 6;
}
