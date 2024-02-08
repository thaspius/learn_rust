fn main() {
    part1();
    part2();
    // part3();
    part4();
    part5();
}

fn part1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success on part 1!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn part2() {
    print();
}

// Replace i32 with another type
fn print() -> () {
    println!("Success on part 2!");
 }

fn part3() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!("I will never return!");
}

fn part4() {
    println!("Success on part 4!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        // switch if tp == 1
        1 => {
            // TODO
        }
        // switch default option
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    // panic!();
    // unimplemented!()
    todo!()
}

fn part5() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success on part 5!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}