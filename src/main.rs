use std::{io::{self, Read}, thread, time};
use termion::{clear, cursor};

const WORK_DURATION: u64 = 25 * 60; // 25 minutes in seconds
const BREAK_DURATION: u64 = 5 * 60; // 5 minutes in seconds

fn main() -> io::Result<()> {
    let work_duration = time::Duration::from_secs(WORK_DURATION);
    let break_duration = time::Duration::from_secs(BREAK_DURATION);

    let mut time_remaining = work_duration;
    let mut is_work_session = true;

    loop {
        print!("{}{}", clear::All, cursor::Goto(1, 1));

        let session_type = if is_work_session {
            "Work"
        } else {
            "Break"
        };
        let time_remaining_str = format!("{}:{:02}", time_remaining.as_secs() / 60, time_remaining.as_secs() % 60);

        println!("{} session - time remaining: {}", session_type, time_remaining_str);
        println!("Press Enter to skip to the next session");

        let mut buffer = [0; 1];
        let _ = io::stdin().read_exact(&mut buffer);

        if buffer[0] == b'\n' {
            if is_work_session {
                is_work_session = false;
                time_remaining = break_duration;
            } else {
                is_work_session = true;
                time_remaining = work_duration;
            }
        } else {
            time_remaining = match time_remaining.checked_sub(time::Duration::from_secs(1)) {
                Some(duration) => duration,
                None => {
                    // Play a beep sound after each session
                    print!("\x07"); 
                    print!("\x07"); 
                    print!("\x07"); 

                    if is_work_session {
                        is_work_session = false;
                        break_duration
                    } else {
                        is_work_session = true;
                        work_duration
                    }
                }
            };
        }

        thread::sleep(time::Duration::from_secs(1));
    }
}
