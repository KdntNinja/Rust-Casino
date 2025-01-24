use std::io::{self};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        let mut input_mode = true;
        let mut input = String::new();
        let mut weights = Vec::new();
        let mut num_weights = 0;
        let mut current_weight = 0;
        let mut error_message = String::new();

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        "Fruit Weights",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ));
                f.render_widget(block, size);

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                            Constraint::Percentage(60),
                            Constraint::Percentage(10),
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let input_text = if input_mode {
                    if current_weight == 0 {
                        "Enter the number of weights:".to_string()
                    } else {
                        format!("Enter weight {} in grams:", current_weight)
                    }
                } else {
                    format!("The mean weight of the fruits is: {:.2} grams", calculate_mean(&weights))
                };

                let input_paragraph = Paragraph::new(format!("{}\n{}", input_text, input))
                    .style(Style::default().fg(Color::White))
                    .block(Block::default().borders(Borders::ALL).title("Input"));
                f.render_widget(input_paragraph, chunks[0]);

                let output_text = if !input_mode {
                    format!("The mean weight of the fruits is: {:.2} grams", calculate_mean(&weights))
                } else {
                    weights.iter().map(|w| format!("{:.2} grams", w)).collect::<Vec<String>>().join("\n")
                };

                let output_paragraph = Paragraph::new(output_text)
                    .style(Style::default().fg(Color::White))
                    .block(Block::default().borders(Borders::ALL).title("Output"));
                f.render_widget(output_paragraph, chunks[3]);

                if !error_message.is_empty() {
                    let error_paragraph = Paragraph::new(error_message.clone())
                        .style(Style::default().fg(Color::Red))
                        .block(Block::default().borders(Borders::ALL).title("Error"));
                    f.render_widget(error_paragraph, chunks[4]);
                }
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => break,
                    KeyCode::Enter => {
                        if input_mode {
                            if current_weight == 0 {
                                match input.trim().parse::<usize>() {
                                    Ok(num) => {
                                        num_weights = num;
                                        if num_weights > 0 {
                                            current_weight += 1;
                                            error_message.clear();
                                        } else {
                                            error_message = "Please enter a positive number.".to_string();
                                        }
                                    }
                                    Err(_) => {
                                        error_message = "Invalid input. Please enter a valid number.".to_string();
                                    }
                                }
                            } else {
                                match input.trim().parse::<f64>() {
                                    Ok(weight) => {
                                        if weight > 0.0 {
                                            weights.push(weight);
                                            if current_weight == num_weights {
                                                input_mode = false;
                                            } else {
                                                current_weight += 1;
                                            }
                                            error_message.clear();
                                        } else {
                                            error_message = "Please enter a positive weight.".to_string();
                                        }
                                    }
                                    Err(_) => {
                                        error_message = "Invalid input. Please enter a valid number.".to_string();
                                    }
                                }
                            }
                            input.clear();
                        }
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    _ => {}
                }
            }
        }
    }
}

fn calculate_mean(weights: &Vec<f64>) -> f64 {
    let total_weight: f64 = weights.iter().sum();
    total_weight / weights.len() as f64
}
