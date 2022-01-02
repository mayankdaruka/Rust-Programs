/** learn about ownership - rust's central feature
 * no garbage collection, AND no explicitly allocating and freeing memory
 * third approach - memory managed through system of ownership w/ set of rules that compiler checks at compile time
 * relation to values stored on either stack or heap
 */

/** 3 rules of ownership
 * each value in rust has a variable called its owner
 * can only have 1 owner at a time
 * when owner goes out of scope, value is dropped
 */

/** when a variable comes into scope, it is valid
 * variable remains valid until it goes out of scope
 */

fn main() {
    // first string type: string literals - immutable
    // second string type: String - manages data allocated on heap (can be mutated as well)
    // why the difference? comes down to how these two types deal with memory
    let mut s = String::from("hello"); // memory requested from allocator during runtime
    println!("{}", s);
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hi");
    // shallow copy
    let s2 = s1; // copies over pointer, len, capacity of s1 to s2 (not actual data in heap, just pointer to data)
    // however this causes an issue where if s1 and s2 go out of scope, the same memory is freed twice (double free error)
    // to ensure memory safety, s1 becomes invalid after it is assigned to s2

    // println!("{}", s1); // value moved error
    println!("{}", s2);

    // deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);



    let s = String::from("hello");

    takes_ownership(s); // value of s moves into function - s no longer valid here

    let x = 5;

    makes_copy(x); // x moves into function but can still use afterwards since i32 is Copy

    // println!("Value of s: {}", s); // move occurs because String does not implement the Copy trait
    println!("Value of x: {}", x);

    let s1 = String::from("test");
    let s2 = takes_and_gives_ownership(s1);

    // println!("s1: {}", s1); // value moved to s2, so s1 not valid anymore
    println!("s2: {}", s2);

    // what if we wanted to pass a value to a function but not let the function take ownership of it?
    let len = calculate_length(&s2);
    println!("The length of {} is: {}", s2, len); 

} // memory is automatically returned once variable that owns it goes out of scope (drop function in rust)

fn takes_ownership(some_string: String) {
    println!("This is taking ownership: {}", some_string);
} // some_string goes out of scope and memory freed

fn makes_copy(some_integer: i32) {
    println!("This is making copy: {}", some_integer);
} // some_integer goes out of scope, nothing special happens

// transferring ownership
fn takes_and_gives_ownership(some_string: String) -> String {
    some_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}