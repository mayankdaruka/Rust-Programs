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
    let s2 = s1; // copies over pointer, len, capacity of s1 to s2 (not actual data in heap, just pointer to data)
    // however this causes an issue where if s1 and s2 go out of scope, the same memory is freed twice (double free error)
    // to ensure memory safety, s1 becomes invalid after it is assigned to s2

    // println!("{}", s1); // value moved error
    println!("{}", s2);


} // memory is automatically returned once variable that owns it goes out of scope (drop function in rust)