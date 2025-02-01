// src/timer/display.rs
use std::io::{self, Write, stdout};
use std::time::Duration;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    cursor,
    style::{self, Attribute, SetAttribute},
};

pub struct DisplayManager {
    stdout: io::Stdout,
}

impl DisplayManager {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn select_duration(&self) -> io::Result<Duration> {
        let options = ["1 hr", "2 hr", "3 hr", "4 hr", "Exit"];
        let mut selected = 0;
        
        loop {
            let (width, height) = terminal::size()?;
            execute!(self.stdout.lock(), terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
            
            self.draw_menu(&options, selected, width, height)?;

            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => {
                        selected = selected.checked_sub(1).unwrap_or(options.len() - 1);
                    }
                    KeyCode::Down => {
                        selected = (selected + 1) % options.len();
                    }
                    KeyCode::Enter => {
                        return Ok(match selected {
                            0 => Duration::from_secs(5),   
                            1 => Duration::from_secs(2 * 60 * 60),
                            2 => Duration::from_secs(3 * 60 * 60),
                            3 => Duration::from_secs(4 * 60 * 60),
                            4 => std::process::exit(0),
                            _ => unreachable!()
                        });
                    }
                    KeyCode::Esc => {
                        self.stdout.lock().flush()?;
                        std::process::exit(0)
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn check_for_exit(&self) -> io::Result<bool> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if let KeyCode::Esc = key_event.code {
                    std::process::exit(0);
                }
            }
        }
        Ok(false)
    }

    pub fn update_timer(&self, remaining: Duration) -> io::Result<()> {
        let hours = remaining.as_secs() / 3600;
        let minutes = (remaining.as_secs() % 3600) / 60;
        let seconds = remaining.as_secs() % 60;

        let (width, height) = terminal::size()?;
        
        self.draw_ascii_art(width, height)?;

        let ascii_height = 19;
        let timer_y = height / 2 - ascii_height + ascii_height + 2;

        let mut stdout = self.stdout.lock();
        
        execute!(
            stdout, 
            cursor::MoveTo(width/2 - 8, timer_y), 
            terminal::Clear(ClearType::CurrentLine)
        )?;
        print!("Time Remaining:");

        execute!(
            stdout, 
            cursor::MoveTo(width/2 -4, timer_y + 1), 
            terminal::Clear(ClearType::CurrentLine)
        )?;

        print!("{:02}:{:02}:{:02}", hours, minutes, seconds);
        stdout.flush()?;
        
        Ok(())
    }

    pub fn show_finished_message(&self) -> io::Result<()> {
        let (width, height) = terminal::size()?;
        let center_x = width / 2 - 15; 
        let center_y = height / 2;

        execute!(
            self.stdout.lock(), 
            cursor::MoveTo(center_x + 8, center_y), 
            terminal::Clear(ClearType::CurrentLine)
        )?;
        println!("Timer Finished!");
        
        Ok(())
    }

    // Private helper methods
    fn draw_menu(&self, options: &[&str], selected: usize, width: u16, height: u16) -> io::Result<()> {
        let title = "Select Timer Duration";
        let divider = "-------------------";
        
        let title_x = (width as usize - title.len()) / 2;
        let divider_x = (width as usize - divider.len()) / 2;
        let menu_x = (width as usize - 20) / 2;
        let menu_y = height as usize / 2 - options.len() / 2;

        let mut stdout = self.stdout.lock();

        execute!(stdout, cursor::MoveTo(title_x as u16, menu_y as u16 - 2))?;
        print!("{}", title);
        
        execute!(stdout, cursor::MoveTo(divider_x as u16, menu_y as u16 - 1))?;
        print!("{}", divider);

        for (index, &option) in options.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(menu_x as u16, (menu_y + index) as u16))?;
            
            if index == selected {
                execute!(stdout, SetAttribute(Attribute::Reverse))?;
                print!("{:^20}", option);
                execute!(stdout, SetAttribute(Attribute::Reset))?;
            } else {
                print!("{:^20}", option);
            }
            execute!(stdout, cursor::MoveTo(menu_x as u16, (menu_y + index + 1) as u16))?;
        }
        
        Ok(())
    }

    fn draw_ascii_art(&self, width: u16, height: u16) -> io::Result<()> {
        let ascii_art = [
            "                           (   )",
            "                          (    )",
            "                           (    )",
            "                          (    )",
            "                            )  )",
            "                           (  (                  /\\",
            "                            (_)                 /  \\  /\\",
            "                    ________[_]________      /\\/    \\/  \\",
            "           /\\      /\\        ______    \\    /   /\\/\\  /\\/\\",
            "          /  \\    //_\\       \\    /\\    \\  /\\/\\/    \\/    \\",
            "   /\\    / /\\/\\  //___\\       \\__/  \\    \\/",
            "  /  \\  /\\/    \\//_____\\       \\ |[]|     \\",
            " /\\/\\/\\/       //_______\\       \\|__|      \\",
            "/      \\      /XXXXXXXXXX\\                  \\",
            "        \\    /_I_II  I__I_\\__________________\"",
            "               I_I|  I__I_____[]_|_[]_____I",
            "               I_II  I__I_____[]_|_[]_____I",
            "               I II__I  I     XXXXXXX     I",
            "            ~~~~~\"   \"~~~~~~~~~~~~~~~~~~~~~~~~",
        ];

        let ascii_width = 58;
        let ascii_height = ascii_art.len() as u16;
        let center_x = (width / 2).saturating_sub(ascii_width as u16 / 2);
        let center_y = height / 2;

        let mut stdout = self.stdout.lock();
        execute!(stdout, cursor::MoveTo(center_x, center_y - ascii_height), terminal::Clear(ClearType::All))?;

        for (i, line) in ascii_art.iter().enumerate() {
            execute!(
                stdout, 
                cursor::MoveTo(center_x, center_y - ascii_height + i as u16),
                SetAttribute(Attribute::Bold),
                crossterm::style::SetForegroundColor(style::Color::Magenta)
            )?;
            writeln!(stdout, "{}", line)?;
        }
        
        Ok(())
    }
}
