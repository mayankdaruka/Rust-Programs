fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // let spaces = "   ";
    // let spaces = spaces.len();

    let mut spaces = "spaces";
    println!("{}", spaces);

    spaces = "hahah";

    println!("{}", spaces);

    let unsigned : u8 = 255;

    // out of range
    // let signed : i8 = 255;
    let signed : i8 = 127;

    println!("Unsigned: {}, Signed: {}", unsigned, signed);

    // let y : f32 = 3; // Mismatched types
    let mut y : f32 = 3.0;
    println!("{}", y); // prints 3 without the .0
    y = 3.5;
    println!("{}", y);

    let f : bool = false;
    // chars are 4 bytes in size, unlike java where they are 2 bytes
    let c : char = '😻'; // can represent more than just ASCII
    println!("{} {}", f, c);

    // compound types in rust: tuples and arrays
    let tup: (i32, f64, u8) = (30, 4.4, 200);
    // println!("{}", tup); // cannot be formatted with the default formatter
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    let first_elem = tup.0;
    println!("{}", first_elem);

    // let nonexistent_elem = tup.3; // no field 3 on type _____

    // let arr = [1, 2, 3, 4];
    let a : [u16; 6] = [33, 654, 12, 76, 1, 77];
    // println!("{}", a); // cannot be formatted with the default formatter

    let first_elem : u16 = a[0];
    println!("{}", first_elem);

    another_function(10);
    println!("-----------");
    another_function(16);

    // let x = (let y = 6); // error because let y = 6 does not return a value
    let y = {
        let x = 4;
        x + 1 // this is an expression. once you add a semicolon at the end, it is a statement
    };
    println!("The value of y is: {}", y);

    let (x, y) = return_function();
    println!("The return values are: {}, {}", x, y);

    println!("Plus one: {}", plus_one(5));


    // control flow

    let num = 5;

    if num > 0 {
        println!("the number is positive");
    } else if num < 0 {
        println!("the number is negative");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Value of number: {}", number);

    let mut x = 0;

    'outer_loop: loop {
        let mut y = 10;
        println!("outer loop {}", x);
        loop {
            // println!("inner loop {}", y);
        
            // if x == 5 {
            //     break;
            // }
            if y == 0 && x == 2 {
                println!("break out of the outer loop");
                break 'outer_loop;
            }

            if y == -2 {
                break;
            }

            y -= 2;
        }
        x += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 3;
        }
    };

    println!("The value of result is: {}", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}


fn another_function(x : i8) {
    println!("Another function. {}", x);
}

fn return_function() -> (i32, f32) {
    let x = 2;
    let y = 4.4;
    (x, y)
}

fn plus_one(x : i8) -> i8 {
    x + 1
}
