#[macro_export]
macro_rules! impl_from {
    ($err_to:expr, $err_from_type:ty) => {
        impl From<$err_from_type> for Error {
            fn from(e: $err_from_type) -> Error {
                $err_to(e)
            }
        }
    };
}
