use std::{fmt::Display, io};

pub(crate) struct State<'a> {
    dice: Vec<Die<'a>>,
}

impl<'a> State<'a> {
    pub(crate) fn add_die(&mut self, dice: Die<'a>) {
        self.dice.push(dice);
    }

    pub(crate) fn print_dice(&self) -> Option<String> {
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

    pub(crate) fn get_dice(&'a self) -> &'a [Die<'a>] {
        &self.dice
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

pub(crate) struct Die<'a> {
    name: String,
    values: Vec<Symbol<'a>>,
}

impl<'a> Die<'a> {
    pub(crate) fn new(name: String) -> Die<'a> {
        Self {
            name,
            values: vec![],
        }
    }

    pub(crate) fn insert_symbol(&mut self, symbol: Symbol<'a>, index: usize) {
        self.values.insert(index, symbol);
    }

    pub(crate) fn get_range(&self) -> std::ops::Range<u32> {
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

    pub(crate) fn get_name(&self) -> &str {
        &self.name
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
pub(crate) struct Symbol<'a> {
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

    pub(crate) const COLLECTION: [Symbol<'a>; 21] = [
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

pub(crate) enum Error {
    Io(io::Error),
    Eframe(eframe::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<eframe::Error> for Error {
    fn from(value: eframe::Error) -> Self {
        Self::Eframe(value)
    }
}
