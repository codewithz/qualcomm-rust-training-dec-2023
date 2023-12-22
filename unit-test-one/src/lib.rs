// While writing Tests 
// 1. Mock the data or state needed by test case
// 2. Run the code that needs to be tested.
// 3. Check if the actual behaviour of the code and expected is fine using asserts
// Can also check if Unit  codes are creating panic!


pub fn adder(x:i32,y:i32) -> i32{
    x-y
}

#[cfg(test)]
mod tests{

    // This brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn it_adds(){
        let result=adder(3, 5);
        let expected=8;

        assert_eq!(result,expected);
    }
}