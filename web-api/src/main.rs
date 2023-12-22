use warp::Filter;
use parking_lot::{RwLock};
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize,Deserialize};
// type keyword is used to create type aliases, allowing you to give a new name to an existing type. 
// Type aliases can improve code readability and make it more expressive.
type Items=HashMap<String,i32>;

#[derive(Debug,Serialize,Deserialize,Clone)]
struct  Item {
    name:String,
    quantity:i32
}

#[derive(Clone)]
struct Store{
    grocery_list:Arc<RwLock<Items>>
}

impl Store{
    fn new() -> Self{
        Store{
            grocery_list:Arc::new(RwLock::new(HashMap::new()))
        }
    }
}

#[tokio::main]
async fn main() {

    // GET  /hello/warp ==> 200 OK 
    let hello=warp
                            ::path!("hello" / String)
                            .map(|name| format!("Hello, {}",name));

    warp::serve(hello)
        .run(([127,0,0,1],3030))
        .await;
}
