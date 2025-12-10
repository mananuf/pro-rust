#[macro_export]
macro_rules! call_twice {
    ($fn_name: expr, $value: expr) => {
        $fn_name($fn_name($value))
    };
}