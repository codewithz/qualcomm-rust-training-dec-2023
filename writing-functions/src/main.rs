#[derive(Debug)]
struct  Coffee{
    id:i32,
    count:i32
}


impl Coffee{
    // Methods 

    fn increase_count(&mut self,amount:i32){
        self.count+=amount;
        self.print();
    }

    fn print(&self){
        println!("Increasing Count .. Coffee {} has a count {}",self.id,self.count);
    }

    // Associated Functions 

    fn new (id:i32,count:i32) ->Coffee{
        Coffee{
            id,count
        }
    }
}



fn do_something(){
    println!("Doing SOmething")
}

fn main() {
    
    
    // Named Functions 

    fn add(a:f32,b:f32)->f32{
        a+b
    }

    println!("Addition Result : {} ",add(3.1,2.3));

    do_something();


// CLosures 

let print_text = ||println!("Print the text");

print_text();

let add_one = |x:i32| x+1;

let added_one=add_one(33);

println!("After adding 1: {}",added_one);

// Multi Line Rust 
let squared_sum = |x:i32,y:i32| {

    //find the sum of two parameters 
    let mut sum=x+y;

    let mut result=sum*sum;

    return result;
};

let result=squared_sum(1,4);
println!("Squared sum is : {}",result);

// Using Closures as Higher order functions 


let add_ten = | x:i32 | x+10;

let numbers:Vec<i32>= (1..10).map(add_ten).collect();

println!("Numbers added with 10 : {:?}",numbers);

//  Write a closure for filter all divisbles of 3 from a range of (1..30)

let divide_by_3=|number:i32|number%3==0;

let divisible_by_3:Vec<i32>= (1..30)
                            .into_iter()
                            .filter(|number|number%3==0)
                            .collect();

println!("Numbers divisble by 3: {:?}",divisible_by_3);

// Scopes and Closures 

let  outer_var =1 ;

let my_closure= |x:i32| x*10+outer_var;
// If you uncomment the next line and make the outer_var mutable, it will not compile
// Reason -- outer_var is now owned by closure
// outer_var+=1;
println!("{}",outer_var);

println!("Closure with captured env value : {}",my_closure(5));
println!("Closure with captured env value : {}",my_closure(6));

// Methods and Associated Functions 

let mut my_coffee=Coffee{id:1000,count:5};
my_coffee.increase_count(10);

let mut my_other_coffee=Coffee::new(1200, 30);
my_other_coffee.increase_count(20);

}
