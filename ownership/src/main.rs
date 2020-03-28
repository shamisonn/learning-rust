fn main() {
    // string and String
    let mut s = "hello".to_string();
    s = s + ", world";
    println!("{}", s);

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    // stack vars activated auto copy
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    // heap vars can't active auto copy
    let s1 = String::from("hello");
    let s2 = s1;
    // ^ move pointer. so s1 pointer point null(=delete)
    // println!("{}", s1); // <- compile error!
    println!("{}", s2);

    // so use clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);

    // function calling is likely same: variable assignment
    let s = String::from("hello");
    heap_args_function(s);
    // println!(s); // <- compile error: s moved to function
    let x = 5;
    stack_args_function(x);
    println!("{}", x); // ok

    // so use clone
    let s = String::from("hello");
    heap_args_function(s.clone());
    println!("{}", s);
}

fn heap_args_function(some_string: String) {
    println!("{}", some_string);
} // free some_string memory.

fn stack_args_function(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer is copied. so this is ok.
