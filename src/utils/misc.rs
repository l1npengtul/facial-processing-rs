use crate::{error::FacialProcessingError, utils::face::FaceLandmark, vector};
use cv_convert::{TryFromCv, TryIntoCv};
#[cfg(feature = "dlib")]
use dlib_face_recognition::{Point, Rectangle};
use image::imageops::FilterType;
use nalgebra::{DMatrix, Matrix, Matrix1x2, Matrix1x4, Matrix2x1, Matrix3, Matrix4x1};
use opencv::core::{Vec3, Vec3d};
use opencv::{
    calib3d::{
        rodrigues, rq_decomp3x3, solve_pnp, solve_pnp_ransac, solve_pnp_ransac_1, UsacParams,
        SOLVEPNP_AP3P, SOLVEPNP_DLS, SOLVEPNP_EPNP, SOLVEPNP_IPPE, SOLVEPNP_IPPE_SQUARE,
        SOLVEPNP_ITERATIVE, SOLVEPNP_MAX_COUNT, SOLVEPNP_SQPNP, SOLVEPNP_UPNP,
    },
    core::{
        Mat, MatExpr, MatExprTrait, MatTrait, Point2d, Point3d, ToInputArray, ToOutputArray,
        Vector, _InputArray, _InputOutputArray, CV_32F, CV_64F,
    },
    video::KalmanFilter,
    Error,
};
use std::ops::Sub;
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
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
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

impl Sub<Point2D> for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Point2D) -> Self::Output {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
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
impl From<Vec3d> for EulerAngles {
    fn from(vec: Vec3d) -> Self {
        EulerAngles {
            x: *vec.get(0).unwrap(),
            y: *vec.get(1).unwrap(),
            z: *vec.get(2).unwrap(),
        }
    }
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

#[derive(Copy, Clone)]
pub enum PnPArguments {
    NoRandsc,
    Randsc {
        iter: i32,
        reproj: f32,
        conf: f64,
        inliner: Box<opencv::Result<_InputOutputArray>>,
    },
}
impl Default for PnPArguments {
    fn default() -> Self {
        PnPArguments::NoRandsc
    }
}

pub struct PnPSolver {
    face_3d: Vector<Point3d>,
    camera_res: Point2D,
    camera_distortion: Mat,
    camera_matrix: Mat,
    pnp_mode: i32,
    pnp_extrinsic: bool,
    pnp_args: PnPArguments,
}
impl PnPSolver {
    pub fn new(
        camera_res: Point2D,
        calc_mode: Option<i32>,
        pnp_args: PnPArguments,
    ) -> Result<Self, FacialProcessingError> {
        // Fake 3D Model definition
        let face_3d: Vector<Point3d> = vector![
            Point3d::new(0.0, 0.0, 0.0),          // Nose Tip
            Point3d::new(0.0, -330.0, -65.0),     // Chin
            Point3d::new(-225.0, 170.0, -135.0),  // Left corner left eye
            Point3d::new(225.0, 170.0, -135.0),   // Right corner right eye
            Point3d::new(-150.0, -150.0, -125.0), // Mouth Corner left
            Point3d::new(150.0, -150.0, -125.0),  // Mouth Corner right
        ];

        let focal_len = camera_res.x;
        let center = Point2D::new(camera_res.x / 2, camera_res.y / 2);
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

        let pnp_mode = match calc_mode {
            Some(mode) => match mode {
                SOLVEPNP_AP3P | SOLVEPNP_DLS | SOLVEPNP_ITERATIVE | SOLVEPNP_IPPE
                | SOLVEPNP_IPPE_SQUARE | SOLVEPNP_MAX_COUNT | SOLVEPNP_SQPNP | SOLVEPNP_EPNP
                | SOLVEPNP_UPNP => mode,
                _ => {
                    return Err(FacialProcessingError::InitializeError(format!(
                        "{} is not a valid PNP setting!",
                        mode
                    )))
                }
            },
            None => SOLVEPNP_SQPNP,
        };

        Ok(PnPSolver {
            face_3d,
            camera_res,
            camera_distortion,
            camera_matrix,
            pnp_mode,
            pnp_extrinsic: false,
            pnp_args,
        })
    }

    pub fn raw_forward(&self, data: FaceLandmark) -> Result<(Mat, Mat), FacialProcessingError> {
        match &self.pnp_args {
            PnPArguments::NoRandsc => {
                let mut rvec = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let mut tvec = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let fp: Vector<Point2d> = data.pnp_landmarks().to_vec().map(|pt| Point2D::into(pt));

                match solve_pnp(
                    &self.face_3d.input_array(),
                    &fp.input_array(),
                    &self.camera_matrix.input_array(),
                    &self.camera_distortion.input_array(),
                    &mut rvec.input_array(),
                    &mut tvec.input_array(),
                    self.pnp_extrinsic,
                    self.pnp_mode,
                ) {
                    Ok(b) => {
                        if b {
                            Ok((rvec, tvec))
                        }
                        Err(FacialProcessingError::InternalError(format!(
                            "PnP Calculation failed"
                        )))
                    }
                    Err(why) => Err(FacialProcessingError::InternalError(why.to_string())),
                }
            }
            PnPArguments::Randsc {
                iter,
                reproj,
                conf,
                inliner,
            } => {
                let mut rvec = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let mut tvec = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let fp: Vector<Point2d> = data.pnp_landmarks().to_vec().map(|pt| Point2D::into(pt));
                let mut il = match *inliner {
                    Ok(mut na) => na.output_array(),
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };

                match solve_pnp_ransac(
                    &self.face_3d.input_array(),
                    &fp.input_array(),
                    &self.camera_matrix.input_array(),
                    &self.camera_distortion.input_array(),
                    &mut rvec.input_array(),
                    &mut tvec.input_array(),
                    self.pnp_extrinsic,
                    *iter,
                    *reproj,
                    *conf,
                    &mut il,
                    self.pnp_mode,
                ) {
                    Ok(b) => {
                        if b {
                            Ok((rvec, tvec))
                        }
                        Err(FacialProcessingError::InternalError(format!(
                            "PnP Calculation failed"
                        )))
                    }
                    Err(why) => Err(FacialProcessingError::InternalError(why.to_string())),
                }
            }
        }
    }

    pub fn forward(&self, data: FaceLandmark) -> Result<EulerAngles, FacialProcessingError> {
        match self.raw_forward(data) {
            Ok((rvec, _tvec)) => {
                let mut dest = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let mut jackobin = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                match rodrigues(
                    &rvec.input_array(),
                    &mut dest.output_array(),
                    &mut jackobin.output_array(),
                ) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(FacialProcessingError::InternalError(
                            "Failed to calculate rodrigues!".to_string(),
                        ))
                    }
                }

                let mut mtx_r = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let mut mtx_q = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let mut qx = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let mut qy = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };
                let mut qz = match Mat::default() {
                    Ok(m) => m,
                    Err(why) => return Err(FacialProcessingError::InternalError(why.to_string())),
                };

                match rq_decomp3x3(
                    &mut dest.input_array(),
                    &mut mtx_r.output_array(),
                    &mut mtx_q.output_array(),
                    &mut qx.output_array(),
                    &mut qy.output_array(),
                    &mut qz.output_array(),
                ) {
                    Ok(rots) => Ok(EulerAngles::from(rots)),
                    Err(why) => Err(FacialProcessingError::InternalError(why.to_string())),
                }
            }
            Err(f) => Err(f),
        }
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
