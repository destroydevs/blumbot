use crate::screen::Clicker;
use crate::window_search::get_all_windows_info;
use colored::Colorize;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::string::ToString;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod screen;
mod key_listener;
mod window_search;

const PREFIX: &str = " [BLUMBOT] ";
const ERROR: &str = " [ ERROR ] ";

fn main() {

    execute_with_log(|state| {

        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            key_listener::register(sender);
        });

        if receiver.recv().unwrap() {
            *state = true;
        }

    }, "Creating hook...", "Hook created!", None);

    let telegram_window = Rc::new(RefCell::new(None));

    let window_ptr = Rc::clone(&telegram_window);

    execute_with_log(move |state| {
        let all_windows = get_all_windows_info()
            .into_iter()
            .filter(|w| w.is_visible && w.width() > 100 && w.height() > 100)
            .collect::<Vec<_>>();

        if let Some(w) = all_windows.iter()
            .find(|w| w.class.starts_with("Qt51515"))
        {
            *window_ptr.borrow_mut() = Some(w.clone());
            *state = true;
        }

    }, "Searching window...", "Window has been found!", Some("Window not found."));

    let target_window = telegram_window.borrow()
        .as_ref()
        .unwrap()
        .clone();



    let clicker = Clicker::new();
    loop {
        clicker.find_and_click_green(&target_window);
    }
}

fn execute_with_log<F>(f: F, start: &str, end: &str, error: Option<&str>)
where
    F: FnOnce(&mut bool),
{
    let mut state = false;
    let instant = Instant::now();
    println!("{} {}", PREFIX.black().on_bright_green(), start.green().bold());

    f(&mut state);

    if state {
        println!("{} {} {}ms",
                 PREFIX.black().on_bright_green(),
                 end.green().bold(),
                 instant.elapsed().as_millis()
        );
    } else {

        let mut error_message: &str = end;

        if error.is_some() {
            error_message = error.unwrap();
        }

        println!("{} {} {}ms",
                 ERROR.black().on_bright_red(),
                 error_message.red().bold(),
                 instant.elapsed().as_millis()
        );

        thread::sleep(Duration::from_secs(60*60));
    }
}
