use std::{thread, time::Duration};


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

        let v=vec![1,2,3,4,5];

        let handle=thread::spawn(move ||{
            print!("Here is the value of vector {:?}",v);
        });

        // drop(v); Not allowed here

        handle.join().unwrap();

    

}
