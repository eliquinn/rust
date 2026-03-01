

fn main() {
}

fn asdfn() {
    
    let s1 = gives_ownership(); //refers to the function's 's'
    
    let s2 = String::from("hello");    

    let s3 = takes_and_gives_back(s2);
    //s2 is moved into takes_.. which also moves its return value
    //into s3.

}


fn gives_ownership() -> String {    //will move the return value
                                    //into the function that calls it

    let some_string = String::from("yours");
    some_string // we can return like this!! WOWO!!!
                // ownership of this will change

}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    //
    a_string //is returned back
}
fn asdf() {
    let s = String::from("hello"); //s comes into scope
    
    // s's value moves into the function
    // and is no longer valid
    takes_ownership(s);
    //s is out of scope now
   // because it is on the heap, or does not implement the copy trait? 
    
    println!("{s}");
    let x = 5;
    // copies into the function


    makes_copy(x);

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn scope() {

    //s is not valid here, since it's not yet declared
    let mut s = String::from("hello"); 
    //s is valid from this point forward
    s.push_str(", world!");
    println!("Hello, world!");
} // s is no longer valid, out of scope
// the memory is now freed automatically with the drop function

fn Poop() {
    let x = 5;
    let y = x; // copy x to y
    
    let s1 = String::from("hello");
    let s2 = s1; //merely a pointer to the same data on the heap
    //s1 is no longer valid. it poits to nothing
    //when both of these go out of scope, they will try to free the same memory.
    //this is known as a double free error
    //to ensure memory safety, rust considers s1 no longer valid.

    //to make a real clone, we must do this
    let s3 = s2.clone();

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");
} 
