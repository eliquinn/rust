

fn main() {
    let mut s = String::from("poop");
    //you can make a new scope with brakcets
    {
        let r1 = &mut s;
        println!("{r1}");
    }
    
    //make as many mutable references as you want
    
    //let r1 = &s;
   // let r2 = &s;
    //but if there is a mutable reference, there can only be one reference
  //  let r3 = &mut s;
    change(&mut s); 
    println!("{s}");
}

//mutable reference. 
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn thing() {

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
