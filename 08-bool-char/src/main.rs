fn main() {
    let connected_to_internet: bool = false;
    let has_cat: bool = true;

    println!("connected_to_internet = {connected_to_internet}");
    println!("has_cat = {has_cat}");

    let money = 5000;
    println!("money > 0 = {}", money > 0);

    if money > 0 {
        println!("you are not broke!");
    }

    // lets look at char
    let letter: char = 'z';
    let omega = 'Ω';

    println!("letter = {letter}");
    println!("omega = {omega}");
}
