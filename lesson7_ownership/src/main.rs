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
    // println!("All parts complete for lesson 7");
}

fn part1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}", x, y);

    println!("Success on part 1!");
}

fn part2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    println!("Success on part 2!");
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    s
}

fn part3() {
    let s = give_ownership();
    println!("{}", s);

    println!("Success on part 3!");
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    s
}

fn part4() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);

    println!("Success on part 4!");
}

fn print_str(s: String) {
    println!("{}", s);
}

// Don't use clone, use copy instead
fn part5() {
    // changed String to &str, which is a string literal and is a fixed size
    // because &str is immutable it is not on the heap and can be safely copied
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);

    println!("Success on part 5!");
}

fn part6() {
    let s = String::from("hello, ");
    
    // Modify this line only !
    let mut s1 = s;

    s1.push_str("world");

    println!("Success on part 6!");
}

fn part7() {
    let x: Box<i32> = Box::new(5);
    
    let mut y = Box::new(4);      // Implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success on part 7!");
}

fn part8() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s = t.0;
 
    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
    
    println!("Success on part 8!");
}

fn part9() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2): (String, String) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
    
    println!("Success on part 9!");
}
