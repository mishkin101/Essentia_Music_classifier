//references 
//only 1 owner

//borrow value without taking ownership via references. can be mutable and immutable
fn main(){
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x; //need to add mut to reference and owner. wihtout it, its immutale 
    *_r +=1;

    //can only have 1 mutable reference, or many immutable references
    // println!("{}", _x);
    println!("{}", _r);
}


