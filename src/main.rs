use std::thread::sleep;
use std::time::Duration;

use clap::*;
use cardpack::prelude::*;
use inquire::{CustomType, Select};
use crossterm::terminal::{Clear,ClearType};
use crossterm::execute;
use rand::seq::IndexedRandom;

#[derive(Parser)]
struct Cli{
    #[arg(short,long)]
    bust:Option<i32>,
    #[arg(short,long)]
    starter_money:Option<u64>,
    #[arg(short,long)]
    daily_games:Option<u8>,
    #[arg(short,long)]
    profit:Option<f32>,
    #[arg(short,long)]
    days:Option<u8>
}

fn main(){
    clear();
    let instance = Cli::parse();
    let bust = instance.bust.unwrap_or(21);
    let starter_money = instance.starter_money.unwrap_or(50);
    let daily_games = instance.daily_games.unwrap_or(3);
    let profit = instance.profit.unwrap_or(0.20);
    let days = instance.days.unwrap_or(3);
    let mut money = starter_money.clone();

    match Select::new("Do you want/need a tutorial", vec!["Yes","No"]).prompt(){
        Ok(choice)=>{
            match choice{
                "Yes"=>{
                    println!("\n\n\n");
                    println!("Each day, for {}, you will start off with {}.", format!("{days} days").purple(), format!("{starter_money} clams").blue());
                    println!("You will be given {} to make a {} on your money, otherwise you {} :O"
                        , format!("{daily_games} games").red()
                        , format!("{}% Gain",profit*100.0).underline()
                        , "DIE".red().bold());

                    println!("This is typical blackjack,except that the ace is counted as a 1, and the bust is at {bust}. GLHF\n\n[ Press Enter to Continue ]");
                    halt();
                    clear();
                },
                "No"=>{}
                _=>{
                    issue();
                }
            }
        },
        Err(_)=>{}
    }
    for day in 1..(days+1){
        clear();
        let day_start = money;
        let quota = day_start as f32*1.0+(100.0 * profit);
        for g in 1..(daily_games+1){
            if money == 0{
                println!("You are too poor to bet. You lose");
                return
            }
            println!("Day {day} | Game {g}\n\n");
            println!("Quota of {quota}");
            match Select::new("Would you like to play a game of Blackjack",vec!["Yes","No"]).prompt(){
                Ok(choice)=>{
                    println!("{}",format!("{}",choice).italic());
                    match choice{
                        "Yes"=>{game(bust,&mut money)},
                        "No"=>{sleep(Duration::from_secs(1));},
                        _=>{}
                    }
                }
                Err(_)=>{
                    issue()
                }
            }
        }
        if money as f32 >= day_start as f32*1.0+(100.0 * profit){
            println!("You met the quota! You survive..")
        }
        else{
            println!("You missed the quota. you lose.");
            return
        }
        halt();
        sleep(Duration::from_secs(1));
    }
    println!("You survived the {days} days, congratulations!")
}


fn halt(){
    let mut void: String = String::new();
    std::io::stdin().read_line(&mut void).ok();
}

fn game(bust:i32,money:&mut u64){
    clear();
    let mut bet:u64 = 0;
    loop{
        let prompt: u64 = CustomType::new(format!("Please select a bet, you have {}",*money).as_str()).prompt().unwrap();
        if prompt <= *money{
            *&mut bet = prompt;
            break
        }else{
            println!("You are unable to afford this bet, try again.")
        }
    }
    let mut deck: Standard52Deck = Pile::deck().shuffled();
    let mut hand: Vec<Standard52Card> = Vec::new();
    let dealer_value: &i32 = [17,18,19,20].choose(&mut rand::rng()).unwrap();
    let mut player_value: i32;
    loop{
        clear();
        println!("Dealer has a hand of {dealer_value}");
        player_value = 0;
        hand.push(deck.draw_first().unwrap());
        for c in hand.clone(){
            println!("{}",c.fluent_name_default());
            player_value+=get_card_value(&c);
        }
        println!("Your hand is worth: {}",player_value);
        if player_value > bust{
            *money-=bet;
            println!("You busted at {player_value} \nYou lost a bet of {} and now only have {} left",bet,*money);
            return;
        }
        match Select::new("Stand or Hit", vec!["Stand","Hit"]).prompt().unwrap(){
            "Hit"=>{}
            "Stand"=>{
                break
            }
            _=>{}
        }
    }
    if &player_value >= dealer_value{
        println!("You win! You gained {bet} clams!" );
        *money += bet;
        println!("You now have {}",*money)
    }else{
        println!("You lost! You lost {bet} clams");
        *money -= bet;
        println!("You now have {}",*money)
    }
}

fn clear(){
    execute!(std::io::stdout(),Clear(ClearType::All)).ok();
}

fn issue(){
    panic!("This, logically shouldn't be possible, [except for if you used CONTROL + C to exit my program >:(] but if you managed to reach here, good job? I guess.
don't bother reporting this, as I am not emotionally attached to this code in the slightest, and unless\n
this code somehow possesses a zero-day exploit, I won't exactly be inclined to fix this issue.")
}

fn get_card_value(card:&Card<Standard52>) -> i32{
    match card.base().index()[0..1].to_string().as_str(){
        "A"=>{
            1
        }
        "K"=>{
            10
        }
        "Q"=>{
            10
        }
        "J"=>{
            10
        }
        "T"=>{
            10
        }
        n=>{
            n.parse().unwrap()
        }
    }
}
