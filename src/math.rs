use bytemuck::{Pod, Zeroable};
use std::fmt;
use std::num::TryFromIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait ToFloat {
    type Float;
    fn into_float(self) -> Self::Float;
    fn from_float(float: Self::Float) -> Self;
}

impl ToFloat for u32 {
    type Float = f32;

    fn into_float(self) -> Self::Float {
        self as f32
    }

    fn from_float(float: Self::Float) -> Self {
        float as u32
    }
}

macro_rules! define_integer_type {
    ($name:ident, $inner:ty) => {
        #[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Pod, Zeroable)]
        #[repr(C)]
        pub struct $name(pub $inner);

        impl $name {
            pub const fn div(self, rhs: $inner) -> Self {
                Self(self.0 / rhs)
            }

            pub const fn into_f32(self) -> f32 {
                self.0 as f32
            }

            pub const fn into_f64(self) -> f64 {
                self.0 as f64
            }
        }

        impl From<$name> for f64 {
            fn from(value: $name) -> Self {
                value.into_f64()
            }
        }

        impl From<$name> for f32 {
            fn from(value: $name) -> Self {
                value.into_f32()
            }
        }

        impl From<$name> for $inner {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl Div for $name {
            type Output = $inner;

            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl Div<$inner> for $name {
            type Output = Self;

            fn div(self, rhs: $inner) -> Self::Output {
                Self(self.0 / rhs)
            }
        }

        impl DivAssign<$inner> for $name {
            fn div_assign(&mut self, rhs: $inner) {
                self.0 /= rhs;
            }
        }

        impl Div<f32> for $name {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self((self.0 as f32 / rhs).round() as $inner)
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                self * rhs.0
            }
        }

        impl Mul<$inner> for $name {
            type Output = Self;

            fn mul(self, rhs: $inner) -> Self::Output {
                Self(self.0 * rhs)
            }
        }

        impl Mul<f32> for $name {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self((self.0 as f32 * rhs).round() as $inner)
            }
        }

        impl MulAssign<$inner> for $name {
            fn mul_assign(&mut self, rhs: $inner) {
                self.0 *= rhs;
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl Add<$inner> for $name {
            type Output = Self;

            fn add(self, rhs: $inner) -> Self::Output {
                Self(self.0 + rhs)
            }
        }

        impl AddAssign<$inner> for $name {
            fn add_assign(&mut self, rhs: $inner) {
                self.0 += rhs;
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self - rhs.0
            }
        }

        impl Sub<$inner> for $name {
            type Output = Self;

            fn sub(self, rhs: $inner) -> Self::Output {
                Self(self.0 - rhs)
            }
        }

        impl SubAssign<$inner> for $name {
            fn sub_assign(&mut self, rhs: $inner) {
                self.0 -= rhs;
            }
        }

        impl Zero for $name {
            fn is_zero(&self) -> bool {
                self.0 == 0
            }
        }
    };
}

pub trait Zero {
    fn is_zero(&self) -> bool;
}

impl Zero for i32 {
    fn is_zero(&self) -> bool {
        *self != 0
    }
}

impl Zero for u32 {
    fn is_zero(&self) -> bool {
        *self != 0
    }
}

define_integer_type!(Dips, i32);
define_integer_type!(Pixels, i32);
define_integer_type!(UPixels, u32);

impl std::ops::Neg for Dips {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Neg for Pixels {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl TryFrom<UPixels> for Pixels {
    type Error = TryFromIntError;

    fn try_from(value: UPixels) -> Result<Self, Self::Error> {
        value.0.try_into().map(Self)
    }
}

impl TryFrom<Pixels> for UPixels {
    type Error = TryFromIntError;

    fn try_from(value: Pixels) -> Result<Self, Self::Error> {
        value.0.try_into().map(Self)
    }
}

impl Dips {
    pub const INCH: Self = Dips(2540);
    pub const CM: Self = Dips(1000);
    pub const MM: Self = Self::CM.div(10);
}

impl From<f32> for Dips {
    fn from(cm: f32) -> Self {
        Dips((cm * 1000.) as i32)
    }
}

impl ToFloat for Dips {
    type Float = f32;

    fn into_float(self) -> Self::Float {
        self.0 as f32 / 1000.
    }

    fn from_float(float: Self::Float) -> Self {
        Self::from(float)
    }
}

impl fmt::Debug for Dips {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}dip", self.0)
    }
}

impl From<f32> for Pixels {
    fn from(pixels: f32) -> Self {
        Pixels(pixels as i32)
    }
}

impl ToFloat for Pixels {
    type Float = f32;

    fn into_float(self) -> Self::Float {
        self.0 as f32
    }

    fn from_float(float: Self::Float) -> Self {
        Self::from(float)
    }
}

impl fmt::Debug for Pixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}

impl From<f32> for UPixels {
    fn from(pixels: f32) -> Self {
        Self(pixels as u32)
    }
}

impl ToFloat for UPixels {
    type Float = f32;

    fn into_float(self) -> Self::Float {
        self.0 as f32
    }

    fn from_float(float: Self::Float) -> Self {
        Self::from(float)
    }
}

impl fmt::Debug for UPixels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}px", self.0)
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Point<Unit> {
    pub x: Unit,
    pub y: Unit,
}

impl<Unit> Point<Unit> {
    pub fn new(x: impl Into<Unit>, y: impl Into<Unit>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn into_u32(self) -> Point<u32>
    where
        Unit: Into<u32>,
    {
        Point {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<T> ToFloat for Point<T>
where
    T: ToFloat,
{
    type Float = Point<T::Float>;

    fn into_float(self) -> Self::Float {
        Point {
            x: self.x.into_float(),
            y: self.y.into_float(),
        }
    }

    fn from_float(float: Self::Float) -> Self {
        Point {
            x: T::from_float(float.x),
            y: T::from_float(float.y),
        }
    }
}

impl From<appit::winit::dpi::PhysicalSize<u32>> for Size<Pixels> {
    fn from(value: appit::winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: Pixels(value.width.try_into().expect("width too large")),
            height: Pixels(value.height.try_into().expect("height too large")),
        }
    }
}

impl<Unit> Add<Size<Unit>> for Point<Unit>
where
    Unit: Add<Output = Unit>,
{
    type Output = Point<Unit>;

    fn add(self, rhs: Size<Unit>) -> Self::Output {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
        }
    }
}

impl<Unit> AddAssign<Size<Unit>> for Point<Unit>
where
    Unit: AddAssign,
{
    fn add_assign(&mut self, rhs: Size<Unit>) {
        self.x += rhs.width;
        self.y += rhs.height;
    }
}

impl<Unit> Mul<Size<Unit>> for Point<Unit>
where
    Unit: Mul<Output = Unit>,
{
    type Output = Point<Unit>;

    fn mul(self, rhs: Size<Unit>) -> Self::Output {
        Self {
            x: self.x * rhs.width,
            y: self.y * rhs.height,
        }
    }
}

impl<Unit> MulAssign<Size<Unit>> for Point<Unit>
where
    Unit: MulAssign,
{
    fn mul_assign(&mut self, rhs: Size<Unit>) {
        self.x *= rhs.width;
        self.y *= rhs.height;
    }
}

impl<Unit> Div<Size<Unit>> for Point<Unit>
where
    Unit: Div<Output = Unit>,
{
    type Output = Point<Unit>;

    fn div(self, rhs: Size<Unit>) -> Self::Output {
        Self {
            x: self.x / rhs.width,
            y: self.y / rhs.height,
        }
    }
}

impl<Unit> DivAssign<Size<Unit>> for Point<Unit>
where
    Unit: DivAssign,
{
    fn div_assign(&mut self, rhs: Size<Unit>) {
        self.x /= rhs.width;
        self.y /= rhs.height;
    }
}

impl<Unit> Zero for Point<Unit>
where
    Unit: Zero,
{
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<Unit> Neg for Point<Unit>
where
    Unit: Neg<Output = Unit>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

unsafe impl bytemuck::Pod for Point<Pixels> {}
unsafe impl bytemuck::Zeroable for Point<Pixels> {}
unsafe impl bytemuck::Pod for Point<Dips> {}
unsafe impl bytemuck::Zeroable for Point<Dips> {}
unsafe impl bytemuck::Pod for Point<i32> {}
unsafe impl bytemuck::Zeroable for Point<i32> {}
unsafe impl bytemuck::Pod for Point<f32> {}
unsafe impl bytemuck::Zeroable for Point<f32> {}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Size<Unit> {
    pub width: Unit,
    pub height: Unit,
}

impl<Unit> Size<Unit> {
    pub fn new(width: impl Into<Unit>, height: impl Into<Unit>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }

    pub fn into_u32(self) -> Size<u32>
    where
        Unit: Into<u32>,
    {
        Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }

    pub fn area(&self) -> <Unit as Mul>::Output
    where
        Unit: Mul + Copy,
    {
        self.width * self.height
    }
}

impl<Unit> Default for Size<Unit>
where
    Unit: Default,
{
    fn default() -> Self {
        Self {
            width: Unit::default(),
            height: Unit::default(),
        }
    }
}

impl<Unit> ToFloat for Size<Unit>
where
    Unit: ToFloat,
{
    type Float = Size<Unit::Float>;

    fn into_float(self) -> Self::Float {
        Size {
            width: self.width.into_float(),
            height: self.height.into_float(),
        }
    }

    fn from_float(float: Self::Float) -> Self {
        Self {
            width: Unit::from_float(float.width),
            height: Unit::from_float(float.height),
        }
    }
}

impl<Unit> Zero for Size<Unit>
where
    Unit: Zero,
{
    fn is_zero(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }
}

impl<Unit> Div<i32> for Size<Unit>
where
    Unit: Div<i32, Output = Unit>,
{
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl<Unit> Mul<i32> for Size<Unit>
where
    Unit: Mul<i32, Output = Unit>,
{
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl<Unit> From<Size<Unit>> for Point<Unit> {
    fn from(value: Size<Unit>) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}

impl<Unit> From<Point<Unit>> for Size<Unit> {
    fn from(value: Point<Unit>) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}

impl From<Size<UPixels>> for wgpu::Extent3d {
    fn from(value: Size<UPixels>) -> Self {
        Self {
            width: value.width.0,
            height: value.height.0,
            depth_or_array_layers: 1,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Rect<Unit> {
    pub origin: Point<Unit>,
    pub size: Size<Unit>,
}

impl<Unit> Rect<Unit> {
    pub const fn new(origin: Point<Unit>, size: Size<Unit>) -> Self {
        Self { origin, size }
    }

    pub fn into_u32(self) -> Rect<u32>
    where
        Point<Unit>: Into<Point<u32>>,
        Size<Unit>: Into<Size<u32>>,
    {
        Rect {
            origin: self.origin.into(),
            size: self.size.into(),
        }
    }

    pub fn intersects(&self, other: &Self) -> bool
    where
        Unit: Add<Output = Unit> + Ord + Copy,
    {
        let (
            Point {
                x: r1_left,
                y: r1_top,
            },
            Point {
                x: r1_right,
                y: r1_bottom,
            },
        ) = self.extents();
        let (
            Point {
                x: r2_left,
                y: r2_top,
            },
            Point {
                x: r2_right,
                y: r2_bottom,
            },
        ) = other.extents();
        !(r1_right <= r2_left || r2_right <= r1_left || r1_bottom <= r2_top || r1_top >= r2_bottom)
    }
}

impl<Unit> Default for Rect<Unit>
where
    Unit: Default,
{
    fn default() -> Self {
        Self {
            origin: Point::default(),
            size: Size::default(),
        }
    }
}

impl<Unit> Rect<Unit>
where
    Unit: Add<Output = Unit> + Ord + Copy,
{
    pub fn extents(&self) -> (Point<Unit>, Point<Unit>) {
        let extent = self.origin + self.size;
        (
            Point::new(self.origin.x.min(extent.x), self.origin.y.min(extent.y)),
            Point::new(self.origin.x.max(extent.x), self.origin.y.max(extent.y)),
        )
    }
}

impl<Unit> From<Size<Unit>> for Rect<Unit>
where
    Unit: Default,
{
    fn from(size: Size<Unit>) -> Self {
        Self::new(Point::default(), size)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Pod, Zeroable)]
#[repr(C)]
pub(crate) struct Ratio {
    pub div_by: u16,
    pub mul_by: u16,
}

impl Ratio {
    pub fn from_f32(scale: f32) -> Self {
        let mut best = Ratio {
            div_by: 0,
            mul_by: 0,
        };
        let mut best_diff = f32::MAX;
        for div_by in 1..=u16::MAX {
            let mul_by = (div_by as f32 * scale) as u16;
            let test = Ratio { div_by, mul_by };
            let delta = (test.into_f32() - scale).abs();
            if delta < best_diff {
                best = test;
                best_diff = delta;
                if delta < 0.00001 {
                    break;
                }
            }
        }

        best
    }

    pub fn into_f32(self) -> f32 {
        self.mul_by as f32 / self.div_by as f32
    }
}

// impl Mul<Ratio> for Dips {
//     type Output = Pixels;

//     fn mul(self, rhs: Ratio) -> Self::Output {
//         Pixels(self.0 / i32::from(rhs.div_by) * i32::from(rhs.mul_by))
//     }
// }

// impl Div<Ratio> for Pixels {
//     type Output = Dips;

//     fn div(self, rhs: Ratio) -> Self::Output {
//         Dips(self.0 / i32::from(rhs.mul_by) * i32::from(rhs.div_by))
//     }
// }

// impl Mul<Ratio> for Point<Dips> {
//     type Output = Point<Pixels>;

//     fn mul(self, rhs: Ratio) -> Self::Output {
//         Point {
//             x: self.x * rhs,
//             y: self.y * rhs,
//         }
//     }
// }

// impl Div<Ratio> for Point<Pixels> {
//     type Output = Point<Dips>;

//     fn div(self, rhs: Ratio) -> Self::Output {
//         Point {
//             x: self.x / rhs,
//             y: self.y / rhs,
//         }
//     }
// }

// impl Mul<Ratio> for Size<Dips> {
//     type Output = Size<Pixels>;

//     fn mul(self, rhs: Ratio) -> Self::Output {
//         Size {
//             width: self.width * rhs,
//             height: self.height * rhs,
//         }
//     }
// }

// impl Div<Ratio> for Size<Pixels> {
//     type Output = Size<Dips>;

//     fn div(self, rhs: Ratio) -> Self::Output {
//         Size {
//             width: self.width / rhs,
//             height: self.height / rhs,
//         }
//     }
// }

// #[test]
// fn scaling() {
//     let factor = Ratio {
//         div_by: 3,
//         mul_by: 2,
//     };
//     let dips = Dips(3);
//     let pixels = dips * factor;
//     assert_eq!(pixels.0, 2);
//     assert_eq!(pixels / factor, dips);
// }

#[test]
fn scale_factor_from_f32() {
    let factor = Ratio::from_f32(1.0 / 3.0);
    assert_eq!(
        factor,
        Ratio {
            div_by: 3,
            mul_by: 1
        }
    );
    let factor = Ratio::from_f32(16.0 / 9.0);
    assert_eq!(
        factor,
        Ratio {
            div_by: 9,
            mul_by: 16
        }
    );
    let factor = Ratio::from_f32(3. / 4.);
    assert_eq!(
        factor,
        Ratio {
            div_by: 4,
            mul_by: 3
        }
    );
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ScreenTransformation([f32; 16]);

impl ScreenTransformation {
    pub fn ortho(left: f32, top: f32, right: f32, bottom: f32, near: f32, far: f32) -> Self {
        let tx = -((right + left) / (right - left));
        let ty = -((top + bottom) / (top - bottom));
        let tz = -((far + near) / (far - near));

        // I never thought I'd write this as real code
        Self([
            // Row one
            2. / (right - left),
            0.,
            0.,
            0.,
            // Row two
            0.,
            2. / (top - bottom),
            0.,
            0.,
            // Row three
            0.,
            0.,
            -2. / (far - near),
            0.,
            // Row four
            tx,
            ty,
            tz,
            1.,
        ])
    }
}

impl ScreenTransformation {
    pub fn into_array(self) -> [f32; 16] {
        self.0
    }
}
