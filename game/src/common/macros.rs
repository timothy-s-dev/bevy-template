#[macro_export]
macro_rules! get_single_or_return {
    ($q:expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_mut_or_return {
    ($q:expr) => {
        match $q.get_single_mut() {
            Ok(m) => m,
            _ => return,
        }
    };
}

pub use get_single_mut_or_return;
pub use get_single_or_return;
