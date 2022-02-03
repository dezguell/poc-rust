use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("This is the Guess The Number Game");
    println!();    
    
    let number_to_guess = rand::thread_rng().gen_range(1..101);    
    //println!("The number to guess is {}", number_to_guess);   

    
    loop{        
        println!("Please enter the number!");
        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Fail to read the number");           
            
        let guess_int: u32 = match guess_str.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
            
        
    let  msg_prefix = "The number you guessed was";
    let msg_and_it_is_a= "and it is a";
    let msg_wrong_guess = "wrong guess, the number is to";
    let msg_right_guess = "right guess, you win!";
    let msg_you_loss = "you loss";
        match guess_int.cmp(&number_to_guess){
            Ordering::Less => println!("{} {} {} {} small, {}!", msg_prefix, guess_str, msg_and_it_is_a, msg_wrong_guess, msg_you_loss),
            Ordering::Greater => println!("{} {} {} {} hight, {}!", msg_prefix, guess_str, msg_and_it_is_a, msg_wrong_guess, msg_you_loss),
            Ordering::Equal => {
                println!("{} {} {}", msg_prefix, guess_str, msg_right_guess);
                break;
            }
        }
        println!();
    }
}