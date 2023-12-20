fn main() {
    let numbers=[2,1,17,99,34,56];

    let numbers_iterator=numbers.iter();

    for number in numbers_iterator {
        println!("{}",number);
    }

    // next() in Rust -- next() is used for traversing through the values of an iterator 

    let colors=vec!["Red","Green","Blue"];

    let mut colors_iterator=colors.iter();
    println!("Colors Iterator = {:?}",colors_iterator);

    // Fetch value using next()
     println!("{:?}",colors_iterator.next());
     println!("{:?}",colors_iterator.next());
     println!("{:?}",colors_iterator);
     println!("{:?}",colors_iterator.next());
     println!("{:?}",colors_iterator);
     println!("{:?}",colors);

    //  iter() -- it borrows each element of the collection.

    // 1. iter() 
    println!("---------------------into_iter---------------------------");
    // 2. into_iter() -- will iteraate on same elements of the collection. 
    // after going through collection it won't be available as the value moves within the loop

    let colors=vec!["Red","Green","Blue"];

    for color in colors.into_iter(){
            println!("{}",color);
    }

    // colors is not available as for loop scope have ended and colors value
    // was moved to the for loop
    // println!("Colors Vector {:?}",colors);

    // 3. Using iter_mut()  - mutably borrow each element of the collection.
    //                        We can modify the collection in place 
    println!("---------------------iter_mut---------------------------");
    let mut colors= ["Red","Green","Blue"];
    println!("Before ={:?}",colors);
    for color in colors.iter_mut(){

        *color="Black";

    }

    println!("After ={:?}",colors);

    // Iterator Adapters 

    let numbers=vec![1,2,3];

    let even_numbers:Vec<i32>=numbers.iter().map(|x|x*2).collect();

    println!("{:?}",even_numbers);

    
}
