fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("numbers = {:?}", numbers);

    // this is fixed size, for variable size there is vector
    let nums: [u8; 3] = [8, 9, 10];
    println!("nums = {:?}", nums);

    let ones = [1; 10];
    println!("ones = {:?}", ones);

    let days = ["Mon", "Tues", "Wed", "Thurs", "Fri", "Sat", "Sun"];
    let first = days[0];
    let last = days[6];
    println!("first = {first}");
    println!("last = {last}");
    println!("there are {} days", days.len());
}
