fn main() {
    // let data = (10, 3.5, false); // this works too
    let data: (i8, f32, bool) = (10, 3.5, false);
    println!("data = {:?}", data);

    // destructuring
    let (n, d, b) = data;
    println!("n = {n}, d = {d}, b = {b}");

    // index based
    let first = data.0;
    let second = data.1;
    let third = data.2;

    println!("first = {first}, second = {second}, third = {third}");

    // empty tuple aka 'unit'
    let empty = (); // or let empty: ()
    // this is as return for expression not returning any value
    println!("empty = {:?}", empty);
}
