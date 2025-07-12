mod playback;
mod waves;
mod notes;

use crate::waves::sin_wave::{SinWave};
use crate::waves::*;
use crate::playback::Playback;
use alsa::pcm::{Access, Format, HwParams, PCM, State};
use alsa::{Direction, ValueOr};


use std::{thread::sleep, time::Duration};

use tokio::sync::{mpsc::{self, Receiver}};

use tokio::task::futures;

const SAMPLE_RATE: u32 = 44_100;

#[tokio::main]
async fn main() {
    let wave = SinWave::new(
        WaveSettings::new(440.0, 8000.0)
    );

    let mut pb = Playback::new();

    pb.write_wave(&wave, 2.0);
    pb.play();
    pb.write_wave(&wave, 2.0);
    pb.drain();
}

// use std::{thread::sleep, time::Duration};

// use tokio::sync::{mpsc::{self, Receiver}};

// use tokio::task::futures;

// #[derive(Debug)]
// enum Command {
//     Play {
//         key: String,
//     },
//     Stop {
//         key: String,
//     }
// }

// async fn foo(mut rx: Receiver<Command>) {
//     let quit_string = String::from("quit") ;
//     loop {
//         println!("looping");

//         match rx.recv().await {
//             Some(message) => {
//                 match message {
//                     Command::Play { key } => println!("playing = {:?}", key),
//                     Command::Stop { key } =>{
//                         println!("stoping = {:?}", key);
//                         break;
//                     }
//                 }
//             },
//             _ => sleep(Duration::from_secs_f32(0.1)),
//         }
//     }

//     println!("DONE");
// }

// #[tokio::main]async fn main() {
//     let (sender, mut rx) = mpsc::channel(32);
//     let sender2 = sender.clone();

//     tokio::spawn(async move {
//         sender.send(Command::Play { key: (String::from("A")) }).await.unwrap();
//     });

//     tokio::spawn(async move {
//         foo(rx).await;
//     });

//     loop {
//         let mut buf: String =  String::new();
//         std::io::stdin().read_line(&mut buf);
//         let s = sender2.clone();
//         tokio::spawn(async move {
//             s.send(Command::Play { key: (String::from("A")) }).await.unwrap();
//         });
//         if buf.trim() == "quit" {
//             println!("QUITTING");
//             tokio::spawn(async move {
//                 sender2.send(Command::Stop { key: (String::from("")) }).await.unwrap();
//             });
//             break;
//         }
//     }

// }
