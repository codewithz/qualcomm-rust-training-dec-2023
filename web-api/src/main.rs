use warp::{http,Filter};
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

// Common Function 

fn json_body()-> impl Filter<Extract = (Item,),Error = warp::Rejection> + Clone {
    // When accepting the body, we want JSON Body
    // (to reject huge payload)

    warp::body::content_length_limit(1024*16).and(warp::body::json())
}


// FOr posting in List 

async fn add_grocery_list_item(
    item:Item,
    store:Store
) -> Result<impl warp::Reply,warp::Rejection>{
    let r=store.grocery_list.read();
    Ok(warp::reply::json(&*r))
}

#[tokio::main]
async fn main() {

    // // GET  /hello/warp ==> 200 OK 
    // let hello=warp
    //                         ::path!("hello" / String)
    //                         .map(|name| format!("Hello, {}",name));

    let store=Store::new();
    let store_filter=warp
                                            ::any()
                                            .map(move || store.clone());


    let add_items=warp::post()
                                                               .and(warp::path("v1"))
                                                               .and(warp::path("groceries"))
                                                               .and(warp::path::end())
                                                               .and(json_body())
                                                               .and(store_filter.clone())
                                                               .and_then(add_grocery_list_item);


    warp::serve(add_items)
        .run(([127,0,0,1],3030))
        .await;
}
