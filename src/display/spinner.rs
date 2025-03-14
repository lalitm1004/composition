use std::{
    io::{self, Write},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

static SPINNER: Mutex<Option<SpinnerState>> = Mutex::new(None);

struct SpinnerState {
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

pub fn start(message: &str) {
    let mut spinner_guard = SPINNER.lock().unwrap();

    if spinner_guard.is_some() {
        drop(spinner_guard);
        end();
        spinner_guard = SPINNER.lock().unwrap();
    }

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let message = message.to_string();

    let handle = thread::spawn(move || {
        let spinner_chars = ['|', '/', '-', '\\'];
        let mut index = 0;

        while running_clone.load(Ordering::SeqCst) {
            print!("\r{} {}", message, spinner_chars[index]);
            io::stdout().flush().unwrap();
            index = (index + 1) % spinner_chars.len();
            thread::sleep(Duration::from_millis(100));
        }

        print!("\r{} \r", " ".repeat(message.len() + 2));
        io::stdout().flush().unwrap();
    });

    *spinner_guard = Some(SpinnerState {
        running,
        handle: Some(handle),
    });
}

pub fn end() {
    let mut spinner_guard = SPINNER.lock().unwrap();

    if let Some(state) = spinner_guard.take() {
        state.running.store(false, Ordering::SeqCst);

        if let Some(handle) = state.handle {
            drop(spinner_guard); // Release the lock before joining
            let _ = handle.join();
        }
    }
}
