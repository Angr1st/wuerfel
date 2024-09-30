use std::fmt::Display;

use oorandom;

struct State<'a> {
    dices: Vec<Dice<'a>>,
}

impl<'a> State<'a> {
    fn add_dice(&mut self, dice: Dice<'a>) {
        self.dices.push(dice);
    }
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self { dices: vec![] }
    }
}

impl<'a> Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.dices.len() == 0 {
            writeln!(f, "No dices configured!")?;
        } else {
            writeln!(f, "Outputting all currently configured dices.")?;
            for dice in self.dices.iter() {
                write!(f, "{}", dice)?;
            }
        }
        std::fmt::Result::Ok(())
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

fn configure_dice(dice: &mut Dice<'_>, range: std::ops::Range<usize>) {
    for i in range {
        let j = i - 1;
        dice.values.insert(
            j,
            Symbol::COLLECTION
                .get(i)
                .expect("Symbol should exist. Otherwise check the range!")
                .clone(),
        );
    }
}

fn main() {
    let mut state = State::default();

    let mut d4 = Dice::new("D4".to_string());
    configure_dice(&mut d4, 1..5);
    state.add_dice(d4);
    let mut d6 = Dice::new("D6".to_string());
    configure_dice(&mut d6, 1..7);
    state.add_dice(d6);
    let mut d10 = Dice::new("D10".to_string());
    configure_dice(&mut d10, 1..11);
    state.add_dice(d10);
    let mut d20 = Dice::new("D20".to_string());
    configure_dice(&mut d20, 1..21);
    state.add_dice(d20);

    println!("{}", state);
}
