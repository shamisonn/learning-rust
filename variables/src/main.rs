fn main() {
    // DONE: impl "Convert temperatures between Fahrenheit and Celsius."
    println!("{} ºC", convert_f_to_c(convert_c_to_f(24.0)));
    // DONE: impl "Generate the nth Fibonacci number."
    for i in 0..10 {
        println!("fib({}): {}", i, fib(i));
    }
    // DONE: impl "Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song."
    lyrics();
    _main();
}

fn convert_f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}
fn convert_c_to_f(c: f32) -> f32 {
    c * 1.8 + 32.0
}

fn fib(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn lyrics() {
    let lyrics_part = [
        "2 Turtle Doves",
        "3 French Hens",
        "4 Calling Birds",
        "5 Golden Rings",
        "6 Geese a Laying",
        "7 Swans a Swimming",
        "8 Maids a Milking",
        "9 Ladies Dancing",
        "10 Lords a Leaping",
        "11 Pipers Piping",
        "12 Drummers Drumming",
    ];

    for i in 0..12 {
        println!("On the first day of Christmas");
        println!("my true love sent to me:");
        for j in (0..i).rev() {
            println!("{}", lyrics_part[j]);
        }

        println!("and a Partridge in a Pear Tree");
        println!();
    }
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
