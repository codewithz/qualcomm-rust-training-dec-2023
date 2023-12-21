use rusqlite::{Connection,Result,Error};
// ORM for Rust -- > diesel,sqlx,rustorm
#[derive(Debug)]
struct Coffee{
    name:String,
    description:String,
    count:i32
}

fn main() -> Result<(),Error> {

    // Create db if doesn't exist 
    let con=Connection::open("coffee.db")?;
    con.execute(
        "CREATE TABLE IF NOT EXISTS coffee(
             id integer primary key,
             name text not null unique,
             description text,
             count integer
        )", [])?;

        let coffee=Coffee{
            name:String::from("Drip2"),
            description:String::from("Nice, dark roast"),
            count:30
        };

        con.execute("INSERT INTO coffee (name,description,count) 
                            values (?1,?2,?3)",
                             [coffee.name,coffee.description,coffee.count.to_string()])?;

        
        let mut stmt=con.prepare("Select name,description,count from coffee")?;

        let coffee_iter=
            stmt.query_map([],|row|{
            Ok(Coffee{
                name:row.get(0)?,
                description:row.get(1)?,
                count:row.get(2)?
            })
        })?;

        for coffee in coffee_iter{
            // println!("Coffee {:?}",coffee);
            match coffee {
             Ok(coffee)=> println!("{:?}",coffee),
             Err(err) => println!("{}",err)
               
            }
        }

                          

    Ok(())
}
