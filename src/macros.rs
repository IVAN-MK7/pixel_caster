
/// ternary (c# alias for °statement° ? °true case° : °false case° )
/// if $test is true return $true_expr, else return $false_expr
#[macro_export]
macro_rules! ternary {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}
// example of how to use macro
// ternary!(foo != bar => println!("it is true"); println!("it is false"));

// keeps subtracting units of $sub from $val until it reaches $limit or $sub has no more units
#[macro_export]
macro_rules! sub_limited {
    ($val:expr, $sub:expr, $limit:expr) => {
        if $val >= $sub {
            $val - $sub
        }
        else {
            let mut v = $val;
            let mut s = $sub;
            while s > 0 {
                if v > $limit {
                    v -= 1; // sub
                    s -= 1;
                }
                else { break;}
            }
            v
        }
    }
}
// keeps adding units of $add to $val until it reaches $limit or $add has no more units
#[macro_export]
macro_rules! add_limited {
    ($val:expr, $add:expr, $limit:expr) => {
        {
            let mut v = $val;
            let mut s = $add;
            if s < 0 {
                while s < 0 {
                    if v > $limit {
                        v -= 1; // sub
                        s += 1;
                    }
                    else { break;}
                }
            }
            else {
                while s > 0 {
                    if v > $limit {
                        v += 1; // add
                        s -= 1;
                    }
                    else { break;}
                }
            }
            v
        }
    }
}
