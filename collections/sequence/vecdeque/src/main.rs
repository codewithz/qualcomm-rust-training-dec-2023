use std::collections::VecDeque;


#[derive(Debug, PartialEq)]
struct Coffee {
    id: i32,
    name: String
}


fn main() {
	// VecDeque use as a queue
	let mut coffee_queue = VecDeque::from([
		Coffee{id: 1000, name: String::from("Coffee1")},
		Coffee{id: 2000, name: String::from("Coffee2")},
		Coffee{id: 3000, name: String::from("Coffee3")}
	]);

	coffee_queue.push_back(Coffee{id: 4000, name: String::from("Coffee4")});

	println!("End of queue: {:?}\n", coffee_queue.back());

	coffee_queue.pop_back();

	println!("End of queue: {:?}\n", coffee_queue.back());

	coffee_queue.push_front(Coffee{id: 0000, name: String::from("Coffee0")});

	println!("Front of queue: {:?}\n", coffee_queue.front());

	coffee_queue.pop_front();

	println!("Front of queue: {:?}\n", coffee_queue.front());

	// Get the raw values out of the VecDeque as a pair of slices
	// Two slices are returned since this represents the underlying structure
	// of the VecDequeue as a ring buffer

	println!("Coffee queue as slices: {:?}\n", coffee_queue.as_slices());

	// Example of two slices: values are not necessarily contiguous in memory with a VecDeque
	coffee_queue.push_front(Coffee{id: 0000, name: String::from("Coffee0")});
	println!("Coffee queue as slices: {:?}\n", coffee_queue.as_slices());

	// Force your queue to be contiguous in memory
	coffee_queue.make_contiguous();
	println!("Coffee queue as slices: {:?}\n", coffee_queue.as_slices());

}



