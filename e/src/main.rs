use std::io::{self, Write};
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};
use chrono::{Local};

fn main() {
    loop {
        println!("3 options: (1) start timer, (2) find date, or (3) exit");
        println!("(1/2/3)");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => start_timer(),
            "2" => Ok(find_date()),
            "3" => {
                println!("Exiting...");
                return;
            }
            _ => Ok(println!("Invalid option. Please enter 1, 2, or 3.")),
        };
    }
}

fn start_timer() -> io::Result<()> {
    println!("Starting timer... \nPress spacebar to stop.");
    let start = Instant::now();
    loop{
        let elapsed = start.elapsed();
        print!("\rElapsed time: {:.2}s", elapsed.as_secs_f64());
        io::stdout().flush().unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read()? {
                if key_event.code == KeyCode::Char(' ') {
                    break;
                }
            }
        }
    }
    println!("\nTimer stopped. Total elapsed time: {:.2?}", start.elapsed());
    Ok(())
}

fn find_date() {
    let today = Local::now();
    println!("Today's date is: {}", today.format("%m/%d/%Y"));
}
