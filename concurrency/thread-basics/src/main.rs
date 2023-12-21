use std::{thread, time::Duration, rc::Rc};


fn main() {
  
//   let handle=  thread::spawn(||{
//        for i in 1..10{
//         println!("Number : {} from a spawned thread",i);
//         thread::sleep(Duration::from_secs(1));
//        }
//     });
//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("Number : {} from main thread!!",i);
//         thread::sleep(Duration::from_secs(1));
//     }

        // let v=vec![1,2,3,4,5];

        // let handle=thread::spawn(move ||{
        //     print!("Here is the value of vector {:?}",v);
        // });

        // // drop(v); Not allowed here

        // handle.join().unwrap();

        // let one=thread::spawn(||{
        //     println!("Logging from Thread 1 !!");
        // });

        // // Use of Thread builder 

        // let two =thread
        //                     ::Builder
        //                     ::new()
        //                     .name("Thread 2".to_string())
        //                     .spawn(||{
        //                         println!("Logging from Thread 2!");
        //                     }).unwrap();
                            
        // let three=thread::spawn(||{
        //     // Capture the underlying THread Object

        //     let thread_two=two.thread();
        //     println!("Thread 2 Name  and ID : {:?}/{:?}",
        //                 thread_two.name(),
        //                 thread_two.id());

        //     two.join().unwrap();
        //     println!("Logging from Thread 3!")
        // });

        // // Join the thread handdles 

        // one.join().unwrap();
        // three.join().unwrap();

        let mut data=Rc::new(  vec![1,2,3]);
        
      

        for i in 0..3{
            // Create a new owned reference 
            let data_ref=data.clone();
            thread::spawn(move ||{
                data_ref[0]+=i;
            });
        }

    

}


// `Rc<Vec<i32>>` cannot be sent between threads safely
// within `{closure@src\main.rs:67:27: 67:34}`, the trait `Send` is not implemented for `Rc<Vec<i32>>`rustcClick for full compiler diagnostic

// Reason : As the error message mentions, Rc cannot be sent between threads safely.
//  This is because the internal reference count is not maintained in a thread-safe manner
//  and can have a data race.