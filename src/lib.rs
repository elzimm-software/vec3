use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

pub type Point3 = Vec3;

pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            e: [0.0, 0.0, 0.0],
        }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e: [e0, e1, e2],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.e[0]*rhs.e[0]+self.e[1]*rhs.e[1]+self.e[2]*self.e[2]
    }

    #[inline]
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::from(
            self.e[1]*rhs.e[2]-self.e[2]*rhs.e[1],
            self.e[2]*rhs.e[0]-self.e[0]*rhs.e[2],
            self.e[0]*rhs.e[1]-self.e[1]*rhs.e[0]
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::from(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl<T> Index<T> for Vec3
where
    T: Into<usize>,
{
    type Output = f64;

    fn index(&self, index: T) -> &Self::Output {
        &self.e[index.into()]
    }
}

impl<T> Index<T> for &Vec3
where
    T: Into<usize>,
{
    type Output = f64;

    fn index(&self, index: T) -> &Self::Output {
        &self.e[index.into()]
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl<T> MulAssign<T> for Vec3
where
    T: Into<f64>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs_f64 = rhs.into();
        self.e[0] *= rhs_f64;
        self.e[1] *= rhs_f64;
        self.e[2] *= rhs_f64;
    }
}

impl<T> DivAssign<T> for Vec3
where
    T: Into<f64>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs_f64 = rhs.into();
        if rhs_f64 == 0.0 {
            panic!("Attempt to divide by zero")
        }
        self.e[0] /= rhs_f64;
        self.e[1] /= rhs_f64;
        self.e[2] /= rhs_f64;
    }
}

impl fmt::Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl<'a> Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::from(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
    }
}

impl<'a> Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::from(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
    }
}

impl<'a> Mul<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::from(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl<T> Mul<T> for &Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let rhs_f64 = rhs.into();
        Vec3::from(self.e[0] * rhs_f64, self.e[1] * rhs_f64, self.e[2] * rhs_f64)
    }
}

impl Mul<&Vec3> for f64
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for i32
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for u8
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for u32
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<&Vec3> for f32
{
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl<T> Div<T> for &Vec3
where
    T: Into<f64>,
{
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        (1.0 / rhs.into()) * self
    }
}
