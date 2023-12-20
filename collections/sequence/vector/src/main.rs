#[derive(Debug, PartialEq)]
struct Coffee {
    id: i32,
    name: String
}


fn main() {
	// Set up a vector of coffee structs that we can test with
	let mut coffees = Vec::new();

	for n in 1..5 {
		let coffee = Coffee {
			id: 1000 * n,
			name: String::from("Coffee - ".to_owned() + &n.to_string())
		};
		coffees.push(coffee);
	}

	println!("{:?}\n", coffees);

	// Element access and insertion
	println!("First Coffee: {:?}\n", coffees[0]); // Direct index access
	// println!("Seventh Coffee: {:?}\n", coffees[8]); // Panic!

	// Prefer this method of element access!
	println!("Second Coffee: {:?}\n", coffees.get(1));
	println!("Seventh Coffee: {:?}\n", coffees.get(8)); // No panic here!

	// When it comes to adding/removing elements, a Vec functionally behaves like a Stack!
	coffees.pop(); // Remove a value
	println!("Coffee Vec after removal: {:?}\n", coffees);

	// Alternatively, you can replace elements or insert elements at specific indexes
	let removed_coffee = coffees.remove(0);
	println!("Coffee Vec after removal at index 0: {:?}\n", coffees);

	coffees.insert(0, removed_coffee);
	println!("Coffee Vec after insertion at index 0: {:?}\n", coffees);


	// Immutable iteration
	for coffee in &coffees {
		println!("Coffee: {}\n", coffee.name);
	}

	// Mutable iteration
	for coffee in &mut coffees {
		coffee.id += 1000;
		println!("Coffee ID: {}\n", coffee.id);
	}

	// Iterate with index
	for (idx, coffee) in coffees.iter().enumerate() {
        println!("{}: {:?}\n", idx, coffee);
	}

	// Most useful utility functions 

	// Checking to see if a Vector contains a given element 
	println!(
		"Are we including this coffee? {}\n",
		coffees.contains(&Coffee{id: 2000, name: String::from("Coffee - 1")})
	);

	// Check the length of a Vec
	println!("How many coffees? {}\n", coffees.len());


	// Filtering a Vec 
	coffees.retain(|coffee| coffee.id > 2000);
	println!("Coffees: {:?}\n", coffees);

	// Combining two Vecs
	let mut more_coffees = vec!(Coffee{id: 9999, name: String::from("Best Coffee")});
	coffees.append(&mut more_coffees);
	println!("Coffees: {:?}\n", coffees);

    let mut one =vec![1,2,3];
    let two=vec![4,5];
    println!("Before Extend: {:?}",one);
    one.extend(two.iter().cloned());

    println!("After Extend: {:?}",one);
    println!("Two : {:?}",two);


	// Note that "append" alters both vectors -> It removes and adds
	println!("More Coffees Vec: {:?}\n", more_coffees);


	// Remove all values from a Vec
	coffees.clear();
	println!("How many coffees? {}\n", coffees.len());
}

