#[macro_export]
macro_rules! vec_string {
    ($($e:expr),*) => {vec![$($e.to_owned()), *]};
    ($($e:expr,)*) => {vec![$($e.to_owned()), *]};
}
