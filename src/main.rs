use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    const MSG_PREFIX: &str = "The number you guessed was";
    const MSG_AND_IT_IS_A: &str = "and it is a";
    const MSG_WRONG_GUESS: &str = "wrong guess, the number is to";
    const MSG_RIGHT_GUESS: &str = "right guess, you win!";
    const MSG_YOU_LOSS: &str = "you loss";

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
        
        match guess_int.cmp(&number_to_guess){
            Ordering::Less => get_loss_by_less(&guess_str),
            Ordering::Greater => get_loss_by_greater(&guess_str),            
            Ordering::Equal => {
                get_win_msg(&guess_str);
                break;
            }
        }
        println!();
    }

    fn get_loss_by_less(guess_str:&str){
        println!("{} {} {} {} small, {}!", MSG_PREFIX, guess_str, MSG_AND_IT_IS_A, MSG_WRONG_GUESS, MSG_YOU_LOSS)
    }

    fn get_loss_by_greater(guess_str:&str){
        println!("{} {} {} {} hight, {}!", MSG_PREFIX, guess_str, MSG_AND_IT_IS_A, MSG_WRONG_GUESS, MSG_YOU_LOSS)
    }
    
    fn get_win_msg(guess_str:&str){
        println!("{} {} {}", MSG_PREFIX, guess_str, MSG_RIGHT_GUESS)
    }    
}




