
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// mpsc -- Multiple Producer Single Consumer 

fn main() {
    
    let (tx,rx)=mpsc::channel();

    let tx2=tx.clone();

    thread::spawn(move || 
    {
       let values=vec![
        String::from("Hi"),
        String::from("From"),
        String::from("The"),
        String::from("Thread"),
       ];

       for value in values{
        tx.send(value).unwrap();
        thread::sleep(Duration::from_secs(1));
       }
    }
    );

    thread::spawn(move || 
    {
       let values=vec![
        String::from("More"),
        String::from("Messages"),
        String::from("For"),
        String::from("You"),
       ];

       for value in values{
        tx2.send(value).unwrap();
        thread::sleep(Duration::from_secs(1));
       }
    }
    );

    for received in rx{
        println!("Got {}",received);
    }

   
}
