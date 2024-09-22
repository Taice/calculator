use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};
use layout::{Alignment, Constraint, Direction, Layout};
use ratatui::*;
use symbols::border;
use widgets::{block::Position, Block, Borders, Paragraph};

use crate::calculator::{get_result_from_string, get_right_num};

pub fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut expression_string: String = String::new();
    let mut expression_result: f64 = 0.0;
    loop {
        terminal.draw(|frame| {
            let expression = Paragraph::new(expression_string.as_str());
            let result = Paragraph::new(format!("{}", expression_result));

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Space above the widget
                    Constraint::Length(3), // Height of the centered widget
                    Constraint::Min(0),    // Fill the remaining space
                ])
                .split(frame.area());

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
                .split(frame.area());

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

            frame.render_widget(expression.block(block1), top_chunk[1]);
            frame.render_widget(result.block(block2), top_chunk_result[1]);
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if let KeyCode::Char(ch) = key.code {
                    match ch {
                        'q' => return Ok(()),
                        '0'..='9' => {
                            if expression_string.is_empty() {
                                expression_string += &ch.to_string();
                            } else if let ')' = expression_string
                                .chars()
                                .nth(expression_string.len() - 1)
                                .unwrap()
                            {
                                expression_string += "*";
                                expression_string += &ch.to_string();
                            } else {
                                expression_string += &ch.to_string();
                            }
                            if let Some(num) = get_result_from_string(&expression_string) {
                                expression_result = num;
                            }
                        }
                        '+' | '-' | '*' | '/' => {
                            match expression_string
                                .chars()
                                .nth(expression_string.len() - 1)
                                .unwrap()
                            {
                                '+' | '-' | '*' | '/' | '.' => {
                                    expression_string.pop();
                                    expression_string += &ch.to_string()
                                }
                                _ => expression_string += &ch.to_string(),
                            }
                        }
                        '.' => {
                            if expression_string.is_empty() {
                                continue;
                            }
                            if let '0'..='9' = expression_string
                                .chars()
                                .nth(expression_string.len() - 1)
                                .unwrap()
                            {
                                let num = get_right_num(&expression_string);
                                if num % 1.0 == 0.0 {
                                    expression_string += &'.'.to_string();
                                }
                            }
                        }
                        '(' => {
                            if expression_string.is_empty() {
                                expression_string += "(";
                            } else if let '0'..='9' = expression_string
                                .chars()
                                .nth(expression_string.len() - 1)
                                .unwrap()
                            {
                                expression_string += "*(";
                            } else {
                                expression_string += "(";
                            }
                        }
                        ')' => {
                            if expression_string.is_empty() {
                                continue;
                            }
                            let mut count = 0;
                            for char in expression_string.chars() {
                                if char == '(' {
                                    count += 1;
                                } else if char == ')' {
                                    count -= 1;
                                }
                            }
                            if count > 0 {
                                match expression_string
                                    .chars()
                                    .nth(expression_string.len() - 1)
                                    .unwrap()
                                {
                                    '0'..='9' | ')' => expression_string += ")",
                                    '(' => {
                                        expression_string.pop();
                                    }
                                    _ => {
                                        expression_string.pop();
                                        expression_string += ")";
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            if key.code == KeyCode::Backspace {
                expression_string.pop();
            } else if key.code == KeyCode::Enter {
                expression_string = expression_result.to_string();
            }
        }
    }
}
