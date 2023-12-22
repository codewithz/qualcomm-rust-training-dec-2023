// While writing Tests 
// 1. Mock the data or state needed by test case
// 2. Run the code that needs to be tested.
// 3. Check if the actual behaviour of the code and expected is fine using asserts
// Can also check if Unit  codes are creating panic!


pub fn adder(x:i32,y:i32) -> i32{
    x-y
}

pub fn single_digit_adder(x:i8,y:i8)-> i8{
    fn is_single_digit(x:i8) -> bool{
        x<10 && x>-10
    }

    if!(is_single_digit(x) && !is_single_digit(y)){
        panic!("Only single digits are allowed")
    }
    else{
        x+y
    }
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

    #[test]
    // #[should_panic]
    fn it_should_only_accept_single_digits(){
        single_digit_adder(4, 1);
    }
}