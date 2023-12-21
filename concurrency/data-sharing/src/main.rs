
use std::{sync::{Mutex, Arc}, thread, rc::Rc};

fn main() {
//    let m= Mutex::new(5);

//    {
//     let mut num=m.lock().unwrap();
//     *num=6;
    
//    }

//    println!("Number Value: {:?}",m)

let counter=Arc::new(Mutex::new(0));

let mut handles=vec![];

for _ in 0..10{
    let counter=Arc::clone(&counter);
    let handle =thread::spawn(move || {
        let mut num=counter.lock().unwrap();

        *num+=1;
    });

handles.push(handle);
}

for handle in handles{
    handle.join().unwrap();
}

println!("Result : {}",*counter.lock().unwrap());

}


// poisoned: false: The poisoned flag indicates
//  whether the Mutex has been poisoned. 
//  In Rust, a Mutex can become "poisoned" 
//  if a thread panics while holding the lock.
//   If this flag is false, it means the Mutex is not poisoned.
