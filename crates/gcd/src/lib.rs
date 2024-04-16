use std::mem;

pub fn naive_gcd(x: u64, y: u64) -> u64 {
    if x % y == 0 {
        y
    } else {
        naive_gcd(y, x % y)
    }
}

pub fn binary_gcd_swap(mut x: u64, mut y: u64) -> u64 {
    if y > x {
        mem::swap(&mut x, &mut y);
    }
    // x >= y

    if y == 0u64 {
        x
    } else {
        let shift = (x | y).trailing_zeros();
        y >>= y.trailing_zeros(); // then, y is odd
        while x > 0u64 {
            x >>= x.trailing_zeros(); // then, both x,y are odd

            if y > x {
                mem::swap(&mut x, &mut y);
            }
            // x >= y
            x -= y;
            // x is even, y is odd
        }
        y << shift
    }
}

pub fn binary_gcd_swap_v2(mut x: u64, mut y: u64) -> u64 {
    if x > y {
        mem::swap(&mut x, &mut y);
    }
    // y >= x

    if x == 0u64 {
        y
    } else {
        let shift = (x | y).trailing_zeros();
        x >>= x.trailing_zeros(); // then, x is odd
        while y > 0u64 {
            y >>= y.trailing_zeros(); // then, both x, y are odd

            if x > y {
                mem::swap(&mut x, &mut y);
            }
            // y >= x
            y -= x;
            // y is even, x is odd
        }
        x << shift
    }
}

pub fn binary_gcd_noswap(mut x: u64, mut y: u64) -> u64 {
    if y == 0 {
        return x;
    }
    if x == 0 {
        return y;
    }

    let shift = (x | y).trailing_zeros();
    y >>= y.trailing_zeros(); // then, y is odd
    while x > 0u64 {
        x >>= x.trailing_zeros();
        // x is odd

        if y > x {
            let t = y - x;
            y = x;
            // y is odd
            x = t;
            // x is even
        } else {
            x -= y;
            // x is even, y is odd
        }
    }
    y << shift
}

pub fn binary_gcd_minmax(mut x: u64, mut y: u64) -> u64 {
    // if y == 0 {
    //     return x;
    // }
    // if x == 0 {
    //     return y;
    // }
    if std::cmp::min(x, y) == 0 {
        return std::cmp::max(x, y);
    }

    let shift = (x | y).trailing_zeros();
    y >>= y.trailing_zeros(); // then, y is odd
    while x > 0u64 {
        x >>= x.trailing_zeros(); // then, x is odd

        let t = x.abs_diff(y); // t is even
        y = std::cmp::min(x, y); // should be odd
        x = t;
    }
    y << shift
}

pub fn binary_gcd_recursive(mut x: u64, y: u64) -> u64 {
    if x == 0 {
        return y;
    }

    let shift = (x | y).trailing_zeros();
    x >>= x.trailing_zeros(); // x is odd

    fn inner(odd: u64, mut y: u64, shift: u32) -> u64 {
        if y == 0 || odd == y {
            odd << shift
        } else {
            y >>= y.trailing_zeros(); // then, y is odd
            inner(std::cmp::min(odd, y), y.abs_diff(odd), shift)
        }
    }

    inner(x, y, shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    use num::Integer;

    use proptest::{prop_assert_eq, proptest};

    proptest! {
        #[test]
        fn test_naive_gcd(x in proptest::num::u64::ANY, y in proptest::num::u64::ANY) {
            prop_assert_eq!(
                naive_gcd(x,y), x.gcd(&y)
            );
        }

        #[test]
        fn test_binary_gcd_swap(x in proptest::num::u64::ANY, y in proptest::num::u64::ANY) {
            prop_assert_eq!(
                binary_gcd_swap(x,y), x.gcd(&y)
            );
        }
        #[test]
        fn test_binary_gcd_swap_v2(x in proptest::num::u64::ANY, y in proptest::num::u64::ANY) {
            prop_assert_eq!(
                binary_gcd_swap_v2(x,y), x.gcd(&y)
            );
        }
        #[test]
        fn test_binary_gcd_noswap(x in proptest::num::u64::ANY, y in proptest::num::u64::ANY) {
            prop_assert_eq!(
                binary_gcd_noswap(x,y), x.gcd(&y)
            );
        }
        #[test]
        fn test_binary_gcd_minmax(x in proptest::num::u64::ANY, y in proptest::num::u64::ANY) {
            prop_assert_eq!(
                binary_gcd_minmax(x,y), x.gcd(&y)
            );
        }
        #[test]
        fn test_binary_gcd_recursive(x in proptest::num::u64::ANY, y in proptest::num::u64::ANY) {
            prop_assert_eq!(
                binary_gcd_recursive(x,y), x.gcd(&y)
            );
        }
    }
}
