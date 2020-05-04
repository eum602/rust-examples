
fn main() {
    constants_variables();    
}

fn constants_variables(){
    let x = 5; //by default this variable is immutable
    println!("The value of x is: {}", x);
    //x = 6; //reassignment thorws an error unless x was declared as mutable => 'mut'
    const SOME_INT_VALUE:u32 = 5_000;
    println!("const SOME_INT_VALUE:u32 = {} " , SOME_INT_VALUE);
    println!("
    * Constants are like immutables variables, but unlike immutable variables, constants are ALWAYS immutable.
    * The naming convention to declare a constant is: const SOME_INT_VALUE:u32 = 5_000;
    * Constants do not accept the result of a function call that could only be computed at runtime.
    ");    
}