use gif::{Encoder, Frame, Repeat};
use rand::Rng;
use std::borrow::Cow;
use std::fs::File;
mod neighborhoods;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const PIXELS: usize = WIDTH * HEIGHT;

fn real_to_int_map(val: f64) -> u8 {
    if val > 0.5 {
        1
    } else {
        0
    }
}

fn sum_values_in_neighborhood(
    cellular_state: &[[f64; WIDTH]; HEIGHT],
    neighborhood: &[[i32; neighborhoods::NEIGHBORHOOD_WIDTH]; neighborhoods::NEIGHBORHOOD_WIDTH],
    i: usize,
    j: usize,
) -> f64 {
    0.1
}

fn main() {
    /*
    How the color map works, each triple of values is one color R,G,B
    The first triple will be the color you get if you have a 0 for that pixel
    The second triple will be the color you get if you have a 1 for that pixel
    And so on
    */
    let color_map = &[
        0x00, 0x00, 0x80, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let mut frames = Vec::new();
    let mut rng = rand::thread_rng();

    //initialize the states to something random
    let mut cellular_state: [[f64; WIDTH]; HEIGHT] = [[0.0; WIDTH]; HEIGHT];

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            cellular_state[i][j] = rng.gen();
        }
    }

    for _ in 1..10 {
        //simulate
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                cellular_state[i][j] = rng.gen();
            }
        }
        // convert everything to values between 0 and 255
        let mut frame: [u8; PIXELS] = [0; PIXELS];
        let mut frame_i = 0;
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                frame[frame_i] = real_to_int_map(sum_values_in_neighborhood(
                    &cellular_state,
                    &neighborhoods::neighborhood_1,
                    i,
                    j,
                ));
                frame_i += 1;
            }
        }
        frames.push(frame);
    }

    let mut image = File::create("beacon.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for state in &frames {
        let mut frame = Frame::default();
        frame.width = WIDTH as u16;
        frame.height = HEIGHT as u16;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}
