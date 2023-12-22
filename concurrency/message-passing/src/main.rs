
use std::sync::mpsc;
use std::thread;

// mpsc -- Multiple Producer Single Consumer 

fn main() {
    
    let (tx,rx)=mpsc::channel();

    thread::spawn(move || 
    {
        let message=String::from("Hi");
        tx.send(message).unwrap();
        // The line below will not work becuase the ownership of message is transferred to tx
        // println!("Message sent was : {}",message);
    }
    );

    let received= rx.recv().unwrap();
    println!("Got: {}",received);

   
}
