use std::{borrow::Borrow, fmt::Display};

use oorandom;

struct State<'a> {
    dices: Vec<Dice<'a>>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self { dices: vec![] }
    }
}

struct Dice<'a> {
    name: String,
    values: Vec<Symbol<'a>>,
}

impl<'a> Dice<'a> {
    fn new(name: String) -> Dice<'a> {
        Self {
            name,
            values: vec![],
        }
    }
}

impl<'a> Display for Dice<'a> {
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

    const COLLECTION: [Symbol<'a>; 8] = [
        Symbol::ZERO,
        Symbol::ONE,
        Symbol::TWO,
        Symbol::THREE,
        Symbol::FOUR,
        Symbol::FIVE,
        Symbol::SIX,
        Symbol::SEVEN,
    ];
}

impl<'a> Display for Symbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Symbol: {}, Number: {}", self.name, self.number)
    }
}

fn main() {
    let mut state = State::default();
    let mut d6 = Dice::new("D6".to_string());

    for i in 1..7 {
        let j = i - 1;
        d6.values.insert(
            j,
            Symbol::COLLECTION
                .get(i)
                .expect("Symbol should exist. Otherwise check the range!")
                .clone(),
        );
    }
    println!("{}", d6);
    state.dices.push(d6);
}
