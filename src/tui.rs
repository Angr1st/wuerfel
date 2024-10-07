use std::io;

use oorandom::Rand32;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

use crate::State;

struct App<'a> {
    state: State<'a>,
    current_index: Option<usize>,
    current_range: std::ops::Range<usize>,
    current_die_roll: Option<u32>,
    random: Rand32,
    exit: bool,
}

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut exit = self.exit;
        while !exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            exit = self.exit;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => self.exit(),
            KeyCode::Left => self.previous_die(),
            KeyCode::Right => self.next_die(),
            KeyCode::Enter => self.roll_die(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn previous_die(&mut self) {
        if self.current_range.len() == 0 {
            return;
        }
        if self.current_index.is_none() && self.current_range.len() != 0 {
            self.current_index = Some(self.current_range.end);
        } else {
            self.current_index = self.current_index.map(|index| {
                if index == 0 {
                    self.current_range.end
                } else {
                    index - 1
                }
            });
        }
    }

    fn next_die(&mut self) {
        if self.current_range.len() == 0 {
            return;
        }
        if self.current_index.is_none() && self.current_range.len() != 0 {
            self.current_index = Some(0);
        } else {
            let max_index = self.current_range.end;
            self.current_index =
                self.current_index
                    .map(|index| if index + 1 > max_index { 0 } else { index + 1 });
        }
    }

    fn roll_die(&mut self) {
        if let Some(index) = self.current_index {
            if let Some(die) = self.state.get_dice().get(index) {
                let die_range = die.get_range();
                self.current_die_roll = Some(self.random.rand_range(die_range));
            }
        }
    }
}

impl<'a> Widget for &'a App<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" wuerfel App ".bold());
        let instructions = if self.current_index.is_some() {
            Title::from(Line::from(vec![
                " Previous ".into(),
                "<Left>".blue().bold(),
                " Roll die ".into(),
                "<Enter>".blue().bold(),
                " Next ".into(),
                "<Right>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]))
        } else {
            Title::from(Line::from(vec![
                " Previous ".into(),
                "<Left>".blue().bold(),
                " Next ".into(),
                "<Right>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ]))
        };
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let mut dice_text = Text::from(vec![Line::from(vec![
            "Dice: ".into(),
            self.state.print_dice().unwrap_or("None".into()).yellow(),
        ])]);
        if let Some(index) = self.current_index {
            if let Some(die) = self.state.get_dice().get(index) {
                dice_text.push_line(Line::from(vec![
                    "Currently selected die: ".into(),
                    die.get_name().into(),
                ]));
            }
        }
        if let Some(roll) = self.current_die_roll {
            dice_text.push_line(Line::from(vec![
                "Current roll: ".into(),
                roll.to_string().into(),
            ]));
        }
        Paragraph::new(dice_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

pub fn run_tui<'a>(state: State<'a>, random: Rand32) -> io::Result<()> {
    let range = 0..(state.get_dice().len() - 1);
    let mut app = App {
        state,
        random,
        current_index: None,
        current_range: range,
        current_die_roll: None,
        exit: false,
    };
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
