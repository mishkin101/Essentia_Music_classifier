

fn main() {
    println!("Hello");
    let mut a: u16 =5; //dont need to add type annotation to variables
    println!("value is, {}", a);
    let x = 5; // x is an integer
    let x = x + 1; // Shadow x with a new integer variable
    {
        let x = x * 2; // Shadow x again within a nested scope
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x in the outer scope is: {}", x); // 6
    //shadowing prevents us from having to rename original variable if we need to chaneg type
    //
}

const PI: f64 =3.14;

//shadowing: declare nw variale with same name as prevoois variable 
// 1st variable is "shadowed by second, so 1st variable is what compiler sees"
//