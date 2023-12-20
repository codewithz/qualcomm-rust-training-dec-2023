
// Enum  -- TemperatureCategory -- HOT, ICED
// Enum  -- Roast -- DARK,MEDIUM,LIGHT

// Structs -- Coffee, Espresso, Tea, Beer

// Trait  -- Brew


#[derive(Debug,Clone,Copy)]
enum TemperatureCategory {
    HOT,
    ICED
}

#[derive(Debug,Clone,Copy)]
enum Roast {
    DARK,
    MEDIUM,
    LIGHT
}

// Trait 

trait  Brew {
    fn brew(&self)->(){
        println!("Brewing.....")
    }
}

#[derive(Debug,Clone)]
struct  Coffee{
    name:String,
    temperature:TemperatureCategory,
    roast:Roast
}

impl Brew for Coffee{
    fn brew(&self)->() {
        println!("Brewing {:?} , {:?} roast coffee named {}",
                    self.temperature,
                    self.roast,
                    self.name
                );
    }
}

struct  Tea{
    temperature:TemperatureCategory,
    origin:String,
    brand:String,
    rating:i32
}

impl Brew for Tea {
    fn brew(&self)->() {
        println!("Brewing {}, {:?} tea from {} with rating {}",
                self.brand,
                self.temperature,
                self.origin,
                self.rating
                );
    }
}

struct  Beer {}

impl Brew for Beer {}
fn main() {

    let coffee=Coffee{
        name:String::from("Drip"),
        temperature:TemperatureCategory::HOT,
        roast:Roast::DARK
    };

    coffee.brew();

    let tea=Tea{
        brand:String::from("Harney and Sons"),
        origin:String::from("UK"),
        temperature:TemperatureCategory::HOT,
        rating:95
    };

    tea.brew();

    let beer=Beer{};
    beer.brew();

    // This function takes any ref to an 
    // item that implements Brew trait
    // Polymorphism in Rust
    fn brew_drink(drink: &impl Brew){
        println!("---------------\n 
        I am inside a generic Brew FUnction
         \n---------------------- ");
        drink.brew();
    }

    brew_drink(&coffee);
    brew_drink(&tea);
    brew_drink(&beer);

   
}
