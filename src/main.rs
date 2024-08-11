use std::io;
use rand::Rng;

const THROWS: u32 = 1000;

fn random_throw() -> (u32, u32) {
    let mut rng = rand::thread_rng();
    let mut dice1 = rng.gen_range(1..=6); 
    let mut dice2 = rng.gen_range(1..=6); 

    if dice2 > dice1 {
        std::mem::swap(&mut dice1, &mut dice2);
    }

    (dice1, dice2)
}

fn check_dice(attacker_dice: u32, defender_dice: u32) -> bool {
    attacker_dice > defender_dice
}

fn check_throw(dice1: u32, dice2: u32) -> bool {
    let (mut one_dice, mut two_dice) = (0, 0);

    for _i in 0..THROWS {
        let (defender_dice1, defender_dice2) = random_throw();
        let defender_loss = check_dice(dice1, defender_dice1) as u32
                         + check_dice(dice2, defender_dice2) as u32;

        if defender_loss == 2 {
            one_dice += 1;
        } else {
            two_dice += 1;
        }
    }

    one_dice <= two_dice
}

fn read_dice_input(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(num) if (1..=6).contains(&num) => return num,
            _ => println!("Please enter a number between 1 and 6."),
        }
    }
}

fn main() {
    let dice1 = read_dice_input("Enter the first dice value: ");
    let dice2 = read_dice_input("Enter the second dice value: ");

    let (dice1, dice2) = if dice1 >= dice2 {
        (dice1, dice2)
    } else {
        (dice2, dice1)
    };

    if check_throw(dice1, dice2) {
        println!("You should throw two dice");
    } else {
        println!("You should throw one die");
    }
}
