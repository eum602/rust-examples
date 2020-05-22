
fn main() {
    constants_variables();
    shadowing();
    data_types();
    data_types_scalars_integers();
    data_types_floating_point();
    operations();
    boolexample();
    characterexample()
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

fn data_types(){
    let some: u32 = "43".parse().expect("The value to parse is not of type u32");
    println!("Even when rust can infer what type of values can be passed, sometimes it is needed to
    explicitly pass the type of value. for example we pass '43' and the we parse it as u32 the the value
    is and u32 ==> {}",some);

    println!("There are two types of data in rust: 
    * Scalar types
    * Compound types")

}

fn data_types_scalars_integers () {
    println!("A single value
    * Integers have defined sizes
    * Integers are signed or unsigned:
        * Signed Integers:
            int32 -2^16->2^16-1
        * Unsigned integers:
            uint32 0->2^32-1
    
    ** Integer literals:
        * 67_225_567
        * 0xaf
        * 0b10101_1111
        * b'A'

    **Overflow: Lets say you declare u8 but 
    at runtime the variable gets 257 ==> the program exits panicking.
    In --release the Two's complement wrapping occurs. so 256 becomes zero; 257 becomes 1 and so on.
    To customize wrapping use WRAPPING LIBRARY.    
    ")
}

fn data_types_floating_point() {
    println!("
    floating point data types are:
    * 32bits ==> let l: f32 = 5.0 ==> l is f32 
    *64 bits ==> default ==> let k = 35.0 ==> by default k is f64
    ")    
}

fn operations() {
    println!("
    
    // addition
    let sum = 5 + 10;
    

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
    ")

}

fn boolexample()  {
    println!("
    Bools:
    let t: bool = false;
    ")    
}
fn characterexample() {
    println!("**Chars**
    Chars are four bytes in size.
    Unicode scalar value ==> Any kind like japanese, korean, etc.
    ");

   let c = 'z';
   let z = 'Ƶ';
   let hearth = '❤';
   println!("{}",c);
   println!("{}",z);
   println!("{}",hearth)
}