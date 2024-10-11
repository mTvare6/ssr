use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::Stylize;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph,Row,Table,TableState};
use ratatui::Terminal;
use tui_textarea::{Input, Key, TextArea};
use std::io;
use regex::Regex;
mod student_data;
use crate::student_data::{get_student_data_json};

fn inactivate(textarea: &mut TextArea<'_>){
    textarea.set_cursor_style(Style::default());
    textarea.set_block(textarea.block().unwrap().clone().style(Style::default().fg(Color::DarkGray)));
}

fn activate(textarea: &mut TextArea<'_>) {
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(textarea.block().unwrap().clone().style(Style::default().fg(Color::White)));
}





fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;
    let students = get_student_data_json()?.documents;
    let mut show_list = students.clone();
    let re_null = Regex::new("").unwrap();

    let mut re  = [
        Regex::new("").unwrap(),
        Regex::new("").unwrap(),
        Regex::new("").unwrap(),
        Regex::new("").unwrap(),
        Regex::new("").unwrap(),
        Regex::new("").unwrap(),
        Regex::new("").unwrap(),
    ];

    let mut textarea = [
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
    ];

    let boxname = [
        "Name",
        "Programme",
        "Roll No",
        "Department",
        "Hall",
        "Home",
        "Gender"
    ];


    let mut which = 0;
    for (i, mut textarea) in textarea.iter_mut().enumerate() {
        textarea.set_cursor_line_style(Style::default());
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(boxname[i])
        );
        inactivate(&mut textarea);
    }
    activate(&mut textarea[0]);

    let mut table_index = 0;
    let mut table_len = show_list.len();
    let mut table = show_list.clone().into_iter()
        .map(|e| -> Row { vec![e.n, e.d, e.i].into_iter().collect() })
        .collect::<Table>()
        .widths([Constraint::Ratio(1, 3); 3])
        .column_spacing(1)
        .style(Style::new().blue())
        .header(
            Row::new(vec!["Name", "Dept", "Roll"])
                .style(Style::new().bold())
                .bottom_margin(1),
        )
        .block(Block::new().title(""))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">> ");
    let mut table_state = TableState::default();
    table_state.select(Some(table_index));

    macro_rules! switch_box {
        ($l:expr) => {
            if which!=7{
                inactivate(&mut textarea[which]);
            }
            which = $l;
            if which!=7{
                activate(&mut textarea[which]);
            }
        }
    }

    loop {

        term.draw(|f| {
            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Fill(1)
                ])
                .split(f.area());
            let horz_layout_1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(outer_layout[1]);
            let horz_layout_2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(outer_layout[2]);

            let horz_layout_3 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Ratio(4, 5),
                    Constraint::Ratio(1, 5),
                ])
                .split(outer_layout[3]);

            let vert_lagout_1 = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Ratio(2, 5),
                    Constraint::Ratio(3, 5),
                ])
                .split(horz_layout_3[1]);

            f.render_widget(&textarea[0], outer_layout[0]);
            f.render_widget(&textarea[1], horz_layout_1[0]);
            f.render_widget(&textarea[2], horz_layout_1[1]);
            f.render_widget(&textarea[3], horz_layout_1[2]);
            f.render_widget(&textarea[4], horz_layout_2[0]);
            f.render_widget(&textarea[5], horz_layout_2[1]);
            f.render_widget(&textarea[6], horz_layout_2[2]);

            f.render_stateful_widget(&table, horz_layout_3[0], &mut table_state);
            if table_len!=0 {
                f.render_widget(Paragraph::new(format!(
                    "Name: {}\nRoll: {}\nProgramme: {}\nGender: {}\nHome: {}\nHall: {}\nBlood Group: {}\nRoom: {}\nMail: {}@iitk.ac.in",
                    &show_list[table_index].n,
                    &show_list[table_index].i,
                    &show_list[table_index].p,
                    &show_list[table_index].g,
                    &(show_list[table_index].a.clone().unwrap_or("".to_string())),
                    &show_list[table_index].h,
                    &show_list[table_index].b,
                    &show_list[table_index].r,
                    &show_list[table_index].u,
                )), vert_lagout_1[1]);
            }
        })?;

        // blocking, stops until a key press
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input {
                key: e @ (Key::Char('h')|Key::Char('l')),
                ctrl: true,
                ..
            } => {
                switch_box!((which +  match e{ Key::Char('h') => 7, Key::Char('l') => 1, _ => todo!()})  % 8);
            }
            input => {
                if which!=7{
                    textarea[which].input(input);
                    re[which] = Regex::new(&textarea[which].lines()[0]).unwrap_or(re_null.clone());
                    show_list = students.clone().into_iter()
                        .filter(|x| re[0].is_match(&x.n))
                        .filter(|x| re[1].is_match(&x.p))
                        .filter(|x| re[2].is_match(&x.i))
                        .filter(|x| re[3].is_match(&x.d))
                        .filter(|x| re[4].is_match(&x.h))
                        .filter(|x| if let Some(a) = &x.a { re[5].is_match(&a) } else {false})
                        .filter(|x| re[6].is_match(&x.g)).collect();

                    table_len = show_list.len();
                    table_index = 0;
                    table = show_list.clone().into_iter()
                        .map(|e| -> Row { vec![e.n, e.d, e.i].into_iter().collect() })
                        .collect::<Table>()
                        .widths([Constraint::Ratio(1, 3); 3])
                        .column_spacing(1)
                        .style(Style::new().blue())
                        .header(
                            Row::new(vec!["Name", "Dept", "Roll"])
                                .style(Style::new().bold())
                                .bottom_margin(1),
                        )
                        .block(Block::new().title(""))
                        .highlight_style(Style::new().reversed())
                        .highlight_symbol(">> ");
                    table_state.select(Some(table_index));

                }
                else{
                    match input.key{
                        Key::Up => table_index=(table_index+table_len-1)%table_len,
                        Key::Down => table_index=(table_index+1)%table_len,
                        _ => {},
                    };
                    table_state.select(Some(table_index));
                }
            }
        }


    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    Ok(())
}
