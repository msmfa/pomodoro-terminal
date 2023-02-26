use std::{thread, time};

const WORK_DURATION: u64 = 25 * 60; // 25 minutes in seconds
const BREAK_DURATION: u64 = 5 * 60; // 5 minutes in seconds

fn main() {
    let work_duration = time::Duration::from_secs(WORK_DURATION);
    let break_duration = time::Duration::from_secs(BREAK_DURATION);

    loop {
        println!("Work for {} minutes...", WORK_DURATION / 60);
        thread::sleep(work_duration);

        println!("Take a break for {} minutes...", BREAK_DURATION / 60);
        thread::sleep(break_duration);

        print!("\x07"); // Play a beep sound after each session
        print!("\x07"); // Play a beep sound after each session
        print!("\x07"); // Play a beep sound after each session
    }
}