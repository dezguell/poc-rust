use rand::Rng;
use std::io;
use std::cmp::Ordering;

const HINT_STR_VALUE: &str = "-hint";

fn main() {

    const MSG_PREFIX: &str = "The number you guessed was";
    const MSG_AND_IT_IS_A: &str = "and it is a";
    const MSG_WRONG_GUESS: &str = "wrong guess, the number is to";
    const MSG_RIGHT_GUESS: &str = "right guess, you win!";
    const MSG_YOU_LOSS: &str = "you loss";       

    println!("This is the Guess The Number Game.");
    println!();    
    
    let number_to_guess = rand::thread_rng().gen_range(1..101);    
    //println!("The number to guess is {}", number_to_guess);   

    
    loop{        
        println!("Please enter the number between 1 and 101!.");
        println!("You can type the command -hint after your guess, like (4 -hint), to get a useful hint");
        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Fail to read the number");           
        
        let mut guess_int: u32 = 0;

        if check_hint_request(&guess_str){
            guess_str = guess_str
                .replace(&HINT_STR_VALUE,"")
                .trim()
                .to_string();
                        
            guess_int = convert_str_to_int(&guess_str);
            
            if guess_int.eq(&u32::MIN){
                continue;
            }  

            println!("-------- Hint ----------");
            let _hint_result = number_to_guess - guess_int;
            println!("The number to guess minus the number you entered is equal: {}", _hint_result);
        }
        
        if guess_int.eq(&0){
            guess_int = convert_str_to_int(&guess_str); 
        }
        
        if guess_int.eq(&u32::MIN){
            continue;
        }  
        
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

fn check_hint_request( _text: &std::string::String) -> bool{   
    
    if _text.contains(&HINT_STR_VALUE){
        return true;
    }
    
    false
}

fn convert_str_to_int(_text: &std::string::String) -> u32 {
    match _text.trim().parse()
            {
                Ok(num) => num,
                Err(_) => u32::MIN,
            }    
}



