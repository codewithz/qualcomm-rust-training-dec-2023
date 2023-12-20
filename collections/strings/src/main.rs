fn main() {
    	// Literal, primitive string versus a String
	let primitive_greeting: &str = "Hello World";
	let mut greeting: String = String::from(primitive_greeting);
	println!("\n{}\n", primitive_greeting);
	println!("{}\n", greeting);

	// Common String and primitive string (&str) operations

	// Accessing characters by index

	// Don't do this!
	// println!("First char in greeting: {}\n", greeting[0]);

	let first_char = greeting.as_str().chars().nth(0);

	println!("First char in greeting: {:?}\n", first_char);

	let str_bytes = greeting.as_bytes();

	// Now you can index by integer if you want...
	println!("First char in greeting (bytes): {:?}\n", str_bytes[0]);

	// Adding to a String
	greeting.push_str(", my name is Zartab");

	println!("{}\n", greeting);

	// Removing a character from a String
	greeting.remove(0);
	
	println!("{}\n", greeting);

	// Treating a String like a stack
	greeting.pop();
	println!("{}\n", greeting);

	greeting.push('b'); // Note the single quotes for a char
	println!("{}\n", greeting);

	// Inserting characters by index
	greeting.insert(0, 'H');
	println!("{}\n", greeting);

	// Inserting a string primitive by index
	greeting.insert_str(0, "Well, ");
	println!("{}\n", greeting);

	// Performing find and replace on a String
	let substr = "Hello World";
	let hello_world_start = greeting.find("Hello World").unwrap_or(greeting.len());
	let hello_world_end = hello_world_start + substr.len();

	greeting.replace_range(hello_world_start..hello_world_end, "hello world");
	println!("{}\n", greeting);



	// Lowercase/Uppercase
	greeting.make_ascii_lowercase();
	println!("Lowercase greeting: {}\n", greeting);

	greeting.make_ascii_uppercase();
	println!("Uppercase greeting: {}\n", greeting);

	// string to integer
	let maybe_number = "5000".parse::<u32>();
	println!("Number: {:?}\n", maybe_number);

	let maybe_eleven = "eleven".parse::<u32>();
	println!("Error: {:?}\n", maybe_eleven);

	// Trimming a string
	let str_with_spaces = "   Hello World        ";
	println!("Trimmed str: {}\n", str_with_spaces.trim());
	println!("Trimmed end str: {}\n", str_with_spaces.trim_end());
	println!("Trimmed start str: {}\n", str_with_spaces.trim_start());

	// Matching
	println!("Greeting: {}\n", greeting);
	println!("Does greeting start with 'Hello': {}\n", greeting.starts_with("Hello"));

	// Removing characters by match
	println!("Greeting: {}\n", greeting);
	println!("Trimmed end by match: {}\n", greeting.trim_end_matches("Zartab"));

	let repeating = "11011";
	println!("Trimmed end by match: {}\n", repeating.trim_end_matches("1"));

	println!("Trimmed start by match: {}\n", repeating.trim_start_matches("1"));

	// Going from a String to a Vec
	// Splitting by character
	println!("Split greeting: {:?}", greeting.split(',').collect::<Vec<&str>>());

	// Splitting by str
	println!("Split greeting: {:?}", greeting.split(", HELLO WORLD,").collect::<Vec<&str>>());
    println!("Greetings : {}",greeting);
    let greeting="Hello How are you";
	// Splitting by function
	println!("Split greeting: {:?}", greeting.split(char::is_uppercase).collect::<Vec<&str>>());

	// Splitting at a specific index
	println!("{:?}\n", greeting.split_at(4));
}
