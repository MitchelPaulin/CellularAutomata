use gif::{Encoder, Frame, Repeat};
use rand::Rng;
use std::borrow::Cow;
use std::fs::File;
mod neighborhoods;

const WIDTH: usize = 500;
const HEIGHT: usize = 400;
const PIXELS: usize = WIDTH * HEIGHT;
const FRAMES: usize = 10;

fn real_to_int_map(val: f64) -> u8 {
    if val > 100. {
        1
    } else {
        0
    }
}

fn sum_values_in_neighborhood(
    cellular_state: &[[f64; WIDTH]; HEIGHT],
    neighborhood: &[[i32; neighborhoods::NEIGHBORHOOD_WIDTH]; neighborhoods::NEIGHBORHOOD_WIDTH],
    cell_i: usize,
    cell_j: usize,
) -> f64 {
    
    let mut sum: f64 = 0.0;
    let offset = (neighborhoods::NEIGHBORHOOD_WIDTH / 2) as i32;
    let cell_i = cell_i as i32;
    let cell_j = cell_j as i32;
    let height = HEIGHT as i32;
    let width = WIDTH as i32;

    for i in 0..neighborhoods::NEIGHBORHOOD_WIDTH {
        for j in 0..neighborhoods::NEIGHBORHOOD_WIDTH {
            if neighborhood[i][j] == 1 {
                let i = cell_i + i as i32 - offset;
                let j = cell_j + j as i32 - offset;
                if i >= 0 && j >= 0 && i < height && j < width {
                    sum += cellular_state[i as usize][j as usize];
                }
            }
        }
    }

    return sum;
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

    // initialize the states to something random in [0,1)
    let mut cellular_state: [[f64; WIDTH]; HEIGHT] = [[0.0; WIDTH]; HEIGHT];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            cellular_state[i][j] = rng.gen();
        }
    }

    let mut next_cellular_state: [[f64; WIDTH]; HEIGHT] = [[0.0; WIDTH]; HEIGHT];
    for frame_count in 0..FRAMES {
        // simulate
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                next_cellular_state[i][j] = sum_values_in_neighborhood(
                    &cellular_state,
                    &neighborhoods::neighborhood_1,
                    i,
                    j,
                );
            }
        }
        cellular_state = next_cellular_state;

        // convert cellular state to a frame
        let mut frame: [u8; PIXELS] = [0; PIXELS];
        let mut frame_i = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                frame[frame_i] = real_to_int_map(cellular_state[i][j]);
                frame_i += 1;
            }
        }

        println!("Done frame {}/{}", frame_count + 1, FRAMES);
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
