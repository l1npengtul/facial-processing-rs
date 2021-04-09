use crate::{
    error::FacialProcessingError,
    pt_abs, pt_dist, pt_mdpt,
    utils::{
        face::FaceLandmark,
        misc::{LeftRight, Point2D},
    },
};
use cv_convert::{TryFromCv, TryIntoCv};
use image::{imageops::crop_imm, ImageBuffer, Rgb};
use nalgebra::Matrix3;
use opencv::core::{Point2i, ToInputArray, ToOutputArray, BORDER_CONSTANT, BORDER_DEFAULT};
use opencv::imgproc::{
    bilateral_filter, erode, morphology_default_border_value, threshold, THRESH_BINARY,
};
use opencv::{
    core::{Mat, MatTrait, CV_64F},
    Error,
};
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Eye {
    points: [Point2D; 6],
    side: LeftRight,
    center_pt: Point2D,
    ear_aspect_ratio: f64,
}
impl Eye {
    pub fn new(
        landmarks: FaceLandmark,
        side: LeftRight,
        image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Self {
        let points = landmarks.eye_landmarks(side).to_vec();
        // calculate the ratio
        let vertical = {
            let dist_a = pt_dist!(*points.get(1).unwrap(), *points.get(5).unwrap());
            let dist_b = pt_dist!(*points.get(2).unwrap(), *points.get(4).unwrap());
            (dist_a + dist_b)
        };
        let mut horizontal = pt_dist!(*points.get(0).unwrap(), *points.get(3).unwrap());
        if horizontal == 0_f64 {
            horizontal = 1.0_f64;
        }
        let ear_aspect_ratio = vertical / (2.0 * horizontal);
        // get center location
        // get cropped image
        // let top_left_crop_start = {
        //     Point2D::new(
        //         *points.get(0).unwrap().x(),
        //         max(*points.get(1).unwrap().y(), *points.get(2).unwrap().y()),
        //     )
        // };
        // let bottom_left_crop_size = {
        //     let tmp = Point2D::new(
        //         *points.get(3).unwrap().x(),
        //         min(*points.get(4).unwrap().y(), *points.get(5).unwrap().y()),
        //     );
        //     tmp - top_left_crop_start
        // };
        // let cropped_img = {
        //     let imagebuffer: ImageBuffer<Rgb<u8>, Vec<u8>> = crop_imm(
        //         image,
        //         top_left_crop_start.x() as u32,
        //         top_left_crop_start.y() as u32,
        //         bottom_left_crop_size.x() as u32,
        //         bottom_left_crop_size.y() as u32,
        //     )
        //     .to_image();
        //     imagebuffer
        // };

        // this is hacky code but we'll just assume the iris is at the center of the eye
        // im too lazy for this

        let center_pt = pt_mdpt![
            *points.get(0).unwrap(),
            *points.get(1).unwrap(),
            *points.get(2).unwrap(),
            *points.get(3).unwrap(),
            *points.get(4).unwrap(),
            *points.get(5).unwrap()
        ];

        Eye {
            points: landmarks.eye_landmarks(side),
            side,
            center_pt,
            ear_aspect_ratio,
        }
    }

    pub fn eye_ear_ratio(&self) -> f64 {
        self.ear_aspect_ratio
    }

    pub fn iris_position(&self) -> Point2D {
        self.center_pt
    }

    pub fn side(&self) -> LeftRight {
        self.side
    }

    pub fn points(&self) -> [Point2D; 6] {
        self.points
    }
}
