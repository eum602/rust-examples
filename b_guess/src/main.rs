use std::io; //accept, amongst others, user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();//let allows to create a variable, "mut" ==> MUTABLE variable, otherwise by default is immutable
    //"::" indicates new is a method from string

    io::stdin().read_line(&mut guess)//passing what is read to a mutable reference
        .expect("Failed to read line");//handling error by taking the result

    println!("You guessed: {}", guess);//{} is a placeholder
}
