fn main() {
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

    let p = &x;

    // {:p} is a format specifier for printing the memory address of a variable
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84

    println!("Success on part 1!");
}

fn part2() {
    let x: i32 = 5;
    let y: &i32 = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success on part 2!");
}

fn part3() {
    let mut s: String = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success on part 3!");
}

fn borrow_object(_s: &String) {}

fn part4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success on part 4!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn part5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success on part 5!");
}

fn part6() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success on part 6!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn part7() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success on part 7!");
}

// Error: Borrow an immutable object as mutable
fn part8() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object2(&mut s);

    println!("Success on part 8!");
}

fn borrow_object2(s: &mut String) {}

// Ok: Borrow a mutable object as immutable
// This code has no errors!
fn part9() {
    let mut s = String::from("hello, ");

    borrow_object3(&s);
    
    s.push_str("world");

    println!("Success on part 9!");
}

fn borrow_object3(s: &String) {}

fn part10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    println!("{}", s);

    println!("Success on part 10!");
}

fn part11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    // println!("{}, {}", r1, r2);

    println!("Success on part 11!");
}
