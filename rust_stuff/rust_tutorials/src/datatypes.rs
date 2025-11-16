
fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("signd integer: {}", x);
    println!("unsigned integer: {}", y);


    let pi: f64 = 3.14;
    println!("float: {}", pi);

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("numbers: {:?}", numbers);
    //arrays cannot have mixed datatypes

    let fruits: [&str; 3]  = ["a", "c", "x"];      //string reference and size of array
    //"Alice" is a string slice, not a string.
    let tp: (String,i32, bool) = ("Alice".to_string(), 30, false);  
    println!("Human Tuple: {:?}", tp);

    let mixed_tp = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My mixed tuple: {:?}", mixed_tp );

    // Slices: contigous sequence of elements, allocated next to one another in memory
    //here we are declaring an int array as a slice
    let number_slices:&[i32] = &[1,2,4,5,5];
    println!("My num slices: {:?}", number_slices );

    let animal_slices :&[&str] = &["Lion", "ele"];
    println!("animal slices: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"Lion".to_string(), &"ele".to_string()];
    println!("animal slices: {:?}", animal_slices);

    //this is stored on heap memory
    let stone_cold: String = String::from("Hell, ");
    println!("stone cold: {:?}", stone_cold);

    // since all rust variables are immutable, we can use mut to make them muttable
    let mut stone_cold: String = String::from("Hell, ");
    println!("stone cold: {:?}", stone_cold);
    stone_cold.push_str("Yeah!");


    let  string: String = String::from("Hello, world ");
    let slice: &str = &string[0..5]; //reference to the string
    println!("slice value: {}", slice);


}


//String allocated on HEAP [growable, mutable, owned string type]. dynamic, at runtime
//Owned: not borrowed

//string slice is stored on the stack (stored reference, &str)
//reference string without copying or owning data
//specific size and known number of bytes on the stack

//by default rust data types are immutable

//rust cleans memory allocated to any variable