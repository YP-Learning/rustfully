fn main() {
    let n = 5;

    {
        let n = 10; // this redeclaration is possible due to shadowing
        // but i think this should be scoped variable instead of shadowing
        println!("inner n = {n}");
    }

    println!("outter n = {n}");

    // another use of shadowing
    let spaces = "     "; // using spaces as string variable
    let spaces = spaces.len(); // we will never use the spaces variable, so using shadowing
    // to convert it to usize
    println!("spaces = {spaces}");
}
