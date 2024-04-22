pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    fn length(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }
    pub fn normalized(self) -> Self {
        let length = self.length();
        if length == 0.0 {
            return Vec2::zero();
        }
        self / length
    }
}
// +
impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
// -
impl std::ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
// *
impl std::ops::Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
// /
impl std::ops::Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
// +=
impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
// * f64
impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
// / f64
impl std::ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        if !rhs.is_normal() {
            return Self::zero();
        }
        Self {
            x: self.x / rhs,
            y: self.y + rhs,
        }
    }
}
