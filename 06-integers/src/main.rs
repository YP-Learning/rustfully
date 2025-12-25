fn main() {
    let n1: i8 = 10;  // -128 to 127
    let n2: u8 = 200; // 0 to 255

    // datatypes available are 
    // i8, i16, i32, i64, i128, isize (based on arch)
    // u8, u16, u32, u64, u128, usize 
    println!("n1 = {n1} \tn2 = {n2}");

    // we can figure out the max & min value of any int datatype
    println!("[i8] max = {}, min = {}", i8::MAX, i8::MIN);
    println!("[isize] max = {}, min = {}", isize::MAX, isize::MIN);
    println!("[usize] max = {}, min = {}", usize::MAX, usize::MIN);

    // IMPORTANT: in debug mode rust panics on overflow of variable, in release mode it wraps
    
    let trillion: i64 = 1_000_000_000_000;
    println!("trillion is {}", trillion);
}

