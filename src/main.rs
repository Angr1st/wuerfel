use std::{borrow::Borrow, fmt::Display};

use oorandom;

struct State<'a> {
    name: String,
    values: Vec<Symbol<'a>>,
}

impl<'a> State<'a> {
    fn new(name: String) -> State<'a> {
        Self {
            name,
            values: vec![],
        }
    }
}

impl<'a> Display for State<'a> {
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

impl<'a> Symbol<'a> {
    const ZERO: Self = Self {
        name: "Zero",
        number: 0,
    };
    const ONE: Self = Self {
        name: "One",
        number: 1,
    };
    const TWO: Self = Self {
        name: "Two",
        number: 2,
    };
    const THREE: Self = Self {
        name: "Three",
        number: 3,
    };
    const FOUR: Self = Self {
        name: "Four",
        number: 4,
    };
    const FIVE: Self = Self {
        name: "Five",
        number: 5,
    };
    const SIX: Self = Self {
        name: "Six",
        number: 6,
    };

    const COLLECTION: [Symbol<'a>; 7] = [
        Symbol::ZERO,
        Symbol::ONE,
        Symbol::TWO,
        Symbol::THREE,
        Symbol::FOUR,
        Symbol::FIVE,
        Symbol::SIX,
    ];
}

impl<'a> Display for Symbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Symbol: {}, Number: {}", self.name, self.number)
    }
}

fn main() {
    let mut state = State::new("D6".to_string());

    for i in 1..7 {
        let j = i - 1;
        state.values.insert(
            j,
            Symbol::COLLECTION
                .get(i)
                .expect("Symbol should exist. Otherwise check the range!")
                .clone(),
        );
    }
    println!("{}", state);
}
