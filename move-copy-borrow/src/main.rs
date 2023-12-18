#[derive(Debug,Copy,Clone)]
struct Coffee{
    id:i32,
    count:i32
}


fn main() {

// Performance: Copy is often more performant because it 
// involves a simple bitwise copy. Clone may require additional 
// computation, especially for types with heap-allocated data.

// Semantic Meaning: Use Copy when duplicating a value doesn't 
// involve any special meaning or additional cost. 
// Use Clone when duplication requires custom logic or 
// involves heap-allocated data.

// Automatically Derived: Many simple types automatically 
// derive Copy (e.g., integers, booleans), 
// while Clone typically needs to be implemented manually.

//     // Move 

// //Move semantics in Rust refer to the
// // transfer of ownership from one variable to another.
// //When a value is moved from one variable to another,
// //the original variable can no longer access the value. 
// //Ownership is transferred, and the new owner is responsible
// // for cleaning up the memory
// //when the variable goes out of scope.

//     let original_value=String::from("Hello");

//     let moved_value=original_value; //Ownership is transferred (Moved)
//     // The next line would cause a compilation error, as original_value is no longer valid
//     // println!("{}",original_value)
//     println!("{}",moved_value);

//     // Borrowed Values -- allows a reference to be temporarily taken to a value without
//     // transferring.
//     // Borowing can either be mutable or immutable .

//     let original=String::from("Hi");

//     // Immutable Borrow 
//     let borrowed_value= &original;

//     println!("Original Value : {}",original);
//   //  println!("Borrowed Value : {}",borrowed_value);

//     // Multiple references can borrow the same value simultaneously
//     // but multiple borrowing prevents other borrows for the duration of the mutable borrow 

//     // Mutable Borrow -- prevent othe borrows for it's duration 
//     let mut mutable_value=original;
//     let borrowed_mut_value=&mut mutable_value;
    
//     print!("{}",mutable_value);
//     // print!("{}",borrowed_mut_value);
//    // println!("Borrowed Value : {}",borrowed_value);

//    //Copy -- Stack Based objects

//    let x=5;
//    let y=x; //Copying and not moving 

// //    Both x and y are valid and hold same value 

// println!("x : {} || y : {}",x,y);


// ------------------------------Second Example ------------------------------------


// Moves/Copies primitives 

let a=1;
let b=a;

// Copy by Value 

// primitive 
println!("a: {}",a);
println!("b: {}",b);

let string_a=String::from("Hello");
let string_b=string_a;
// Copy by reference
// Because of Moving , string_a no longer have ownership
// println!("a: {}",string_a);
println!("b: {}",string_b);

let string_c=String::from("Hello");
let string_d=string_c.clone();

println!("c: {}",string_c);
println!("d: {}",string_d);

{
    let greeting =String::from("Hello Rust");
    println!("Greeting from inside the scope : {}",greeting)
}
    // println!("Greeting from outside the scope : {}",greeting)

let coffee_a=Coffee{id:1,count:40};
let coffee_b=coffee_a;

println!("Coffee b {:?}: ",coffee_b);
println!("Coffee a {:?}: ",coffee_a);


let coffee_c=Coffee{id:3,count:30};
let coffee_d=coffee_c.clone();

println!("Coffee c {:?}: ",coffee_c);
println!("Coffee d {:?}: ",coffee_d);





}
