use std::io;
use std::{fmt::Display, io::BufRead};

use getrandom::getrandom;
use oorandom::{self, Rand32};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

mod cli;
mod text;

struct State<'a> {
    dice: Vec<Die<'a>>,
}

impl<'a> State<'a> {
    fn add_die(&mut self, dice: Die<'a>) {
        self.dice.push(dice);
    }

    fn print_dice(&self) -> Option<String> {
        if self.dice.len() == 0 {
            return None;
        }
        let mut buffer = String::new();
        const SEPARATOR: &str = ", ";
        for die in self.dice.iter() {
            if !buffer.is_empty() {
                buffer.push_str(SEPARATOR);
            }
            buffer.push_str(&die.name);
        }
        Some(buffer)
    }
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self { dice: vec![] }
    }
}

impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dice.len() == 0 {
            writeln!(f, "No dices configured!")?;
        } else {
            writeln!(f, "Outputting all currently configured dices.")?;
            for dice in self.dice.iter() {
                write!(f, "{}", dice)?;
            }
        }
        std::fmt::Result::Ok(())
    }
}

struct Die<'a> {
    name: String,
    values: Vec<Symbol<'a>>,
}

impl<'a> Die<'a> {
    fn new(name: String) -> Die<'a> {
        Self {
            name,
            values: vec![],
        }
    }

    fn get_range(&self) -> std::ops::Range<u32> {
        let first_value = self
            .values
            .first()
            .expect("Die should be configured!")
            .number as u32;
        let second_value = self
            .values
            .last()
            .expect("Die should be configured!")
            .number as u32;
        first_value..(second_value + 1)
    }
}

impl<'a> Display for Die<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        for symbol in self.values.iter() {
            write!(f, "{}", symbol)?;
        }
        std::fmt::Result::Ok(())
    }
}

#[derive(Clone)]
struct Symbol<'a> {
    name: &'a str,
    number: usize,
}

macro_rules! CONST_SYMBOL {
    ($symbol: ident, $name:literal, $number:literal ) => {
        const $symbol: Self = Self {
            name: $name,
            number: $number,
        };
    };
}

impl<'a> Symbol<'a> {
    CONST_SYMBOL!(ZERO, "Zero", 0);
    CONST_SYMBOL!(ONE, "One", 1);
    CONST_SYMBOL!(TWO, "Two", 2);
    CONST_SYMBOL!(THREE, "Three", 3);
    CONST_SYMBOL!(FOUR, "Four", 4);
    CONST_SYMBOL!(FIVE, "Five", 5);
    CONST_SYMBOL!(SIX, "Six", 6);
    CONST_SYMBOL!(SEVEN, "Seven", 7);
    CONST_SYMBOL!(EIGHT, "Eight", 8);
    CONST_SYMBOL!(NINE, "Nine", 9);
    CONST_SYMBOL!(TEN, "Ten", 10);
    CONST_SYMBOL!(ELEVEN, "Eleven", 11);
    CONST_SYMBOL!(TWELVE, "Twelve", 12);
    CONST_SYMBOL!(THIRTEEN, "Thirteen", 13);
    CONST_SYMBOL!(FOURTEEN, "Fourteen", 14);
    CONST_SYMBOL!(FIVETEEN, "Fiveteen", 15);
    CONST_SYMBOL!(SIXTEEN, "Sixteen", 16);
    CONST_SYMBOL!(SEVENTEEN, "Seventeen", 17);
    CONST_SYMBOL!(EIGHTEEN, "Eighteen", 18);
    CONST_SYMBOL!(NINETEEN, "Nineteen", 19);
    CONST_SYMBOL!(TWENTY, "Twenty", 20);

    const COLLECTION: [Symbol<'a>; 21] = [
        Symbol::ZERO,
        Symbol::ONE,
        Symbol::TWO,
        Symbol::THREE,
        Symbol::FOUR,
        Symbol::FIVE,
        Symbol::SIX,
        Symbol::SEVEN,
        Symbol::EIGHT,
        Symbol::NINE,
        Symbol::TEN,
        Symbol::ELEVEN,
        Symbol::TWELVE,
        Symbol::THIRTEEN,
        Symbol::FOURTEEN,
        Symbol::FIVETEEN,
        Symbol::SIXTEEN,
        Symbol::SEVENTEEN,
        Symbol::EIGHTEEN,
        Symbol::NINETEEN,
        Symbol::TWENTY,
    ];
}

impl<'a> Display for Symbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Symbol: {}, Number: {}", self.name, self.number)
    }
}

fn configure_die(die: &mut Die<'_>, range: std::ops::Range<usize>) {
    for i in range {
        let j = i - 1;
        die.values.insert(
            j,
            Symbol::COLLECTION
                .get(i)
                .expect("Symbol should exist. Otherwise check the range!")
                .clone(),
        );
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

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
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
        cli::CliOptions::TUI => {
            let mut terminal = ratatui::init();
            terminal.clear()?;
            let app_result = run(terminal);
            ratatui::restore();
            return app_result;
        }
    }
}
