fn main() {
    // let number = 3;

    // if number < 5 {
    // 	println!("condition was true");
    // } else {
    // 	println!("condition was false");
    // }

    // let condition = true;
    // let number = if condition {
    // 	5;
    // } else {
    // 	6;
    // };

    // println!("The value of number is: {}", number);

    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter() {
    // 	println!("the value is: {}", element);
    // }

    // for number in (1..4).rev() {
    // 	println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

	let s1 = String::from("hello");
	let len = calculate_length(&s1);
	println!("The length of '{}' is {}", s1, len);

    fn calculate_length(s: &String) -> usize {
    	s.len()
    }
}
