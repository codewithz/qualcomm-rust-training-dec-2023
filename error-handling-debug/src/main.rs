use std::{fs::{OpenOptions,File},io::Result};

#[derive(Debug)]
struct Coffee{
    id:i32,
    count:i32
}

#[derive(Debug,PartialEq,Clone)]
struct  MyCustomError {
    message:String
}

impl MyCustomError{
    fn new(message :&str) ->MyCustomError{
        MyCustomError{message:message.to_string()}
    }
}

fn open_file(path: &str)->Result<File>{
    OpenOptions::new().read(true).open(path)
}

fn open_file_chain(one: &str,two: &str) -> Result<File>{
    open_file(one)?;
    open_file(two)
}




fn main() {

    let coffees=Vec::from(
        [
            Coffee{id:1000,count:10},
            Coffee{id:2000,count:20},
            Coffee{id:3000,count:30},
        ]
        
    );

    println!("Vector of Coffees {:?}",coffees);
    // Access an invalid index 

    let maybe_coffee=coffees.get(4);

    let result= match maybe_coffee {
        Some(coffee) =>Ok(coffee),
        None => Err(MyCustomError::new("Coffee does not exist!!"))
    };

    match  result {
        Ok(coffee)=>println!("Coffee {:?}",coffee),
        Err(err) =>println!("Error : {}",err.message)
    }


    let open_file_result=open_file("file1.txt");
    match open_file_result {
        Ok(file)=> println!("File {:?}",file),
        Err(err)=> println!("Error {:?}",err)
        // Err(err)=> panic!("{:?}",err)
    };
    let open_file_chain_result =open_file_chain("file.txt", "log.txt");

    match open_file_chain_result {
        Ok(last_file) => println!("{:?}",last_file),
        Err(err)=> println!("Error {:?}",err)
    }
    
}

// Panic Hook 
// use std::panic;

// fn custom_panic_hook(info: &panic::PanicInfo) {
//     // Perform cleanup or logging here
//     println!("Panic occurred: {:?}", info);
// }

// fn main() {
//     // Set a custom panic hook
//     panic::set_hook(Box::new(custom_panic_hook));

//     // Code that might panic
//     panic!("This is a panic!");

//     // The program will terminate after the panic, and the custom_panic_hook will be called.
// }
