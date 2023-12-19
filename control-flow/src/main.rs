enum Color{
    RED,
    GREEN,
    BLUE
}

enum MessageState{
    PENDING(i32),
    SENDING(i32),
    RECEIVED(i32)
}

fn main() {
   let a=1;
   let b=a+1;

   println!("Expression Evaluation :{}",a + 1 * b);

   if a==1{
    println!("Condition is true");
   }else {
        println!("Condition is false");
   }

   let result=if a+1 == 2 {a} else {b};

   println!("After evaluation {}",result);

   let mut counter=0;

   loop {
       if counter ==3 {
        break;
       };
       counter+=1;

   }
    println!("before entering next loop iteration ... {} : ",counter);
   loop {
       if counter<10{
        counter+=1000;
        println!("Continuing next loop iteration ... {}",counter);
        continue;
       }

       if counter>1000{
        println!("Counter is greater than 1000 -- breaking");
        break;
       }
   }

   println!("I am out of the counter loop..");
//    Nested Loops with Labels 

   'one:loop {
       'two:loop {
           println!("Breaking our of Loop Two");
           break 'two; 
       }
       println!("In Loop One");
       break 'one;
   }

//    Assigning values from a loop 
   let mut start= 0;
   let result = loop {
       start+=1;

       if start==20{
        break start;
       }
    
   };

   println!("loop result: {}",result);

//    While 

let mut next_counter=0;

while next_counter <10 {
    println!("Counter: {}",next_counter);
    next_counter+=1;
}

// While with an array  using an expression 

let my_arr =[1,2,3];

let mut current_index=0;

while let Some(number) = my_arr.get(current_index){
    println!("Index is valid -- fetch number : {}",number);
    current_index+=1;
}

// For Loop 

let my_arr=[1,2,3,4,5];

for number in my_arr{
    println!("Number : {}",number);
}

let mut my_other_array=[6,7,8,9,10];

for number in &mut my_other_array{
    *number+=1;
    println!("Mutated Number : {}",number);

}

println!("Mutated Array = {:?}  ",my_other_array);

// Pattern Matching 

let x=5;

match  x {

    10 => println!("x is 10"),
    5 => println!("x is 5"),
    _ => println!("x is something else")
    
}

let my_color=Color::BLUE;

match  my_color {

    Color::RED => println!("Color is Red"),
    Color::BLUE => println!("Color is Blue"),
    Color::GREEN => println!("Color is Green"),
    
}

let msg_state=MessageState::RECEIVED(3);

match msg_state {
    MessageState::RECEIVED(status_code) =>println!("Message Status: {}",status_code),
    _ => println!("Message is not receieved"),
}

// Matching with Results and Options 

let my_option:Option<i32>= Some(111);

match my_option {
    Some(value) =>println!("The option has a value of  {}",value),
    None => println!("The option has no value")
}

let ok_result: Result<i32, &str> =Ok(100);

match  ok_result {
    Ok(value) => println!("The result is {}",value),
    Err(error) => println!("The error message is  {}",error),
    
}

let err_result: Result<i32, &str> =Err("Something went wrong");

match  err_result {
    Ok(value) => println!("The result is {}",value),
    Err(error) => println!("The error message is  {}",error),
    
}

// use of if let expression -- short hand operator for match

if let Some(value) = my_option{
    println!("Option has some value :{}",value);
    
}else {
    println!("The option has no value");
}


}
