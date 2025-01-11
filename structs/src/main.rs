/*
 * Linked List:  I will actually be moving this code to its own little project.
 * Option<..> can be used for Some(value) or None (like in the event the node does not have a next because it is the last node in the list).
 * Box<..> is a smart pointer for data allocation on the heap.  Because Node is recursive ( reference to a Node struct: head)
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

fn main() {
    println!("*************************************************STRUCTS**************************************************");
    println!("* Welcome to Rust Structs!                                                                               *");
    println!("* Struct:                                                                                                *");
    println!("************************************************EXAMPLES**************************************************\n\r");

}
