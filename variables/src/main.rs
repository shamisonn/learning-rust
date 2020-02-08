fn main() {
    // TODO: impl "Convert temperatures between Fahrenheit and Celsius."
    // TODO: impl "Generate the nth Fibonacci number."
    // TODO: impl "Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song."
}

fn _main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x {}", x);
    println!("x {}", tup.0);
    println!("y {}", y);
    println!("z {}", z);

    let a = [1, 2, 3, 4];
    println!("a[0] {}", a[0]);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("x + 1 {}", plus_one(x));

    let condition = true;
    let number = if condition {
        5
    } else {
        6 // "six" <- error
    };

    println!("The value of number is: {}", number);
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn plus_one(x: i32) -> i32 {
    x + 1 //; <- compile error!
}
