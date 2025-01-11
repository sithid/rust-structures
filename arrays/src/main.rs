fn main() {
    let fruits: [&str; 5] = ["apple", "banana", "orange", "pear", "mango"];
    let str_slice= &fruits[0..3];
    let mut string_fruit = String::from("");

    for fruit in fruits{
        string_fruit.push_str(" ");
        string_fruit.push_str(fruit);
    }

    string_fruit = String::from(string_fruit.trim_start());

    println!("\n\r**************************************************ARRAYS**************************************************");
    println!("* Welcome to Rust Arrays!                                                                                *");
    println!("* Array: A contiguous block of memory that holds a fixed number of elements of the same data type.       *");
    println!("* Contiguous: Elements are stored one after another in memory without any gaps.                          *");
    println!("* Constraints: Arrays are of a fixed size and contain only elements of the same data type.               *");
    println!("* String Slice: An array of immutable string references (&str).                                          *");
    println!("*************************************************EXAMPLES*************************************************\n\r");
    println!("string array: let fruits: [&str, 5] = [{:?}]", fruits);
    println!("slice of fruits: let str_slice= &fruits[0..3] = {:?}", str_slice);
    println!("string from &str array: string_fruit = {}, ", string_fruit);
    
    println!("\n\rlet mut numbers; [i32; 10] = [0;10]");

    let numbers: [i32; 16] = [1; 16];

    for( i, value ) in numbers.iter().enumerate() {
        println!("numbers: Index: {}, Value: {}, bit shifted: {}", i, value, value << i);
    }

}
