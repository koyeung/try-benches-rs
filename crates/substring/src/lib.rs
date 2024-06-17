/// skip and take on [[std::str::Chars]
/// https://github.com/letmutex/rust-substring
pub fn substring_v1(text: &str, start: usize, end: usize) -> String {
    text.chars().skip(start).take(end - start).collect()
}

/// Find the byte indices using single [str::char_indices]
/// https://github.com/letmutex/rust-substring
pub fn substring_v4(text: &str, start: usize, end: usize) -> &str {
    if start == end {
        return "";
    }
    let mut iter = text.char_indices();
    let start_byte_idx = iter.nth(start).map(|(i, _)| i).unwrap_or(0);
    let end_byte_idx = iter
        .nth(end - start - 1)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    &text[start_byte_idx..end_byte_idx]
}

/// Left-right char skipping on [std::str::Chars]
/// https://github.com/letmutex/rust-substring
pub fn substring_v5(text: &str, start: usize, end: usize) -> &str {
    let mut iter = text.chars();
    let mut left = 0;
    let mut right = iter.clone().count();
    loop {
        if left < start {
            iter.next();
            left += 1;
        }
        if right > end {
            iter.next_back();
            right -= 1;
        }
        if left == start && right <= end {
            break;
        }
    }
    iter.as_str()
}

#[cfg(test)]
mod tests {
    use proptest::prelude::{prop_assert_eq, prop_compose, proptest, Just};

    use super::*;

    prop_compose! {
        fn string_and_start_end()
            (s in ".{0,5}")
            (start in 0..=s.len() + 3, end in 0..=s.len() + 3, s in Just(s))
            -> (String, usize, usize) {
            (s, start, end)
        }
    }

    proptest! {
        #[test]
        fn test_substring_string((s, start, end) in string_and_start_end()) {
            let char_count = s.chars().count();
            let start = start.min(char_count);
            let end = end.max(start).min(char_count);

            let expected = s.chars().skip(start).take(end - start).collect::<String>();

            // function return String
            prop_assert_eq!(substring_v1(&s, start,end), expected);
        }

        #[test]
        fn test_substring_str((s, start, end) in string_and_start_end()) {
            let char_count = s.chars().count();
            let start = start.min(char_count);
            let end = end.max(start).min(char_count);

            let expected = s.chars().skip(start).take(end - start).collect::<String>();

            // functions return &str
            prop_assert_eq!(substring_v4(&s, start,end), &expected);
            prop_assert_eq!(substring_v5(&s, start,end), &expected);
        }
    }
}
