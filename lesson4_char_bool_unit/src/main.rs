use std::mem::size_of_val;

fn main() {
    part1();
    part2();
    part3();
    part4();
    part5();
    part6();

}

fn part1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success on part 1!");
}

fn part2() {
    let c1: char = '中';
    print_char(c1);

    println!("Success on part 2!");
}

fn print_char(c : char) {
    println!("{}", c);
}

fn part3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success on part 3!");
    }
}

fn part4() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success on part 4!");
}

fn part5() {
    // empty tuple
    let _v: () = ();

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success on part 5!");
}

// implicitly returns unit type empty tuple
fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

fn part6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success on part 6!");
}