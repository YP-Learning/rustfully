fn main() {
    // `let` cannot be used in global scope, `const` can be

    // Immutable variables, this is by default
    // once value is assigned you cannot change it
    // `let` is used to declare a variable;
    let number = 10; // this is Immutable
    println!("number = {number}");

    // this will throw an error as number is immutable
    // number = 20;
    
    // a new mutable variable
    let mut num = 10;
    println!("num = {num}");

    num += 2;
    println!("num = {num}");

    // we use snake case
    let first_name = "Yash";
    println!("My name is {first_name}");

    // to create a constant we create a `const` keyword
    // snake case but all uppercase letters
    // it cannot be assigned runtime value, its like #define
    // i believe we require the dtype to be specified
    const ONE_HOUR: i32 = 60;
    println!("ONE_HOUR is {ONE_HOUR}");

    // these are important values, never to be changed
    const PI: f64 = 3.14159; // this can be in any scope in any errors
    println!("PI is {PI}");
}

