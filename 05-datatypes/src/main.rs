fn main() {
    let user_input = "100";
    // here u32 is required as for parse to identify to which type its to be converted to
    let converted: u32 = user_input.parse().expect("Failed to convert to u32.");

    println!("converted user input = {converted}");

    // scalar types
    let int: i8 = 10;
    let pi: f32 = 3.1415;
    let boolean: bool = false;
    let delta: char = 'd'; 

    println!("int = {int}\npi = {pi}\nboolean = {boolean}\ndelta = {delta}");

    // compound types 
    let coordinates: (f32, f32) = (1.5, 2.5);
    let people: [&str; 3] = ["Yash", "Pravin", "Pranita"];


    println!("coordinates = {:?}", coordinates);
    println!("people = {:#?}", people);
}
