use std::collections::HashSet;
use std::collections::BTreeSet;

// Hash, Eq, PartialEq needed to use this struct as the custom key to a HashSet
#[derive(Debug, PartialEq, Hash, Eq)]
struct Coffee {
    id: i32,
    count: i32 
}

// Custom Ord, PartialOrd needed for the BTreeSet to sort by count
impl Ord for Coffee {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Coffee {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        Some(self.count.cmp(&other.count))
    }
}



fn main() {
    // Note: There is a duplicated element here that isn't included
	let mut coffee_set = HashSet::from([
		Coffee{id: 1000, count: 10},
		Coffee{id: 3000, count: 500},
		Coffee{id: 2000, count: 40},
		Coffee{id: 3000, count: 500}
	]);

	println!("Initial coffee set: {:?}\n", coffee_set);

	// HashSet element access and insertion
	coffee_set.insert(Coffee{id: 4000, count: 1});

	// Fetching a value from a set
	println!("Coffee4: {:?}\n", coffee_set.get(&Coffee{id: 4000, count: 1}));

	// Iterating over all values in a HashSet
	for coffee in &coffee_set {
		println!("{coffee:?}");
	}

	// Most useful HashSet functions
	let set_a = HashSet::from([
		Coffee{id: 1000, count: 10},
		Coffee{id: 2000, count: 20},
		Coffee{id: 3000, count: 30},
		Coffee{id: 4000, count: 40}
	]);

	let set_b = HashSet::from([
		Coffee{id: 1000, count: 10},
		Coffee{id: 9999, count: 99},
		Coffee{id: 8888, count: 88},
		Coffee{id: 7777, count: 77}
	]);

	let difference: HashSet<&Coffee> = set_a.difference(&set_b).collect();
	println!("Difference: {:?}\n", difference);

	let intersection: HashSet<&Coffee> = set_a.intersection(&set_b).collect();
	println!("Intersection: {:?}\n", intersection);

	let set_is_disjoint: bool = set_a.is_disjoint(&set_b);
	println!("Disjoint?: {:?}\n", set_is_disjoint);

	let symmetric_diff: HashSet<&Coffee> = set_a.symmetric_difference(&set_b).collect();
	println!("Symmetric Difference: {:?}\n", symmetric_diff);

	let all_values: HashSet<&Coffee> = set_a.union(&set_b).collect();
	println!("Union: {:?}\n", all_values);
		

	// /**
	//  * BTreeSet
	//  */

	let mut coffee_tree_set = BTreeSet::from([
		Coffee{id: 3000, count: 1},
		Coffee{id: 2000, count: 5},
		Coffee{id: 1000, count: 2}
	]);

	println!("Unsorted HashSet: {:?}\n", coffee_set);
	println!("BTreeSet sorting based on count: {:?}\n", coffee_tree_set);

	// Note: BTreeSet has very similar methods to HashMap and HashSet
	// Remember, sorting is of great value if you are using a BTreeSet

	println!("First value: {:?}\n", coffee_tree_set.first());
	println!("Last value: {:?}\n", coffee_tree_set.last());

	coffee_tree_set.pop_first();
	coffee_tree_set.pop_last();

	println!("After removal: {:?}\n", coffee_tree_set);

	coffee_tree_set.insert(Coffee{id: 6000, count: 50});
	coffee_tree_set.insert(Coffee{id: 7000, count: 0});

	println!("After insertion: {:?}\n", coffee_tree_set);
}
