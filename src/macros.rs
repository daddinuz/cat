#[macro_export]
macro_rules! push {
    ($a:expr, $u:expr, $($v:expr),+,) => {
        $crate::push!(($a, $u), $($v),+)
    };
    ($a:expr, $u:expr, $($v:expr),+) => {
        $crate::push!(($a, $u), $($v),+)
    };
    ($a:expr, $u:expr,) => {
        ($a, $u)
    };
    ($a:expr, $u:expr) => {
        ($a, $u)
    };
}

#[macro_export]
macro_rules! rpn {
    ($($e:expr),+,) => {
        $crate::push!((), $($e),+)
    };
    ($($e:expr),+) => {
        $crate::push!((), $($e),+)
    };
    () => {
        ()
    };
}

#[macro_export]
macro_rules! quote {
    ($($e:expr),*,) => {
        $crate::quote::Quote($crate::rpn![$($e),*])
    };
    ($($e:expr),*) => {
        $crate::quote::Quote($crate::rpn![$($e),*])
    };
}
