use std::{io, str::FromStr};

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

use crate::{core::Die, State};

struct App<'a> {
    state: State<'a>,
    current_die: Option<&'a Die<'a>>,
    current_die_roll: Option<u32>,
    random: Rand32,
    exit: bool,
}

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
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

    fn previous_die(&mut self) {}

    fn next_die(&mut self) {}

    fn roll_die(&mut self) {}
}

impl<'a> Widget for &App<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" wuerfel App ".bold());
        let instructions = Title::from(Line::from(vec![
            " Previous ".into(),
            "<Left>".blue().bold(),
            " Next ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Dice: ".into(),
            self.state.print_dice().unwrap_or("None".into()).yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

pub fn run_tui<'a>(state: State<'a>, random: Rand32) -> io::Result<()> {
    let mut app = App {
        state,
        random,
        current_die: None,
        current_die_roll: None,
        exit: false,
    };
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
