

fn main() {

    let s1 = String::from("hello");
    //Let's pass in a reference (like adress) of s1
    //This doesn't remove ownership of s1.
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

}

//This takes in a reference to a string and return
//an unsigned integer of the arch size.
fn calculate_length(s: &String) -> usize {
    s.len()
}
