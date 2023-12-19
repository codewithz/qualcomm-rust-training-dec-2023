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

// Slices -- arrays and string 

let str_slice="Hello World";

let my_str=String::from(str_slice);

let slice_from_str= &my_str[0..5];
println!("{}",slice_from_str);

let my_arr=[1,2,3,4,5];
let arr_slice=&my_arr[1..3];
println!("My Array Slice {:?}",arr_slice);

let mut my_mut_arr=[1,2,3,4,5];

let  mut_array_slice=&mut my_mut_arr[0..3];
println!("My Mutable Array Slice before modification {:?}",mut_array_slice);
mut_array_slice[0]=1000;

println!("My Mutable Array Slice after modification {:?}",mut_array_slice);
println!("My Mutable Array  {:?}",my_mut_arr);

let  mut_array_slice1=&mut my_mut_arr[1..4];
mut_array_slice1[0]=43000;
println!("My Mutable Array Slice after modification {:?}",mut_array_slice1);
// If we uncomment below line it will start giving errors
// println!("My Mutable Array Slice after modification {:?}",mut_array_slice);

}

// Macros imported by default
// print and println
// format
// vec
// assert and assert_eq
// panic
// file and line