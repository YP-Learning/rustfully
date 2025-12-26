fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // return celsius * 9.0 / 5.0 + 32.0;
    // implicit return ';' is skipped
    celsius * 9.0 / 5.0 + 32.0
}

fn add(a: i32, b: i32) -> isize {
    println!("adding {a} and {b}");
    (a + b) as isize // implicit return + typecasting
}

fn main() {
    hello("Yash");
    hello("Rohan");

    let c = 32.0;
    // .0 is needed, for it to be f64
    println!("{c} deg C is {} deg F", celsius_to_fahrenheit(c));
    
    let result = add(5, 9);
    dbg!(result);
}
