pub enum SupportedProcesses {
    Detection,
    Alignment,
    Eyesolation, // haha im so funny
    PoseEstimation,
}

pub struct Point<T: Num> {
    pub x: T,
    pub y: T,
}
impl<T: Num> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point {
            x,
            y,
        }
    }
}
impl<T: Num> Default for Point<T> {
    fn default() -> Self {
        Point {
            x: 0_f32,
            y: 0_f32,
        }
    }
}

// from tflite, etc...
pub struct BoundingBox<T: Num> {
    pub center: Point<T>,
    pub x_minumum: T,
    pub x_maximum: T,
    pub y_minumum: T,
    pub y_maximum: T,
}
impl<T: Num> BoundingBox<T> {
    // pub fn new()
}

pub struct Eyes {
    pub estimate_open: f32,
    pub left_right: f32,
    pub up_down: f32
}

pub struct Rotation {
    x: f32,
    y: f32,
    z: f32,
}