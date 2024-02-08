use std::ops::Range;
use std::ops::RangeInclusive;

fn main() {
    println!("Lesson 3: Numbers");

    part1();
    part2();
    part3();
    part4();
    part5();
    part6();
    part7();
    part8();
    part9();
    part10();
    part11();
}

fn part1() {
    let x: i32 = 5;
    let mut y: u32 = 5;
    println!("x: {}, y: {}", x, y);

    // cast x to u32
    y = x as u32;

    let z = 10; // Q: What is the type of z? Answer, it defaults to i32
    println!("x: {}, y: {}, z: {}", x, y, z);

    println!("Success on part 1!");
}

fn part2() {
    // we can assign type as part of integer 32_u8 is 32 as an 8 bit integer
    // cast 32 8bit to 16bit to match the type of v
    let v: u16 = 32_u8 as u16;

    println!("v: {}", v);
    println!("Success on part 2!");
}

fn part3() {
    // compiler infers type of i32
    let x = 5;

    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success on part 3!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}

fn part4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success on part 5!");
}

fn part5() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();

    println!("{}, {}", v1, v2);
    println!("Success on part 5!");
}

fn part6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;

    assert!(v == 1597);

    println!("Success on part 6!");
}

fn part7() {
    let x = 1_000.000_1; // = 1000.0001 f32
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());

    println!("Success on part 7!");
}

fn part8() {
    assert_eq!(0.1 as f32 + 0.2 as f32, 0.3 as f32);

    println!("Success on part 8!");
}

fn part9() {
    let mut sum: i32 = 0;

    // .. does not include the last element
    for i in -3..2 { // -3 + -2 + -1 + 0 + 1 (does not include 2)
        sum += i;
    }

    assert_eq!(-5, sum);

    // ..= includes the last element
    for c in 'a' ..= 'z' {
        println!("{}", c as i32);
    }

    println!("Success on part 9!");
}

fn part10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success on part 10!");
}

fn part11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255); 
    
    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);

    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {}", 1 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80 >> 0x02);

    println!("Success on part 11!");
}