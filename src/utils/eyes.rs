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
use opencv::{
    core::{Mat, MatTrait, CV_64F},
    Error,
};
use std::cmp::{max, min};

fn isolate_iris(
    eye_img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    threshold: f64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let kernel: Matrix3<u8> = nalgebra::Matrix3::zeros();
    let kernel_cv = Mat::try_from_cv(kernel).unwrap();
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Eye {
    points: [Point2D; 6],
    side: LeftRight,
    center_pt: Option<Point2D>,
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
        let top_left_crop_start = {
            Point2D::new(
                *points.get(0).unwrap().x(),
                max(*points.get(1).unwrap().y(), *points.get(2).unwrap().y()),
            )
        };
        let bottom_left_crop_size = {
            let tmp = Point2D::new(
                *points.get(3).unwrap().x(),
                min(*points.get(4).unwrap().y(), *points.get(5).unwrap().y()),
            );
            tmp - top_left_crop_start
        };
        let cropped_img = {
            let imagebuffer: ImageBuffer<Rgb<u8>, Vec<u8>> = crop_imm(
                image,
                top_left_crop_start.x() as u32,
                top_left_crop_start.y() as u32,
                bottom_left_crop_size.x() as u32,
                bottom_left_crop_size.y() as u32,
            )
            .to_image();
            imagebuffer
        };

        Eye {
            points: landmarks.eye_landmarks(side),
            side,
            center_pt: None,
            ear_aspect_ratio,
        }
    }
}
