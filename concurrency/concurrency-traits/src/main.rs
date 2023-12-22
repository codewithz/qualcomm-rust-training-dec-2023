use std::{sync::mpsc, thread, time::Duration};

// The Coffee struct implicitly implements Send because i32 and String type is Send
// The Coffee struct implicitly implements Sync because i32 and String type is Sync
#[derive(Debug)]
struct Coffee{
    id:i32,
    count:i32,
    name:String
}

fn main() {
//    Message Passing between Threads 

    let (tx,rx)=mpsc::channel();

    // Not capturing the thread handle , it caused these threads to kick off 
    // -- main thread doesn't have to wait for them to finish
    thread::spawn(move || 
    {
        for id in 0..20{
            let coffee=Coffee {
                id:id+1,
                count:50,
                name:String::from("Drip")
                };
        // We can send coffees here because each field within Coffee is Send
        tx.send(coffee).unwrap();
        thread::sleep(Duration::from_millis(500));
        }
    });


    let receiver_thread=thread::spawn(move || 
    {
        for i in 0..20 {
            let coffee=rx.recv().unwrap();
            println!("{:?}",coffee);
            thread::sleep(Duration::from_millis(750));
        }
    });

    receiver_thread.join().unwrap();



}
