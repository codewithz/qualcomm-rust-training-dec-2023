#[allow(unused_doc_comments)]

use std::collections::LinkedList;


#[derive(Debug, PartialEq)]
struct Coffee {
    id: i32,
    name: String
}


fn main() {

	/**
	  From the Rust docs:
		"NOTE: It is almost always better to use Vec or VecDeque because array-based
		containers are generally faster, more memory efficient, 
		and make better use of CPU cache."
	 */

	let mut coffee_list = LinkedList::from([
		Coffee{id: 1000, name: String::from("Coffee1")},
		Coffee{id: 2000, name: String::from("Coffee2")},
		Coffee{id: 3000, name: String::from("Coffee3")}
	]);

	coffee_list.push_back(Coffee{id: 4000, name: String::from("Coffee4")});

	println!("End of linked list: {:?}\n", coffee_list.back());

	coffee_list.pop_back();

	println!("End of linked list: {:?}\n", coffee_list.back());

	coffee_list.push_front(Coffee{id: 0000, name: String::from("Coffee0")});

	println!("Front of linked list: {:?}\n", coffee_list.front());

	coffee_list.pop_front();

	println!("Front of linked list: {:?}\n", coffee_list.front());

	for coffee in &coffee_list {
		println!("Coffee: {:?}\n", coffee);
	}

	let mut split_coffees = coffee_list.split_off(coffee_list.len() - 1);
	println!("Original list after split: {:?}\n", coffee_list);
	println!("Split off list: {:?}\n", split_coffees);

	split_coffees.append(&mut coffee_list);
	println!("Joined list: {:?}\n", split_coffees);
	println!("Old list: {:?}\n", coffee_list);
	
}
