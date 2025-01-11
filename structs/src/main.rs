#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

fn main() {
    println!("*************************************************STRUCTS**************************************************");
    println!("* Welcome to Rust Structs!                                                                               *");
    println!("* Struct: user-defined composite data types.  Allows you to group together related data under one name.  *");
    println!("* Structs are similar to classes from oop languages but without inheritance.                             *");
    println!("* Structs are like blueprints to creating custom data types.                                             *");
    println!("* Struct Definition: struct Struct_Name {{ field: type, name: String, age: i32, etc: etc }}                *");
    println!("************************************************EXAMPLES**************************************************\n\r");

    let person1 = Person {
        name: String::from("Jimmy"),
        age: 38,
        is_student: false,
    };

    let mut person2 = Person {
        name: String::from("Jim"),
        age: 38,
        is_student: true,
    };

    println!("Person1| {:?}", person1 );
    println!("Person1| Name: {}, Age: {}, Is Student: {}", person1.name, person1.age, person1.is_student );
    
    println!("");

    println!("Person2| {:?}", person2 );
    println!("Person2| Name: {}, Age: {}, Is Student: {}", person2.name, person2.age, person2.is_student );

    println!("");
    println!("Person2 says: don't call me Jim, call me Jimmy!");
    println!("");

    person2.name.push_str("my");

    println!("Person2| {:?}", person2 );
    println!("Person2| Name: {}, Age: {}, Is Student: {}", person2.name, person2.age, person2.is_student );
}
