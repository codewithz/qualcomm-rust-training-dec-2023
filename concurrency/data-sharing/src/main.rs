
use std::sync::Mutex;

fn main() {
   let m= Mutex::new(5);

   {
    let mut num=m.lock().unwrap();
    *num=6;
    
   }

   println!("Number Value: {:?}",m)

}


// poisoned: false: The poisoned flag indicates
//  whether the Mutex has been poisoned. 
//  In Rust, a Mutex can become "poisoned" 
//  if a thread panics while holding the lock.
//   If this flag is false, it means the Mutex is not poisoned.
