use std::io;
use rand::Rng;
use colored::*;
fn main() {
        home_screen();
        let mut balance: f32 = 150.0;
   loop {
             println!("-------------------------");
             println!("Your Balance is: {}",balance);
             println!("{}","Enter bet amount: ".green());

            let mut bet_amount = String::new();

            io::stdin().read_line(&mut bet_amount).expect("Failed to read line.");

           let bet_amount: f32 = match bet_amount.trim().parse() {
                 Ok(num) => num,
                 Err(_) => { 
                    println!("Invalid Input.");
                    continue; 
                           },
            };
            
            if bet_amount > balance {
                println!("This bet amount is greater than your balance.");
                    continue; 
            }
            else if bet_amount < 5.0{
                println!("Sorry, the minimum bet size is $5.");
                    continue; 
            }
            print!("{esc}c", esc = 27 as char);
             balance = balance + spin_slot(bet_amount);

             if balance == 0.0 {
                println!("Game Over");
                println!("Press enter to exit");

             let mut buffer = String::new();
             io::stdin().read_line(&mut buffer).ok(); // ignore the result
                break;
             }
        
   }
}

fn home_screen() {
    println!("");
    println!("~~~~Three 7's is the jackpot 50x ");
    println!("~~~~Three frogs's is the mini jackpot 10x ");
    println!("~~~~Good Luck! ");
}

fn spin_slot (bet: f32) -> f32{
    let seven = 0;
    let frog = 1;
    let bar = 2;
    let slot_one = rand::thread_rng().gen_range(0,3);
    let slot_two = rand::thread_rng().gen_range(0,3);
    let slot_three = rand::thread_rng().gen_range(0,6);
    let arr = [slot_one,slot_two,slot_three];

    let mut s1 = "poo";
    let mut s2 = "poo";
    let mut s3 = "poo";

// first slotty 
    if slot_one == seven {
     s1 = "7";
    }
    else if slot_one == frog
 {
    s1 = "frog"
 }
 else if slot_one == bar
 {
    s1 = "bar"
 }
// second slotty
 if slot_two == seven {
    s2 = "7";
   }
   else if slot_two == frog
{
   s2 = "frog"
}
else if slot_two == bar
{
   s2 = "bar"
}
// third slotty
if slot_three == seven {
    s3 = "7";
   }
   else if slot_three == frog
{
   s3 = "frog"
}
else if slot_three == bar
{
   s3 = "bar"
}

    println!("");
    println!("[ {} ][ {} ][ {} ]",s1,s2,s3);




        //three sevens
        if arr[0] == seven && arr[1] == seven && arr[2] == seven {
        let winnings = bet*50.0;
        println!(" You hit the JACKPOT!!");
        println!(" Winnings: $ {}", winnings);
        return winnings;
        }
        // two sevens
        else if arr[0] == seven && arr[1] == seven {
        let winnings = bet*5.0;
        println!(" Winnings: $ {}", winnings);
        return winnings;
        }
        //three frogs
        else if arr[0] == frog && arr[1] == frog && arr[2] == frog {
            let winnings = bet*10.0;
            println!(" You hit the MINI JACKPOT!!");
            println!(" Winnings: $ {}", winnings);
            return winnings;
        }
         // two frogs
        else if arr[0] == frog && arr[1] == frog {
            let winnings = bet*2.0;
            println!(" Winnings: $ {}", winnings);
            return winnings;
        }
        //three bars
        else if arr[0] == bar && arr[1] == bar && arr[2] == bar {
            let winnings = bet*5.0;
            println!(" Winnings: $ {}", winnings);
            return winnings;
        }
        // two bars
        else if arr[0] == bar && arr[1] == bar {
            let winnings = bet;
            println!(" Winnings: $ {}", winnings);
            return winnings;
        }
        println!(" you lost: $ {}", bet);
        return -1.0*bet;
}