use crate::waves::Wave;
use alsa::pcm::{Access, Format, HwParams, PCM, State};
use alsa::{Direction, ValueOr};

const SAMPLE_RATE: u32 = 44_100;

pub struct Playback {
    pcm: PCM,
}

impl Playback {
    pub fn new() -> Playback {
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
        Playback { pcm }
    }

    // fn _write_buf(&mut self, buf: &[i16]) {
    //     let io = self.pcm.io_i16().unwrap();
    //     assert_eq!(io.writei(&buf[..]).unwrap(), 1);
    // }

    pub fn write_wave<T: Wave>(&mut self, wave: &T) {
        let io = self.pcm.io_i16().unwrap();

        let num_samples = wave.duration() as u32 * SAMPLE_RATE;

        // let mut buf = Vec::<i16>::with_capacity(buf_size as usize);
        let mut i = 0;
        while i < num_samples {
            let sample = wave.wave_func(i as f32, SAMPLE_RATE) as i16;
            // could optimize to write in chunks but probably don't need to yet
            assert_eq!(io.writei(&[sample]).unwrap(), 1);
            i += 1;
        }
    }

    pub fn play(&mut self) {
        // In case the buffer was larger than 2 seconds, start the stream manually.
        if self.pcm.state() != State::Running {
            self.pcm.start().unwrap()
        };
        // Wait for the stream to finish playback.
        self.pcm.drain().unwrap();
    }



    /* Psuedocode for creating a shared channel that will be written
    to and played from immediately.


    const frame_size;

    struct Frame {
        a buffer for a single 'frame' of pcm wave data, frame size will
        control latency, smaller frame ==> lower latency, but higher cpu cost
        Frame size must also be divisable by the period of the wave 
        or there will be audible 'seams'   
    }

    fn create_playback_channel ()  -> frames_writer handle {
        let frames_writer, frames_reader = create a new tokio channel<Frame>
        let playback = create a new playback which will read from the channel 
            until the channel is empty
        return frames_writer handle
    }

    fn input_handler() (
        let frames_writer = create_playback_channel ();
        let wave = new wave()
        i = 0
        while (pressed) {
            pressed = check if pressed
            frame = new empty frame
            for x in i .. i + frame_size {
                frame.append(wave.wave_func(x))
            }
            cloned_writer = clone frames_writer
            cloned_writer.send(frame)
        }
    
    )
    
    
    
    
    */
}
