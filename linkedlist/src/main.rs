/*
 * Linked List:  I will actually be moving this code to its own little project.
 * Option<..> can be used for Some(value) or None (like in the event the node does not have a next because it is the last node in the list).
 * Box<..> is a smart pointer for data allocation on the heap.  Because Node is recursive ( reference to a Node struct: head).
 */

 #[derive(Debug)]
 struct Node {
     data: i32,
     next: Option<Box<Node>>, // Option<> for Some(value) or None, Box<> for a smart pointer.
                              // Node is recursive, have to use Box<>. 
 }
 
 #[derive(Debug)]
 struct LinkedList {
     head: Option<Box<Node>> // First (root) node.
 }
 
 impl LinkedList {
     fn new() -> Self {             // LinkedList Constructor: Self refers to the type were implementing, LinkedList.
         LinkedList { head: None }
     }
 
     fn insert_at_head(&mut self, data: i32 ) {
         let new_node = Box::new(Node { 
             data: data,
             next: self.head.take(),
         });
 
         self.head = Some(new_node);
     }
 }
 
 fn main() {
    println!("**********************************************Linked List*************************************************");
    println!("* Welcome to Rust Linked List!                                                                           *");
    println!("* Linked List:                                                                                           *");
    println!("************************************************EXAMPLES**************************************************");
    println!("* Linked List:                                                                                           *");
    println!("*   let mut list: LinkedList = LinkedList::new()                                                         *");
    println!("*   list.insert_at_head( 10 )                                                                            *");

    let mut list: LinkedList = LinkedList::new();
    list.insert_at_head( 10 );

    println!("*   println!(\"LinkedList.head = {{:?}}\" -> LinkedList.head = {:?}           *", list.head);

    let mut n = list.head.unwrap();

    println!("*   list.head: Data: {}, Next: {:?}                                                                      *", n.data, n.next );
    println!("************************************************EXAMPLES**************************************************\n\r");
 }
 