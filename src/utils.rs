use std::io::stdout;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::process;

use crossterm::execute;
use crossterm::terminal::LeaveAlternateScreen;

pub fn close_program(is_running: &AtomicBool) {
    is_running.store(false, Ordering::SeqCst);
    execute!(stdout(), LeaveAlternateScreen).unwrap();
    process::exit(0);
}