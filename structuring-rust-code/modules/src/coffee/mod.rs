// Top Level Coffee module for WBC(Wired Brained Coffee) -- declares the espresso 
// submodule

pub mod espresso;

#[derive(Debug)]
pub struct Coffee{
    pub name:String,
    pub cost:f64,
    pub count:i32
}