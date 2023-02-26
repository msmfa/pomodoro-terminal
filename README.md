# Pomodoro Timer

This is a simple Pomodoro timer written in Rust that runs in the command line.

## Requirements

To run this program, you'll need to have Rust installed on your system. You can download Rust from the official website:

- https://www.rust-lang.org/tools/install

## Usage

To start the Pomodoro timer, open a terminal window and navigate to the directory that contains the `main.rs` file. Then, run the following command:

`cargo run`

This will compile and run the Rust program. The Pomodoro timer will start running and will print messages to the console for each Pomodoro session.

To stop the program, press `Ctrl-C`.

## Customization

You can customize the duration of the work and break sessions by modifying the `WORK_DURATION` and `BREAK_DURATION` constants in the `main.rs` file. The values are in seconds.

## Beep Sound

By default, the program will play a beep sound after each Pomodoro session. This works on most terminal emulators. If the beep sound doesn't work on your terminal, you can disable it by removing the `print!("\x07");` line from the code.
