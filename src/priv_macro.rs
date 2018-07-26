#[macro_export]
macro_rules! impl_from {
    ($err_to_type:tt:: $err_to_variant:ident, $err_from_type:ty) => {
        impl From<$err_from_type> for $err_to_type {
            fn from(e: $err_from_type) -> Self {
                $err_to_type::$err_to_variant(e)
            }
        }
    };
}
