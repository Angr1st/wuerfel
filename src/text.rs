use std::io::BufRead;

use crate::{Die, State};

fn get_chosen_die<'a>(state: &'a mut State<'a>) -> &'a Die<'a> {
    //After showing the dice. Ask user to select a die by inputting name of the die
    println!("Please enter the name of the die you want to use.");
    let mut user_input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_line(&mut user_input)
        .expect("Failed reading from stdin!");
    loop {
        let trimmed = user_input.trim();
        for die in state.get_dice().iter() {
            let name = die.get_name();
            if name == trimmed {
                println!("Found die: {}", name);
                return die;
            }
        }
        println!("Your input matched with none of the existing dice. Please try again!");
        user_input.clear();
        handle
            .read_line(&mut user_input)
            .expect("Failed reading from stdin!");
    }
}

fn roll_again() -> bool {
    println!("Do you want to roll again?(y/n)");

    let mut user_input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_line(&mut user_input)
        .expect("Failed reading from stdin!");
    loop {
        let trimmed = user_input.trim();

        if trimmed == "y" || trimmed == "Y" {
            return true;
        } else if trimmed == "n" || trimmed == "N" {
            return false;
        } else {
            println!("Your input matched with none of the existing dice. Please try again!");
            user_input.clear();
            handle
                .read_line(&mut user_input)
                .expect("Failed reading from stdin!");
        }
    }
}

fn get_chosen_die_range<'a>(state: &'a mut State<'a>) -> std::ops::Range<u32> {
    let chosen_die = get_chosen_die(state);
    println!("You selected: {}", chosen_die);
    println!("Throwing the die!");

    chosen_die.get_range()
}

pub fn run_text_mode<'a>(state: &'a mut State<'a>, random: &mut oorandom::Rand32) {
    let available_dice = state.print_dice().unwrap_or(String::from("None"));
    println!("Currently available dice: {}", available_dice);

    let die_range = get_chosen_die_range(state);
    loop {
        let random_number = random.rand_range(die_range.clone());
        println!("You rolled a: {}", random_number);
        if !roll_again() {
            break;
        }
    }
}
