use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::fs::File;
use rand::Rng;
mod neighborhoods;

const WIDTH : u16 = 500;
const HEIGHT: u16 = 500;
const PIXELS : usize = (WIDTH as usize) * (HEIGHT as usize);

fn real_to_int_map(val: f64) -> u8{
    if val > 0.5 {
        1
    } else {
        0
    }
}

fn main() {
    /*
    How the color map works, each triple of values is one color R,G,B
    The first triple will be the color you get if you have a 0 for that pixel
    The second triple will be the color you get if you have a 1 for that pixel 
    And so on 
    */
    let color_map = &[
        0x00, 0x00, 0x80, 
        0x00, 0xFF, 0x00, 
        0xFF, 0xFF, 0x00, 
        0, 0, 0,
        0, 0, 0,
        0, 0, 0,
        0, 0, 0,
        0, 0, 0,
        0, 0, 0,
        0, 0, 0
    ];

    let mut frames = Vec::new();
    let mut rng = rand::thread_rng();

    //initialize the states to something random
    let mut cellular_state: [f64; PIXELS] = [0.0; PIXELS];

    for i in 0..PIXELS {
        cellular_state[i] = rng.gen();
    }

    for _ in 1..10 {
        //simulate
        for i in 0..PIXELS {
            cellular_state[i] = rng.gen();
        }
        // convert everything to values between 0 and 255
        let mut frame : [u8; PIXELS] = [0; PIXELS]; 
        for i in 0..PIXELS {
            frame[i] = real_to_int_map(cellular_state[i]);
        }
        frames.push(frame);
    }



    let mut image = File::create("beacon.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, WIDTH, HEIGHT, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for state in &frames {
        let mut frame = Frame::default();
        frame.width = WIDTH;
        frame.height = HEIGHT;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}
