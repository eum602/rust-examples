
fn main() {
    constants_variables();
    shadowing();
}

fn constants_variables(){
    println!("********************Constants and variables********************");
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

fn shadowing()  {
    println!("********************Shadowing********************");
    let m = 4;
    let m = m +6;
    println!("By using 'let' keyword we are allowed to create a new variable with the same name of a previous one:
    For example the value of m=4 is now a new variable with the value: {}",m);
    let m = "Some string";
    println!("The value that originally was an int is now a new variable of type string thanks to 
    shadowing: 'let m='Some string' ==> m is ==> {}",m);
    println!("Another important thing to know is that if we declare 'let mut x = 5' and then we try
    to MUTATE THE TYPE like let x = 'hello' then it will throw an error.");
}