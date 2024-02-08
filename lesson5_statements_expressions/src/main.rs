fn main() {
    part1();
    part2();
    part3();
}

fn part1() {
    // everything in curly braces is an expression
    let v = {
        let mut x = 1;

        // no semicolon means this is the return value
        // with a semicolon, the value is not returned
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Success on part 1!");
}

fn part2() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);
 
    println!("Success on part 2!");
}

fn part3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success on part 3!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}