mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetime;
mod m6_matches;
mod m7_async;
mod m8_handle_error;
mod m9_collections;
mod m10_decl_macros;
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
    let my_floats: [f32; 10] = [0.0; 10];
    println!("{:?}", my_floats);
    let my_floats_new = my_floats.map(|n| n + 2.0);
    println!("{:?}", my_floats_new);

    let name: &str = "Jim";
    println!("{}", name);
    let name_literal: String = name.to_string();
    println!("name is in memory {:p}", &name_literal);
    let length = name_literal.len();
    println!("the length is {}", length);

    let str_slice = &name_literal[0..3];
    println!("str slice is {}", str_slice);

    // Vec
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.insert(3, 'l');
    chars.insert(4, 'o');
    chars.push('.');
    println!("{:?}", chars);
    let removed_char: char = chars.pop().unwrap();
    println!("{}", removed_char);
    println!("{:?}", chars);
    // let chars_again = vec!['h'];
    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    chars_again.iter().for_each(|c| println!("{}", c));
    let words: String = chars_again.iter().collect();
    println!("{}", words);
    for i in chars_again {
        print!("{}", i);
        if i == 'o' {
            print!(", world!\n");
        }
    }
    // println!("{:?}",chars_again );
    // Raw string
    let text = r#"{"message":"Rust is awesome"}"#;
    println!("{}", text);
    let my_vec = my_vec1!(1,2,3);
    println!("{:?}",my_vec );
}

fn _test() {
    let _x = 5;
    let _y = 6;
}

#[allow(dead_code, unused_variables)]
fn test_1() {
    let x = 5;
    let y = 6;
}
