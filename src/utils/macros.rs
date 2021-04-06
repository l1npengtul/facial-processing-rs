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
