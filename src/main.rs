use core::{Die, Error, State, Symbol};

use getrandom::getrandom;
use oorandom::{self, Rand32};

mod cli;
mod core;
mod gui;
mod text;
mod three_dimensional;
mod tui;

fn configure_die(die: &mut Die<'_>, range: std::ops::Range<usize>) {
    for i in range {
        let j = i - 1;
        let symbol = Symbol::COLLECTION
            .get(i)
            .expect("Symbol should exist. Otherwise check the range!")
            .clone();
        die.insert_symbol(symbol, j);
    }
}

fn setup_default_dice(state: &mut State) {
    let mut d4 = Die::new("D4".to_string());
    configure_die(&mut d4, 1..5);
    state.add_die(d4);
    let mut d6 = Die::new("D6".to_string());
    configure_die(&mut d6, 1..7);
    state.add_die(d6);
    let mut d10 = Die::new("D10".to_string());
    configure_die(&mut d10, 1..11);
    state.add_die(d10);
    let mut d20 = Die::new("D20".to_string());
    configure_die(&mut d20, 1..21);
    state.add_die(d20);
}

fn setup_random() -> Rand32 {
    let mut bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    getrandom(&mut bytes).expect("Should be able to get random bytes");
    Rand32::new(u64::from_be_bytes(bytes))
}

fn main() -> Result<(), Error> {
    let cli_options = cli::get_cli_options();
    let mut state = State::default();
    setup_default_dice(&mut state);
    let mut random = setup_random();

    match cli_options {
        cli::CliOptions::Exit => {
            return Ok(());
        }
        cli::CliOptions::Text => {
            text::run_text_mode(&mut state, &mut random);
            return Ok(());
        }
        cli::CliOptions::TUI => tui::run_tui(state, random),
        cli::CliOptions::GUI => gui::run_gui(state, random),
        cli::CliOptions::ThreeDimensional => three_dimensional::run_three_dimensional(),
    }
}
