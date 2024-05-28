#[derive(Clone, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

pub trait Display {
    fn plot(&mut self, x: i64, y: i64, color: &Color);
}

pub fn pixel(display: &mut dyn Display, x: i64, y: i64, color: &Color) {
    display.plot(x, y, color);
}

pub fn line(display: &mut dyn Display, x0: i64, x1: i64, y0: i64, y1: i64, color: &Color) {
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
        display.plot(x, y, color);
        error = error + deltaerr;
        if error >= deltax + 1 {
            y = y + diry;
            error = error - (deltax + 1);
        }
    }
}

pub fn circle(display: &mut dyn Display, x1: i64, y1: i64, radius: i64, color: &Color) {
    let mut x: i64 = 0;
    let mut y: i64 = radius;
    let mut delta: i64 = 1 - 2 * y;
    let mut _error: i64 = 0;
    while y >= x {
        display.plot(x1 + x, y1 + y, color);
        display.plot(x1 + x, y1 - y, color);
        display.plot(x1 - x, y1 + y, color);
        display.plot(x1 - x, y1 - y, color);
        display.plot(x1 + y, y1 + x, color);
        display.plot(x1 + y, y1 - x, color);
        display.plot(x1 - y, y1 + x, color);
        display.plot(x1 - y, y1 - x, color);
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
