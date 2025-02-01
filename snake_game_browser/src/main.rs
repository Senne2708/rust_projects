use std::{io::Write, thread, time::Duration};

fn count_down(seconds: u32) {
    for i in (0..=seconds).rev() {
        print!("\rTime left: {:02} seconds ", i);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));

    }
    println!("\rTime's up")

}
fn main() {
    let countdown_timer = 10;
    count_down(countdown_timer);
}
