#[macro_export]
macro_rules! vector {
    ( $( $elem:expr ),* ) => {
        {
            let mut vector = opencv::core::Vector::new();
            $(
                vector.push($elem);
            )*
            vector
        }
    };
}

#[macro_export]
macro_rules! pt_mdpt {
    ( $( $pt:expr ),* ) => {
        {
            let mut x = 0.0_f64;
            let mut y = 0.0_f64;
            let mut cnt: usize = 0;
            $(
                x += $pt.x();
                y += $pt.y();
                cnt += 1;
            )*
            if cnt == 0 {
                cnt = 1;
            }
            let new_point: $crate::utils::misc::Point2D = $crate::utils::misc::Point2D::new(x/(cnt as f64), y/(cnt as f64));
            new_point
        }
    };
}

#[macro_export]
macro_rules! pt_abs {
    ($pt:expr) => {{
        let new_point: $crate::utils::misc::Point2D =
            $crate::utils::misc::Point2D::new($pt.x().abs(), $pt.y().abs());
        new_point
    }};
}

#[macro_export]
macro_rules! pt_dist {
    ($pt1:expr, $pt2:expr) => {{
        let dist: f64 = ((($pt1.x() - $pt2.x()) * ($pt1.x() - $pt2.x())) as f64
            + ($pt1.y() - $pt2.y()) * ($pt1.y() - $pt2.y()))
        .sqrt();
        dist
    }};
}

#[macro_export]
macro_rules! mat_init {
    () => {{
        opencv::core::Mat::default()
    }};
}
