use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub fn close_program(is_recording: &AtomicBool, is_running: &AtomicBool) {
    is_recording.store(false, Ordering::SeqCst);
    is_running.store(false, Ordering::SeqCst);
}