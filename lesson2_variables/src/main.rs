fn main() {
    // part 1
    let mut x: i32 = 3; // initialized but used, passes
    let _y: i32; // uninitialized but unused, warning, if we didn't put underscore

    x += 2;
    assert_eq!(x, 5);
    println!("x: {}", x);
    println!("Success!");

    // part 2
    let mut z = 1;
    z = z + 2;
    println!("z = {}", z);

    // part 3
    let x2: i32 = 10;
    let y2: i32 = 5;

    {
        println!("The value of x2 is {} and value of y2 is {}", x2, y2);
    }

    println!("The value of x2 is {} and value of y2 is {}", x2, y2);

    // part 4
    print_hell_world();

    // part 5
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    
    println!("The value of x is {}", x);
    assert_eq!(x, 5);
    let x = 42;
    println!("The value of x is {}", x);

    // part 6
    let mut b: i32 = 1;
    println!("The value of b is {}", b);

    b = 7;
    let c: i32 = 4;

    println!("The value of b is {} and value of c is {}", b, c);
    
    // shadowing and re-binding, not x is immutable
    let b = b;

    // shadowing
    let c: &str = "I can also be bound to text!";

    println!("The value of b is {} and value of c is {}", b, c);
    println!("Success after shadowing and re-binding!");

    // part 7 unused variables produce warnings
    part7();

    // part 8
    part8();

    part9();
}

fn print_hell_world() {
    let a = "hello";
    println!("{}, world!", a);
}

#[allow(unused_variables)]
fn part7() { 
    let _x = 1;
}

fn part8() {
    // tuple assignment 
    let (mut x, y) = (1, 2);
    x += 2;
    
    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("x: {}, y: {}", x, y);
}

fn part9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // fill the blank to make the code work
    // assert_eq!([x,y], __);
    assert_eq!([x,y], [3, 2]);

    println!("Success on part 9!");
}