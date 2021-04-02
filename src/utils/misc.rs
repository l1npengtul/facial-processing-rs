#[cfg(feature = "dlib")]
use dlib_face_recognition::Rectangle;
use image::imageops::FilterType;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum LeftRight {
    Left,
    Right,
}
impl Default for LeftRight {
    fn default() -> Self {
        LeftRight::Left
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SupportedProcesses {
    Detection,
    Alignment,
    Eyesolation, // haha im so funny
    PoseEstimation,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct FloatingPoint {
    pub x: f64,
    pub y: f64,
}
impl FloatingPoint {
    pub fn new(x: f64, y: f64) -> Self {
        FloatingPoint { x, y }
    }
}
impl Default for FloatingPoint {
    fn default() -> Self {
        FloatingPoint { x: 0_f64, y: 0_f64 }
    }
}

// from tflite, etc...
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct BoundingBox {
    pub x_minumum: i32,
    pub x_maximum: i32,
    pub y_minumum: i32,
    pub y_maximum: i32,
}
impl BoundingBox {
    // pub fn new()
    pub fn low_point(&self) -> Point {
        Point::new(self.x_minumum, self.y_minumum)
    }
    pub fn high_point(&self) -> Point {
        Point::new(self.x_maximum, self.y_maximum)
    }
    pub fn center(&self) -> FloatingPoint {
        FloatingPoint::new(
            (self.x_maximum - self.x_minumum) / 2_f64 as f64,
            (self.y_maximum - self.y_minumum) / 2_f64 as f64,
        )
    }
}

#[cfg(feature = "dlib")]
impl From<Rectangle> for BoundingBox {
    fn from(r: Rectangle) -> Self {
        BoundingBox {
            x_minumum: r.left as i32,
            x_maximum: r.right as i32,
            y_minumum: r.bottom as i32,
            y_maximum: r.top as i32,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Rotation {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Copy, Clone, Debug)]
pub enum BackendProviders {
    OpenVTuber,
    DLib,
}

#[derive(Copy, Clone, Debug)]
pub struct ImageScale {
    pub target_x: u32,
    pub target_y: u32,
    pub method: FilterType,
}
