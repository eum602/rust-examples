use std::io; //accept, amongst others, user input
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);//random from 1 to 100, defaults type to int32
    println!("The secret number is: {}", secret_number);

    loop {
        println!("
        Please input your guess.");

        let mut guess = String::new();//let allows to create a variable, "mut" ==> MUTABLE variable, otherwise by default is immutable
        //"::" indicates new is a method from string.Rust assumes 'guess' is a string ==> type inference

        io::stdin().read_line(&mut guess)//passing what is read to a mutable reference
            .expect("Failed to read line");//handling error by taking the result

        let guess: u32 = match guess.trim().parse(){//returns an enum with Ok or Err
            //ARMS PATTERNS ==> according to a case do something
            Ok(num) => num,
            Err(_) => continue, //if error only pass to the next in the loop; '_' means to match all error values.
        }; //SHADOWING the previous value of 'guess' ==> converts from one to another type.
        //trim method ==> eliminates spaces at the beginning and at the end of guess. eg. 5\n ==> \n because of pressing enter.
        //parse method ==> converts string to the specified number type (in this case to u32)

        println!("You guessed: {}", guess);//{} is a placeholder

        match guess.cmp(&secret_number){
            //ARMS PATTERNS ==> according to a case do something
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break; //breaking the loop
            }
        }        
    }    
}
