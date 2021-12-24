use lazy_static::lazy_static;
use rdev::{listen, simulate, Button, Event, EventType};
use std::sync::Mutex;
use std::time::Duration;

#[derive(Clone, Debug)]
struct Mouse {
    pub bleft: bool,
    pub bright: bool,
}

lazy_static! {
    static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse {
        bleft: false,
        bright: false
    });
}

fn main() {
    let mut cps: u8 = 11;
    let args = std::env::args().collect::<Vec<String>>()[1..].to_vec();
    for arg in args {
        if arg.starts_with("--cps=") || arg.starts_with("-c=") {
            let splited = arg.split("=").collect::<Vec<_>>();
            if splited.len() < 2 {
                println!("WARN: CPS not set, using default: {}", cps);
            }
            match splited[1].parse::<u8>() {
                Ok(parsed) => cps = parsed,
                Err(_e) => {
                    println!(
                        "Could not parse value {}. is it a number or greater than 255?",
                        splited[1]
                    );
                }
            }
        }
    }

    std::thread::spawn(move || {
        let duration = Duration::from_millis(1000 / cps as u64);
        let send = |event_type: &EventType| {
            match simulate(event_type) {
                Ok(()) => (),
                Err(_) => {
                    println!("We could not send {:?}", event_type);
                }
            };
            // std::thread::sleep(Duration::from_millis(20));
        };

        loop {
            let mouse_state = MOUSE.lock().unwrap().clone();
            /* match rx.recv() {
                Ok(d) => {
                    println!("{:?}", d);
                    mouse_state = d;
                }
                Err(e) => {}
            };*/
            //println!("{:?}", mouse_state);
            if mouse_state.bleft {
                send(&EventType::ButtonPress(Button::Left));
                send(&EventType::ButtonRelease(Button::Left));
            }
            if mouse_state.bright {
                send(&EventType::ButtonPress(Button::Right));
                send(&EventType::ButtonRelease(Button::Right));
            }
            std::thread::sleep(duration.clone());
        }
    });

    if let Err(error) = listen(move |ev: Event| {
        match ev.event_type {
            EventType::ButtonPress(button) => match button {
                Button::Unknown(b) => match b {
                    8 => {
                        MOUSE.lock().as_mut().unwrap().bleft = true;
                    }
                    9 => {
                        MOUSE.lock().as_mut().unwrap().bright = true;
                    }
                    _ => {}
                },
                _ => {}
            },
            EventType::ButtonRelease(button) => match button {
                Button::Unknown(b) => match b {
                    8 => {
                        MOUSE.lock().as_mut().unwrap().bleft = false;
                    }
                    9 => {
                        MOUSE.lock().as_mut().unwrap().bright = false;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        };
    }) {
        println!("Error: {:?}", error)
    }
}
