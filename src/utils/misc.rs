use crate::error::FacialProcessingError;
use cv_convert::{TryFromCv, TryIntoCv};
#[cfg(feature = "dlib")]
use dlib_face_recognition::{Point, Rectangle};
use image::imageops::FilterType;
use nalgebra::{DMatrix, Matrix, Matrix1x2, Matrix1x4, Matrix2x1, Matrix3, Matrix4x1};
use opencv::core::{MatExpr, MatExprTrait};
use opencv::{
    core::{Mat, MatTrait, Point2d, Point3d, CV_32F, CV_64F},
    video::KalmanFilter,
    Error,
};
use std::{cell::Cell, path::Path};

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
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
}
impl Default for Point2D {
    fn default() -> Self {
        Point2D {
            x: 0.0_f64,
            y: 0.0_f64,
        }
    }
}
impl From<Point> for Point2D {
    fn from(pt: Point) -> Self {
        Point2D { x: pt.x, y: pt.y }
    }
}

impl Into<Point2d> for Point2D {
    fn into(self) -> Point2d {
        Point2d::new(self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct FloatingPoint2D {
    pub x: f64,
    pub y: f64,
}
impl FloatingPoint2D {
    pub fn new(x: f64, y: f64) -> Self {
        FloatingPoint2D { x, y }
    }
}
impl Default for FloatingPoint2D {
    fn default() -> Self {
        FloatingPoint2D { x: 0_f64, y: 0_f64 }
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
    pub fn low_point(&self) -> Point2D {
        Point2D::new(self.x_minumum as f64, self.y_minumum as f64)
    }
    pub fn high_point(&self) -> Point2D {
        Point2D::new(self.x_maximum as f64, self.y_maximum as f64)
    }
    pub fn center(&self) -> FloatingPoint2D {
        FloatingPoint2D::new(
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
#[cfg(feature = "dlib")]
impl Into<Rectangle> for BoundingBox {
    fn into(self) -> Rectangle {
        Rectangle {
            left: self.x_minumum as i64,
            top: self.y_maximum as i64,
            right: self.x_maximum as i64,
            bottom: self.y_minumum as i64,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct EulerAngles {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Debug)]
pub enum BackendProviders {
    OpenVTuber {
        face_detector_path: Path,
        face_alignment_path: Path,
        face_eyesolator_path: Path,
    },
    DLib {
        face_alignment_path: Path,
    },
}

#[derive(Copy, Clone, Debug)]
pub struct ImageScale {
    pub target_x: u32,
    pub target_y: u32,
    pub method: FilterType,
}

pub struct PnPSolver {
    face_3d: Vec<Point3d>,
    camera_res: Point2D,
    camera_distortion: Mat,
    camera_matrix: Mat,
    pnp_mode: i32,
    pnp_randsc: bool,
}
impl PnPSolver {
    pub fn new(
        resolution: Point2D,
        mode: i32,
        randsc: bool,
    ) -> Result<Self, FacialProcessingError> {
        // Fake 3D Model definition
        let face_3d: Vec<Point3d> = vec![
            Point3d::new(0.0, 0.0, 0.0),          // Nose Tip
            Point3d::new(0.0, -330.0, -65.0),     // Chin
            Point3d::new(-225.0, 170.0, -135.0),  // Left corner left eye
            Point3d::new(225.0, 170.0, -135.0),   // Right corner right eye
            Point3d::new(-150.0, -150.0, -125.0), // Mouth Corner left
            Point3d::new(150.0, -150.0, -125.0),  // Mouth Corner right
        ];

        let focal_len = resolution.x;
        let center = Point2D::new(resolution.x / 2, resolution.y / 2);
        let camera_matrix_na: Matrix3<f64> = Matrix3::from_row_slice(&[
            focal_len, 0.0, center.x, 0.0, focal_len, center.y, 0.0, 0.0, 1.0,
        ]);
        let camera_matrix = match Mat::try_from_cv(camera_matrix_na) {
            Ok(m) => m,
            Err(why) => {
                return Err(FacialProcessingError::InitializeError(why.to_string()));
            }
        };

        let camera_distortion = match Mat::zeros(4, 1, CV_64F) {
            Ok(mut m) => m.a(),
            Err(why) => {
                return Err(FacialProcessingError::InitializeError(why.to_string()));
            }
        };
    }
}

// pub struct SingleKalmanFilter {
//     kalman_filter: KalmanFilter,
//     state_num: i8,
//     measure_num: i8,
//     state_matrix: Cell<Matrix1x4<f32>>,
//     measurement_matrix: Cell<Matrix1x2<f32>>,
//     prediction_matrix: Cell<Matrix1x4<f32>>,
// }
//
// impl SingleKalmanFilter {
//     pub fn new() -> Result<Self, FacialProcessingError> {
//         let kalman_filter = match KalmanFilter::new(4, 2, 0, CV_32F) {
//             Ok(f) => f,
//             Err(why) => return Err(FacialProcessingError::InitializeError(why.to_string())),
//         };
//
//         let state_matrix: Matrix1x4<f32> = Matrix1x4::new(
//             0.0,
//             0.0,
//             0.0,
//             0.0,
//         );
//
//         let me
//     }
// }
