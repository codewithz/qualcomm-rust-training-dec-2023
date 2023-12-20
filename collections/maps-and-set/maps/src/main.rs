// Hash, Eq, PartialEq are required to use this struct as a custom key to a Hashmap

use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Debug,PartialEq, Eq,Hash)]
struct Coffee {
    id:i32,
    count:i32
}

// Custom Ord and PartialOrd for BTree to sort by count 

impl Ord for Coffee {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
    
}

impl PartialOrd for Coffee{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.count.cmp(&other.count))
    }
}


fn generate_coffee()->Coffee {
    Coffee{
        id:10000,
        count:50
    }
}

fn main() {
    let mut coffee_map = HashMap::from([
		("Coffee1", Coffee{id: 1000, count: 10}),
		("Coffee2", Coffee{id: 2000, count: 40}),
		("Coffee3", Coffee{id: 3000, count: 500})
	]);

	// HashMap element access and insertion
	coffee_map.insert("Coffee4", Coffee{id: 4000, count: 1});

	println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));

	coffee_map.insert("Coffee4", Coffee{id: 5000, count: 99999});

	println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));

		
	// Entry API
	// Only insert if the key doesn't already exist...
	coffee_map.entry("Coffee4").or_insert(Coffee{id: 5000, count: 1});
	println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));

	// Do the same thing but using a custom function
	coffee_map.remove("Coffee4");
	coffee_map.entry("Coffee4").or_insert_with(generate_coffee);
	println!("Coffee4: {:?}\n", coffee_map.get("Coffee4"));



	// Using a custom type as a key

	let mut custom_key_map = HashMap::from([
		(Coffee{id: 1000, count: 5}, "Coffee1"),
		(Coffee{id: 2000, count: 2}, "Coffee2"),
		(Coffee{id: 3000, count: 1}, "Coffee3"),
	]);

	println!("Coffee1: {:?}\n", custom_key_map.get(&Coffee{id: 1000, count: 5}));
	println!("Coffee1: {:?}\n", custom_key_map.get(&Coffee{id: 1000, count: 0}));



	// Iterating over the key/value pairs of a HashMap
	for (coffee, name) in &custom_key_map {
		println!("{coffee:?} / {name}\n");
	}

	// Filtering a HashMap
	custom_key_map.retain(|coffee, _name| coffee.count < 5);
	println!("After filtering: {:?}\n", custom_key_map);

	// BTreeMap

	let mut coffee_descriptions = BTreeMap::from([
		(Coffee{id: 3000, count: 1}, "Medium, fruity"),
		(Coffee{id: 2000, count: 2}, "Ethiopian blend"),
		(Coffee{id: 1000, count: 5}, "Bold, rich flavor"),
	]);
	println!("BTreeMap sorting: {:?}\n", coffee_descriptions);

	// Note: BTreeMap has very similar methods to HashMap
	// Remember, sorting is of great value if you are using a BTreeMap

	println!("First pair: {:?}\n", coffee_descriptions.first_key_value());
	println!("Last pair: {:?}\n", coffee_descriptions.last_key_value());

	coffee_descriptions.pop_first();
	coffee_descriptions.pop_last();

	println!("After removal: {:?}\n", coffee_descriptions);

	coffee_descriptions.insert(Coffee{id: 6000, count: 50}, "Dark, nutty");

	println!("After insertion: {:?}\n", coffee_descriptions);
}
