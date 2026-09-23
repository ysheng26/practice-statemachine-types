// 1. Define the states
// 2. Define the commands
// 3. Define the control pannel
// 4. spawn the control pannel

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
    ToGreen(tokio::sync::oneshot::Sender<Result<(), String>>),
    ToYellow(tokio::sync::oneshot::Sender<Result<(), String>>),
    ToRed(tokio::sync::oneshot::Sender<Result<(), String>>),
}

#[derive(Clone)]
pub struct TrafficLightRemote {
    sender: tokio::sync::mpsc::Sender<Command>,
}

impl TrafficLightRemote {
    pub async fn to_green(&self) -> Result<(), String> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender.send(Command::ToGreen(tx)).await.unwrap();
        match rx.await {
            Ok(inner_result) => inner_result,
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn to_yellow(&self) -> Result<(), String> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender.send(Command::ToYellow(tx)).await.unwrap();
        match rx.await {
            Ok(inner_result) => inner_result,
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn to_red(&self) -> Result<(), String> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender.send(Command::ToRed(tx)).await.unwrap();
        match rx.await {
            Ok(inner_result) => inner_result,
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn spawn_client() -> TrafficLightRemote {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        // initial state
        let mut current_state = Some(TrafficLight::Red(Red::new()));

        while let Some(cmd) = rx.recv().await {
            let Some(state) = current_state.take() else {
                println!("fatal error, invalid state");
                break;
            };

            current_state = Some(match (state, cmd) {
                //
                (TrafficLight::Green(green), Command::ToYellow(reply_channel)) => {
                    let _ = reply_channel.send(Ok(()));
                    TrafficLight::Yellow(green.to_yellow())
                }
                (TrafficLight::Yellow(yellow), Command::ToRed(reply_channel)) => {
                    let _ = reply_channel.send(Ok(()));
                    TrafficLight::Red(yellow.to_red())
                }
                (TrafficLight::Red(red), Command::ToGreen(reply_channel)) => {
                    let _ = reply_channel.send(Ok(()));
                    TrafficLight::Green(red.to_green())
                }
                (
                    original_state,
                    Command::ToYellow(reply_channel)
                    | Command::ToRed(reply_channel)
                    | Command::ToGreen(reply_channel),
                ) => {
                    let _ =
                        reply_channel.send(Err("invalid state command combination".to_string()));
                    original_state
                }
            });
        }

        println!("all threads done");
    });

    TrafficLightRemote { sender: tx }
}
