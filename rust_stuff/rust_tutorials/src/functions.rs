

//functions

fn fun(){
    println!("Hello!")
}


fn main(){
    println!("Hello rust!");
    tell_height(182.0, 54, "joel");
    // any expression will evaluare to last line in the expression, so we don't need the semi colon. can also do return a *b
    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    let y: i32 =add(4,6);
    println!("Value of y is: {}" ,y);

    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your bMI is {:.2}", bmi); //formatting with 2 decimal points

    //borrowing s1 reference 
    let s1 = String::from("RUST");
    let len = calculate_length(&s1); //borrowing s1 reference 
    println!("length of '{}' is {}.", s1, len); //rust accessed s1 string's data without taking ownership of it

    //one owenr of a time
    let s3 = String::from("Rust");
    let s4 =s3;

    println!("{}", s4); // we cannot s3 do this because s4 variable now owns the string
    // println!("{}", s3)

}
//main funciton needs to be entry point 
// function or variables in snake_case: hello_world
//hoising: function can be above or below the code
  
fn hello_world(){
    println!("hello world!")
}

fn add(a: i32, b: i32) -> i32{
    a+b
}

fn tell_height(height: f32, age: u32, name:&str){
    println!("My heigh is: {} cm, age: {}, name: {}", height, age, name)
}
// any variable outside of the keyword should be declared with the const key word
//BMI = height(kg)/height^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg/(height_m)
}
// expression
// 5
// add(3,4)
// if condition {value1} else {value2}
//almost all statements end with semi colon and do not return a value
//let x = let y = 10
//variable declaration: let x =5; (statement, no retur value)


//Ownership
//stopping, resuming the program
//solve safety and high performance issues
// every variable has one value, and only this particular variable ownes this value

//In borrowing, we can temporarily borrow references to values

//Rules:
// every values has only 1 owner
//only one owner can own  a value at a time
//when the owner is out of scope, the value is dropping


fn calculate_length(s: &String) -> usize{
    s.len()
}
