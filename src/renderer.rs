use crate::util::{Color, Draw, HEIGHT, PI, WIDTH};

pub struct Renderer {
    pub buffer: Vec<u32>,
}
impl Renderer {
    pub fn draw_pixel(&mut self, position: (u32, u32), color: Color) {
        if position.0 < WIDTH as u32
            && position.0 > 0
            && position.1 < HEIGHT as u32
            && position.1 > 0
        {
            self.buffer[(position.0 + position.1 * WIDTH as u32) as usize] = color as _;
        }
    }

    pub fn clear(&mut self, color: Color) {
        for iter in 0..HEIGHT * WIDTH {
            self.buffer[iter] = color as _;
        }
    }
}
