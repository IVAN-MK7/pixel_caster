/// ternary (c# alias for °statement° ? °true case° : °false case° )
/// if $test is true return $true_expr, else return $false_expr
// Example of how to use macro:
// ternary!(foo == bar => println!("it is true, they are equal"); println!("it is false, they are not equal"));
#[macro_export]
macro_rules! ternary {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test { $true_expr } else { $false_expr }
    };
}

// keeps adding units of $add to $val until it reaches $limit or $add has no more units
#[macro_export]
macro_rules! add_limited {
    ($val:expr, $add:expr, $limit:expr) => {{
        if $add >= 0 {
            $val.saturating_add($add).max($limit)
        } else {
            $val.saturating_add($add).min($limit)
        }
    }};
}
