use crate::vec::Vec2;

pub trait Display {
    fn plot(&mut self, x: i64, y: i64, symbol: char);
}

pub fn draw_pixel(display: &mut dyn Display, position: &Vec2, pixel: char) {
    let Vec2 { x, y } = position;
    display.plot(*x as i64, *y as i64, pixel);
}

pub fn draw_line(display: &mut dyn Display, x0: i64, x1: i64, y0: i64, y1: i64) {
    let deltax: i64 = (x1 - x0).abs();
    let deltay: i64 = (y1 - y0).abs();
    let mut error: i64 = 0;
    let deltaerr: i64 = deltay + 1;
    let mut y: i64 = y0;
    let mut diry: i64 = y1 - y0;
    if diry > 0 {
        diry = 1;
    }
    if diry < 0 {
        diry = -1;
    }
    for x in x0..x1 {
        display.plot(x, y, '@');
        error = error + deltaerr;
        if error >= deltax + 1 {
            y = y + diry;
            error = error - (deltax + 1);
        }
    }
}

pub fn draw_circle(display: &mut dyn Display, x1: i64, y1: i64, radius: i64) {
    let mut x: i64 = 0;
    let mut y: i64 = radius;
    let mut delta = 1 - 2 * y;
    let mut _error = 0;
    while y >= x {
        display.plot(x1 + x, y1 + y, '@');
        display.plot(x1 + x, y1 - y, '@');
        display.plot(x1 - x, y1 + y, '@');
        display.plot(x1 - x, y1 - y, '@');
        display.plot(x1 + y, y1 + x, '@');
        display.plot(x1 + y, y1 - x, '@');
        display.plot(x1 - y, y1 + x, '@');
        display.plot(x1 - y, y1 - x, '@');
        _error = 2 * (delta + y) - 1;
        if (delta < 0) && (_error <= 0) {
            x += 1;
            delta += 2 * x + 1;
            continue;
        }
        if (delta > 0) && (_error > 0) {
            y -= 1;
            delta -= 2 * y + 1;
            continue;
        }
        x += 1;
        y -= 1;
        delta += 2 * (x - y);
    }
}