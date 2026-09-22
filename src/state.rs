// 1. Define the states
// 2. Define the commands
// 3. Define the control pannel
// 4. spawn the control pannel

use std::sync::mpsc;

#[derive(Debug)]
struct Green;
#[derive(Debug)]
struct Yellow;
#[derive(Debug)]
struct Red;

impl Red {
    pub fn new() -> Self {
        println!("start as red");
        Red {}
    }

    pub fn to_green(self) -> Green {
        println!("red to green");
        Green {}
    }
}

impl Green {
    pub fn to_yellow(self) -> Yellow {
        println!("green to yellow");
        Yellow {}
    }
}

impl Yellow {
    pub fn to_red(self) -> Red {
        println!("yellow to red");
        Red {}
    }
}

#[derive(Debug)]
enum TrafficLight {
    Green(Green),
    Yellow(Yellow),
    Red(Red),
}

#[derive(Debug)]
enum Command {
    ToGreen,
    ToYellow,
    ToRed,
}

#[derive(Clone)]
pub struct TrafficLightRemote {
    sender: mpsc::Sender<Command>,
}

impl TrafficLightRemote {
    pub fn to_green(&self) {
        self.sender.send(Command::ToGreen).unwrap();
    }

    pub fn to_yellow(&self) {
        self.sender.send(Command::ToYellow).unwrap();
    }

    pub fn to_red(&self) {
        self.sender.send(Command::ToRed).unwrap();
    }
}

pub fn spawn_client() -> TrafficLightRemote {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        // initial state
        let mut current_state = Some(TrafficLight::Red(Red::new()));

        while let Ok(cmd) = rx.recv() {
            let Some(state) = current_state.take() else {
                println!("fatal error, invalid state");
                break;
            };

            current_state = Some(match (state, cmd) {
                //
                (TrafficLight::Green(green), Command::ToYellow) => {
                    TrafficLight::Yellow(green.to_yellow())
                }
                (TrafficLight::Yellow(yellow), Command::ToRed) => {
                    TrafficLight::Red(yellow.to_red())
                }
                (TrafficLight::Red(red), Command::ToGreen) => TrafficLight::Green(red.to_green()),
                (original_state, _bad_cmd) => {
                    println!("invalid command {:?} on {:?}", _bad_cmd, original_state);
                    original_state
                }
            });
        }

        println!("all threads done");
    });

    TrafficLightRemote { sender: tx }
}
