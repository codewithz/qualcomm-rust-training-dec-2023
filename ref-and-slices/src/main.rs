#[derive(Debug)]
struct Coffee{
    id:i32,
    count:i32
}

fn main() {


    let mut  a =1;

    // This value will be stored : Stack 

    fn increase ( mut input: i32){
        input+=20;
        println!("Input Parameter after increse :{}",input);
    }

    increase(a);
    println!("a after increse :{}",a);

    //Since the value type is passed via mutable reference -- we can alter it 

    fn increase_by_reference(input : &mut i32){
        *input +=20;
    }

    increase_by_reference(&mut a);
     println!("a after increase by ref :{}",a);

     fn alter_coffee(coffee: &mut Coffee,increase: i32){
        coffee.count+=increase;
     }

     fn print_coffee(coffee: &Coffee){
        println!("My Coffee with id: {} and count: {}",coffee.id,coffee.count);
     }

     let mut my_coffee=Coffee{id:10,count:50};
     alter_coffee(&mut my_coffee, 25);
     print_coffee(&my_coffee);
}

// Macros imported by default
// print and println
// format
// vec
// assert and assert_eq
// panic
// file and line