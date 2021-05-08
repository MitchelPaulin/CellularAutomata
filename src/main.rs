use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::fs::File;
mod neighborhoods;

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
    
    const WIDTH : u16 = 1920;
    const HEIGHT: u16 = 1080;
    const PIXELS : usize = (WIDTH as usize) * (HEIGHT as usize);

    let mut beacon_states = Vec::new();
    let mut count = 0;
    for _ in 1..10 {
        let mut arr: [u8; PIXELS] = [0; PIXELS];
        for i in 0..PIXELS {
            arr[i] = count;
        }
        count = (count + 1) % 255;
        beacon_states.push(arr);
    }



    let mut image = File::create("beacon.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, WIDTH, HEIGHT, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for state in &beacon_states {
        let mut frame = Frame::default();
        frame.width = WIDTH;
        frame.height = HEIGHT;
        frame.buffer = Cow::Borrowed(&*state);
        encoder.write_frame(&frame).unwrap();
    }
}
