use std::collections::HashMap;

fn main() {
    // Vectors 
    // Initialize 

    let mut prime_numbers=vec![2,3,4,5];

    let mut even_numbers=Vec::from([2,4,6,8]);

    let odd_numbers:Vec<i32>=Vec::with_capacity(10);
    println!("Capacity of odd numbers: {:?}\n", odd_numbers.capacity());

    prime_numbers.push(11);
    println!("{:?}",prime_numbers);

    even_numbers.pop();
    println!("{:?}",even_numbers);

    even_numbers.insert(2, 12);
    println!("{:?}",even_numbers);

    // HashMap 

    let mut my_string_map=HashMap::new();
    my_string_map.insert(1, "Hello");
    my_string_map.insert(2, "World");
    println!("String Map {:?}\n", my_string_map);


    let  my_coffee_map=HashMap::from([
        ("Drip",2.99),
        ("Espresso",4.50),
    ]);
    println!("My coffee map: {:?}\n", my_coffee_map);

    let init_capacity_map:HashMap<i32, &str>=HashMap::with_capacity(10);
    println!("My capacity map: {:?}\n", init_capacity_map.capacity());

    // The capacity you provide to with_capacity is a hint,
    //  and the actual capacity may be adjusted internally 
    //  by the hashmap implementation for optimization purposes. 
    //  The hashmap implementation often chooses capacities
    //   that are power-of-two for performance reasons.

    // In the case of your init_capacity_map with a 
    // capacity of 10, the hashmap implementation might 
    // choose a larger capacity, such as 16, to take advantage 
    // of the benefits of power-of-two capacities.

    // String 

    let greetings=String::from("Hello Zartab");
    println!("My Greetings: {}",greetings);

    let mut str_with_capacity=String::with_capacity(5);
    println!("String capacity : {:?}\n", str_with_capacity.capacity());

    for _ in 0..6{
        str_with_capacity.push('Z');
    }
    println!("String capacity : {:?}\n", str_with_capacity);
    println!("String capacity : {:?}\n", str_with_capacity.capacity());

    // /**
	//  * Primitive Collections
	//  * Tuple, Array, Slice
	//  */

	// Tuple
	let my_tuple = ("A", 1, "Hello");
	let (character, int, string) = my_tuple;
	println!("Tuple: {:?}", my_tuple);
	println!("Tuple Character: {}", character);
	println!("Tuple Int: {:?}", int);
	println!("Tuple String: {:?}", string);

	assert_eq!(my_tuple.0, "A");
	assert_eq!(my_tuple.1, 1);
	assert_eq!(my_tuple.2, "Hello");

	// Array
	let mut my_arr: [i32; 4] = [1, 2, 3, 4];
	my_arr[1] = 1;
	println!("Array: {:?}", my_arr);


	// Slice
	let my_slice = &my_arr[1..3];
	println!("Slice: {:?}", my_slice);

	let my_mutable_slice = &mut my_arr[1..3];
	my_mutable_slice[0] = 90;
	println!("Mutable slice: {:?}", my_mutable_slice);








}
