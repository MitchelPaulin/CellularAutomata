use gif::{Encoder, Frame, Repeat};
use rand::Rng;
use std::borrow::Cow;
use std::fs::File;
mod neighborhoods;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const PIXELS: usize = WIDTH * HEIGHT;
const FRAMES: usize = 100;
const FRAME_REPEAT: usize = 1;
const NUDGE_VALUE: f32 = 0.1;

fn real_to_int_map(val: f32) -> u8 {
    (val * 6.) as u8
}

fn bound_value(val: f32) -> f32 {
    if val > 1. {
        return 1.;
    } else if val < 0. {
        return 0.;
    }
    val
}

fn sum_values_in_neighborhood(
    cellular_state: &std::vec::Vec<[f32; WIDTH]>,
    neighborhood: &[[i32; neighborhoods::NEIGHBORHOOD_WIDTH]; neighborhoods::NEIGHBORHOOD_WIDTH],
    cell_i: usize,
    cell_j: usize,
) -> f32 {
    let mut sum: f32 = 0.0;
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

    sum
}

fn main() {
    /*
    How the color map works, each triple of values is one color R,G,B
    The first triple will be the color you get if you have a 0 for that pixel
    The second triple will be the color you get if you have a 1 for that pixel
    And so on
    */
    #[rustfmt::skip]
    let color_map = &[
        0xff, 0xcc, 0x99,
        0xff, 0xcc, 0x33,
        0xff, 0xcc, 0x00,
        0xff, 0x99, 0x66,
        0xff, 0x99, 0x33,
        0xff, 0x99, 0x00,
        0xff, 0x00, 0x00,
    ];

    let mut rng = rand::thread_rng();

    // Set up gif generator
    let mut image = File::create("beacon.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, WIDTH as u16, HEIGHT as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    // initialize the states to something random in [0,1)
    let mut cellular_state = vec![[0.0; WIDTH]; HEIGHT];
    for i in HEIGHT/3..2*HEIGHT/3 {
        for j in WIDTH/3..2*WIDTH/3 {
            cellular_state[i][j] = rng.gen();
        }
    }

    let mut next_cellular_state = vec![[0.0; WIDTH]; HEIGHT];
    for frame_count in 0..FRAMES {
        // simulate
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let mut delta = 0.0;
                let mut result = sum_values_in_neighborhood(
                    &cellular_state,
                    &neighborhoods::neighborhood_1,
                    i,
                    j,
                );
                if result <= 17. {
                    delta -= NUDGE_VALUE;
                } else if 40. >= result && result <= 42. {
                    delta += NUDGE_VALUE;
                }
                result = sum_values_in_neighborhood(
                    &cellular_state,
                    &neighborhoods::neighborhood_2,
                    i,
                    j,
                );
                if (10. ..=13.).contains(&result) {
                    delta += NUDGE_VALUE;
                }
                result = sum_values_in_neighborhood(
                    &cellular_state,
                    &neighborhoods::neighborhood_3,
                    i,
                    j,
                );
                if (9. ..=21.).contains(&result) {
                    delta -= NUDGE_VALUE;
                }
                result = sum_values_in_neighborhood(
                    &cellular_state,
                    &neighborhoods::neighborhood_4,
                    i,
                    j,
                );
                if (78. ..=89.).contains(&result) {
                    delta -= NUDGE_VALUE;
                } else if result >= 108. {
                    delta -= NUDGE_VALUE;
                }
                next_cellular_state[i][j] = bound_value(cellular_state[i][j] + delta);
            }
        }

        for i in 0..HEIGHT{
            for j in 0..WIDTH {
                cellular_state[i][j] = next_cellular_state[i][j];
            }
        }

        // convert cellular state to a frame
        let mut next_frame: [u8; PIXELS] = [0; PIXELS];
        let mut frame_i = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                next_frame[frame_i] = real_to_int_map(cellular_state[i][j]);
                frame_i += 1;
            }
        }

        println!("Done frame {}/{}", frame_count + 1, FRAMES);
        for _ in 0..FRAME_REPEAT {
            let mut frame = Frame::default();
            frame.width = WIDTH as u16;
            frame.height = HEIGHT as u16;
            frame.buffer = Cow::Borrowed(&next_frame);
            encoder.write_frame(&frame).unwrap();
        }
    }
}
