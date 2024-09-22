use std::io;

use buffer::Buffer;
use crossterm::event::{self, KeyCode, KeyEventKind};
use layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::*;
use symbols::border;
use widgets::{block::Position, Block, Borders, Paragraph, Widget};

use crate::calculator::{get_result_from_string, get_right_num};
pub struct App {
    string: String,
    result: f64,
    exit: bool,
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_input()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_input(&mut self) -> io::Result<()> {
        let key;
        if let event::Event::Key(k) = event::read()? {
            key = k;
        } else {
            return Ok(());
        }

        if key.code == KeyCode::Backspace {
            self.string.pop();
            return Ok(());
        }

        if key.kind != KeyEventKind::Press {
            return Ok(());
        }
        match key.code {
            KeyCode::Char(ch) => match ch {
                'c' => *self = Self { string: String::new(), result: 0.0, exit: false },
                'q' => self.exit = true,
                '0'..='9' => self.add_num(ch),

                '+' | '-' | '*' | '/' => self.add_operator(ch),

                '.' => self.add_floating_point(),

                '(' => self.add_left_bracket(),

                ')' => self.add_right_bracket(),

                _ => (),
            },
            KeyCode::Enter => {
                if let Some(num) = get_result_from_string(&self.string) {
                    self.result = num;
                    self.string = self.result.to_string();
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn add_left_bracket(&mut self) {
        if self.string.is_empty() {
            self.string += "(";
        } else if let '0'..='9' = self.string.chars().nth(self.string.len() - 1).unwrap() {
            self.string += "*(";
        } else {
            self.string += "(";
        }
    }

    fn add_right_bracket(&mut self) {
        if self.string.is_empty() {
            return;
        }
        let mut count = 0;
        for char in self.string.chars() {
            if char == '(' {
                count += 1;
            } else if char == ')' {
                count -= 1;
            }
        }
        if count > 0 {
            match self.string.chars().nth(self.string.len() - 1).unwrap() {
                '0'..='9' | ')' => self.string += ")",
                '(' => {
                    self.string.pop();
                }
                _ => {
                    self.string.pop();
                    self.string += ")";
                }
            }
        }
        if let Some(num) = get_result_from_string(&self.string) {
            self.result = num;
        }
    }

    fn add_floating_point(&mut self) {
        if self.string.is_empty() {
            return;
        }
        if let '0'..='9' = self.string.chars().nth(self.string.len() - 1).unwrap() {
            if let Some(n) = get_right_num(&self.string) {
                if n % 1.0 == 0.0 {
                    self.string += &'.'.to_string();
                }
            }
        }
    }

    fn add_operator(&mut self, operator: char) {
        if self.string.is_empty() {
            return;
        }
        match self.string.chars().nth(self.string.len() - 1).unwrap() {
            '+' | '-' | '*' | '/' | '.' => {
                self.string.pop();
                self.string += &operator.to_string()
            }
            _ => self.string += &operator.to_string(),
        }
    }

    fn add_num(&mut self, num: char) {
        if self.string.is_empty() {
            self.string += &num.to_string();
        } else if let ')' = self.string.chars().nth(self.string.len() - 1).unwrap() {
            self.string += "*";
            self.string += &num.to_string();
        } else {
            self.string += &num.to_string();
        }
        if let Some(num) = get_result_from_string(&self.string) {
            self.result = num;
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let expression = Paragraph::new(self.string.as_str());
        let result = Paragraph::new(self.result.to_string());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Space above the widget
                Constraint::Length(3), // Height of the centered widget
                Constraint::Min(0),    // Fill the remaining space
            ])
            .split(area);

        let top_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(5),  // Space on the left
                Constraint::Percentage(90), // Centered widget area
                Constraint::Percentage(5),  // Space on the right
            ])
            .split(chunks[1]);

        let chunks_result = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7), // Space above the widget
                Constraint::Length(3), // Height of the centered widget
                Constraint::Min(0),    // Fill the remaining space
            ])
            .split(area);

        let top_chunk_result = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(5),  // Space on the left
                Constraint::Percentage(90), // Centered widget area
                Constraint::Percentage(5),  // Space on the right
            ])
            .split(chunks_result[1]);

        let block1 = Block::default()
            .title("EXPRESSION")
            .title_alignment(Alignment::Center)
            .title_position(Position::Top)
            .title_bottom("press keys to add to expression")
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let block2 = Block::default()
            .title("RESULT")
            .title_alignment(Alignment::Center)
            .title_position(Position::Top)
            .borders(Borders::ALL)
            .border_set(border::DOUBLE);

        expression.block(block1).render(top_chunk[1], buf);
        result.block(block2).render(top_chunk_result[1], buf);
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            string: String::new(),
            result: 0.0,
            exit: false,
        }
    }
}
