#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

#[derive(Debug)]
struct TuplePerson( String, );

fn main() {
    println!("*************************************************STRUCTS**************************************************");
    println!("* Welcome to Rust Structs!                                                                               *");
    println!("* Struct: user-defined composite data types.  Allows you to group together related data under one name.  *");
    println!("* Structs are similar to classes from oop languages but without inheritance.                             *");
    println!("* Structs are like blueprints to creating custom data types.                                             *");
    println!("*                                                                                                        *");
    println!("* There are 3 types of structs:                                                                          *");
    println!("*   Struct: C-Like Struct                                                                                *");
    println!("*   Tuple Struct: Tuple                                                                                  *");
    println!("*   Unit Struct: Field-less Struct                                                                       *");
    println!("*                                                                                                        *");
    println!("* Struct:                                                                                                *");
    println!("*   Resemble structs written in the c language. You define a struct using the struct keyword             *");
    println!("* followed by the name of the struct, preferably in CamelCase,  followed by curly braces {{}}.             *");
    println!("* Field definitions are contained within the curly braces {{}}. C style structs can also implment          *");
    println!("* methods using the impl keyword. Methods are functions associated with structs.                         *");
    println!("*                                                                                                        *");
    println!("* C Struct Definition:                                                                                   *");
    println!("*   struct Struct_Name {{                                                                                 *");
    println!("*       name: String,                                                                                    *");
    println!("*   }}                                                                                                    *");
    println!("*                                                                                                        *");
    println!("*   impl Struct_Name {{                                                                                   *");
    println!("*       fn new(name: String) -> Struct_Name {{                                                            *");
    println!("*           Struct_Name {{                                                                                *");
    println!("*               name,                                                                                    *");
    println!("*           }}                                                                                            *");
    println!("*       }}                                                                                                *");
    println!("*   }}                                                                                                    *");
    println!("*                                                                                                        *");
    println!("* Tuple Structs:                                                                                         *");
    println!("*   Tuple structs are similar to structs but their fields do not have names. Useful for grouping data    *");
    println!("* together when named fields are not neeeded. A tuple struct is not the same thing as a tuple. Both      *");
    println!("* tuples and tuple structs can be destructured to extract values and assign them to named varibles.      *");
    println!("*                                                                                                        *");
    println!("* Tuple Struct Definition:                                                                               *");
    println!("*   struct Color( i32, i32, i32 );      : Represents an RGB color.                                       *");
    println!("*   struct Point2D( i32, i32 );         : Represents a point on a 2D plane.                              *");
    println!("*   struct Point32( i32, i32, i32 );    : Represents a point on a 3D plane.                              *");
    println!("*   struct TuplePerson( String, ) : Tuple struct with a single field, common required.                   *");
    println!("*                                                                                                        *");
    println!("* Unit Structs:                                                                                          *");
    println!("*   Unit structs are structs with no fields.  You define them with an empty pair of parentheses or       *");
    println!("* just a semicolon.  Useful as markers or when you need a basic type with no data.                       *");
    println!("*                                                                                                        *");
    println!("* Unit Struct Definition:                                                                                *");
    println!("*   struct Connected();                                                                                  *");
    println!("*   struct Disconnected;                                                                                 *");
    println!("**********************************************************************************************************\n\r");

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

    println!("C style structs:\n\r");
    println!("Person1:");
    println!("  -> Person1| Person1 {:?}", person1 );
    println!("  -> Person1| Person1 Name: {}, Age: {}, Is Student: {}", person1.name, person1.age, person1.is_student );
    
    println!("");

    println!("Person2:");
    println!("  -> Person2| Person2 {:?}", person2 );
    println!("  -> Person2| Person2 Name: {}, Age: {}, Is Student: {}", person2.name, person2.age, person2.is_student );
    
    println!("");
    println!("  Note: Person2 doesn't like to be called jimmy, append \"my\" to the end of 'Jim' to construct the name 'Jimmy'");
    println!("    -> person2.name.push_str(\"my\");");
    println!("");

    person2.name.push_str("my");

    println!("  -> Person2| Person2 {:?}", person2 );
    println!("  -> Person2| Person2 Name: {}, Age: {}, Is Student: {}\n\r", person2.name, person2.age, person2.is_student );

    println!("Tuple structs:\n\r");

    let manager: TuplePerson = TuplePerson(String::from("Jimmy"));

    println!("manager:");
    println!("  -> manager| let manager: TuplePerson = TuplePerson(String::from(\"Jimmy\"));");
    println!("  -> manager| manager {:?}", manager);
    println!("  -> manager| manager.0 {:?}", manager.0);
    println!("  -> manager| Destructuring...");
    println!("  -> manager| let TuplePerson( name ) = manager;");
    
    let TuplePerson( name,) = manager;
    
    println!("  -> manager -> name| name {:?}", name);
    println!("  -> manager -> name| name {}", name);

    println!("");
}