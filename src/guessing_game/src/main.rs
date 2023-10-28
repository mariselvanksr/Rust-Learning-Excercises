use std::io;
use std::cmp::Ordering;
use rand::Rng;

/**
    ***Notes:***
    After updating the rand = "0.8.5" in dependencies I got the below error for "cargo build"
    "Blocking waiting for file lock on the registry index"
    Solved by removing the cache stored in the cargo system
    cmd: rm -rf ~/.cargo/registry/index/\* ~/.cargo/.package-cache
    `source: <https://stackoverflow.com/questions/47565203/cargo-build-hangs-with-blocking-waiting-for-file-lock-on-the-registry-index-a>`
**/

fn main() {
    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                
                let mut continue_play = String::new();
                
                println!("\n\n If you want to continue play please input Y (YES). Press any key for exit.");
                
                io::stdin().read_line(&mut continue_play).expect("Please enter Y (YES) or N (NO).");

                let continue_play: String = continue_play.trim().to_lowercase();

                if  continue_play != "y" && continue_play != "yes" {
                   println!("Thanks for playing with me :)");

                   break;
                }
            }
        }   
    }
}
