fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // println!("{}, world!", s1);
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);
    // println!("s = {}", s); // Error s was moved
    let x = 5;
    makes_copy(x);
    println!("x = {}", x);

    let a1 = gives_ownership();
    let a2 = String::from("hello");
    let a3 = takes_and_gives_back(a2);
    // println!("a2 = {}", a2); // Error a2 was moved to a3

    let b1 = String::from("hello");
    let (b1, len) = calculate_length(b1);
    println!("The length of '{}' is {}", b1, len);
    println!("b1 = {}", b1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
