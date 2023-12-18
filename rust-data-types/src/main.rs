

#[derive(Debug)]
struct  Coffee{
    id:i32,
    name:String,
    temperature:TemperatureCategory
    
}

#[derive(Debug)]
enum TemperatureCategory {
    
    HOT(Option<f64>),
    ICED(Option<f64>)
}

fn main() {
    //Integers 
    let my_num:i32=32;
    println!("Integer : {}",my_num);

    // Converting Strings to Integer 

    let parsed_num:i32="123".parse().unwrap();
    println!("Parsed Number: {}",parsed_num);
    println!("Integer to String: {}",parsed_num.to_string());

    // Floats

    let my_float :f32= 10.4;

    println!("Floor: {}",my_float.floor());
    println!("Ceiling: {}",my_float.ceil());
    println!("Rounded: {}",my_float.round());

    // Working with floats and integers 

    let my_int=my_float as i32 + 1;
    let my_new_float=1 as f32 +my_float;
    println!("Coercing float to int : {}", my_int);
    println!("Coercing int to float : {}", my_new_float);

    // Characters 

    let my_char='A';

    println!("Is Uppercase : {}", my_char.is_uppercase());
    println!("Is Lowercase : {}", my_char.is_lowercase());
    println!("Lowercase : {}", my_char.to_ascii_lowercase());
    println!("String : {}", my_char.to_string());

    // Booleans 

    let my_bool=true;
    assert_eq!(my_bool,true);

    // Tuples 

    let my_tuple=('A',5,10.5);
    println!("Char / Integer / Float: {} / {} / {}",my_tuple.0,my_tuple.1,my_tuple.2);


    // Destructuring 
    let numbers_tuple=(1,2,3,4);

    let (one,two,three,four)=numbers_tuple;

    let (_,_,three_test,_)=numbers_tuple;

    println!("One/Two/Three/Four:{}/{}/{}/{}",one,two,three,four);

    println!("Three Test:{}",three_test);

    // Nested Tuples 

    let nested_tuple=((1,2),(3,4));
    let ((x1,y1),(x2,y2))=nested_tuple;
    println!("x1:{},y1:{} || x2:{}, y2{}",x1,y1,x2,y2);

    // Arrays 
     
     let my_array=[1,2,3,4,5];

     for num in my_array{
        println!("Number : {}",num);
     }

     let [one,two,..]=my_array;


     let same_value_arr: [i32;1000]= [10;1000];
    //  let same_value_arr: This declares a variable named same_value_arr.
    // : [i32; 1000]: This specifies the type of the variable as an array of 1000 elements, where each element is of type i32.
    // = [10; 1000]: This initializes the array with the value 10 repeated 1000 times.
    // So, after this line of code, same_value_arr is an array with 1000 elements, and each element has the value 10.

     println!("Array : {:?}",same_value_arr);
     println!("First Element : {}",same_value_arr[0]);
     println!("Length : {}",same_value_arr.len());
     println!("Array Size : {}",std::mem::size_of_val(&same_value_arr));

     //Structs 

     let mut my_coffee=Coffee{
        id:10,
        name:String::from("Latte"),
        temperature: TemperatureCategory::HOT(None)
     };

     my_coffee.id=1000;

     let id=10;

     let coffee_with_temperature=Coffee{
        // Field init shorthand 
        id,
        name:String::from("Cappucino"),
        temperature:TemperatureCategory::HOT(Some(103.2))

     };
    println!("My Coffee with Temp {:?}",coffee_with_temperature);
    //  Combine Structs 

    let combined=Coffee{
        id:2000,
        ..coffee_with_temperature 
    };

     println!("My Coffee {:?}",my_coffee);
    
     println!("My Coffee Combined {:?}",combined);
     println!("Struct Size : {}",std::mem::size_of_val(&my_coffee));

}
