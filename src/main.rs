mod playback;
mod waves;
mod notes;

use crate::waves::sin_wave::{gen_sin_wave, gen_sin_wave_period};
use crate::waves::*;

use alsa::pcm::{Access, Format, HwParams, PCM, State};
use alsa::{Direction, ValueOr};


use std::{thread::sleep, time::Duration};

use tokio::sync::{mpsc::{self, Receiver}};

use tokio::task::futures;


#[tokio::main]
async fn main() {
    const SAMPLE_RATE: u32 = 44100;
    let wave = GenWave::new(
        waves::sin_wave::gen_sin_wave,
        GenWaveSettings {
            freq: 440.0,
            dur: 1.0,
            amp: 8000.0,
            sample_rate: 41000,
        },
    );

    let pcm = PCM::new("default", Direction::Playback, false).unwrap();
    // Open default playback device
    {
        // Set hardware parameters: 44,100 Hz / Mono / 16 bit
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(1).unwrap();
        hwp.set_rate(SAMPLE_RATE, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::s16()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();

        // Make sure we don't start the stream too early
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap())
            .unwrap();
        pcm.sw_params(&swp).unwrap();
    }

    let io = pcm.io_i16().unwrap();

    let num_samples = wave.settings.dur as u32 * SAMPLE_RATE;

    let frame_size = gen_sin_wave_period(&wave.settings);

    let num_frames = num_samples / frame_size as u32;
    let mut frame_buf = Vec::<i16>::with_capacity(frame_size as usize);

    println!("frame size: {}", frame_size);
    // generate a single 'tile-able' frame
    let mut x = 0;
    while x < frame_size {
        let sample = wave.call_wave_func(x as f32) as i16;
        frame_buf.push(sample);
        x += 1;
    }

    let mut i = 0;
    while i < num_frames {
        for sample in &frame_buf {
             assert_eq!(io.writei(&[*sample]).unwrap(), 1);
        }

        i += 1;

    }
    

    // let mut i = 0;
    // while i < num_samples {
    //     let sample = wave.call_wave_func(i as f32) as i16;
    //     // could optimize to write in chunks but probably don't need to yet
    //     assert_eq!(io.writei(&[sample]).unwrap(), 1);
    //     i += 1;
    // }

    if pcm.state() != State::Running {
        pcm.start().unwrap()
    };

    // write some more after starting playback 
    // let mut i = 0;
    // while i < num_samples {
    //     let sample = wave.call_wave_func(i as f32) as i16;
    //     // could optimize to write in chunks but probably don't need to yet
    //     assert_eq!(io.writei(&[sample]).unwrap(), 1);
    //     i += 1;
    // }


    sleep(Duration::from_secs(5));
    // pcm.drain();
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
