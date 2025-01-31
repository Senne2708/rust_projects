use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::process::Command;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    cursor,
    style::{Attribute, SetAttribute},
};

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let duration = select_duration()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    countdown(duration)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn play_sound() {
    // Uses macOS built-in `afplay` to play a system sound
    Command::new("afplay")
        .arg("/System/Library/Sounds/Pop.aiff")
        .spawn()
        .expect("Failed to play sound");
}

fn select_duration() -> io::Result<Duration> {
    let options = ["1 hr", "2 hr", "3 hr", "4 hr", "Exit"];
    let mut selected = 0;
    let mut stdout = io::stdout();

    loop {
        let (width, height) = terminal::size()?;
        
        execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        
        let title = "Select Timer Duration";
        let divider = "-------------------";
        
        let title_x = (width as usize - title.len()) / 2;
        let divider_x = (width as usize - divider.len()) / 2;
        let menu_x = (width as usize - 20) / 2;
        let menu_y = height as usize / 2 - options.len() / 2;

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
        }

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
                        0 => Duration::from_secs(1 * 60 * 60),   
                        1 => Duration::from_secs(2 * 60 * 60),
                        2 => Duration::from_secs(3 * 60 * 60),
                        3 => Duration::from_secs(4 * 60 * 60),
                        4 => std::process::exit(0),
                        _ => unreachable!()
                    });
                }
                KeyCode::Esc => std::process::exit(0),
                _ => {}
            }
        }
    }
}

fn countdown(total_duration: Duration) -> io::Result<()> {
    let start = Instant::now();
    let mut stdout = io::stdout();

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if let KeyCode::Esc = key_event.code {
                    break;
                }
            }
        }

        let elapsed = start.elapsed();
        
        if elapsed >= total_duration {
            execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::CurrentLine))?;
            println!("Timer Finished!");
            play_sound();
            break;
        }

        let remaining = total_duration - elapsed;
        let hours = remaining.as_secs() / 3600;
        let minutes = (remaining.as_secs() % 3600) / 60;
        let seconds = remaining.as_secs() % 60;

        execute!(
            stdout, 
            cursor::MoveTo(0, 0), 
            terminal::Clear(ClearType::CurrentLine)
        )?;

        print!("Time Remaining: {:02}:{:02}:{:02}", hours, minutes, seconds);
        stdout.flush()?;
    }

    Ok(())
}
