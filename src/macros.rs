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
macro_rules! stack {
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
        $crate::literal::Literal($crate::stack![$($e),*])
    };
    ($($e:expr),*) => {
        $crate::literal::Literal($crate::stack![$($e),*])
    };
}
