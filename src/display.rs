use crate::vec::Vec2;

pub trait Display {
    fn plot(&mut self, x: i64, y: i64, symbol: char);
}

pub fn draw_pixel(display: &mut dyn Display, position: &Vec2, pixel: char) {
    let Vec2 { x, y } = position;
    display.plot(*x as i64, *y as i64, pixel);
}
