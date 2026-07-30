// Spreadsheet-style column letters, ie A, B, ..., Z, AA, AB, ...
// This is a bijective base-26 encoding, not a plain base-26 one, since there
// is no letter for zero: A is 1, Z is 26, AA is 27.

pub fn column_letters_to_index(letters: &str) -> Option<i32> {
    if letters.is_empty() {
        return None;
    }

    let mut result: i64 = 0;
    for c in letters.chars() {
        if !c.is_ascii_alphabetic() {
            return None;
        }
        let digit_value = (c.to_ascii_lowercase() as u8 - b'a') as i64 + 1;
        result = result * 26 + digit_value;
    }
    Some((result - 1) as i32)
}

pub fn column_index_to_letters(index: i32) -> String {
    let mut n = index as i64 + 1;
    let mut letters = Vec::new();
    while n > 0 {
        let remainder = ((n - 1) % 26) as u8;
        letters.push((b'A' + remainder) as char);
        n = (n - 1) / 26;
    }
    letters.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_single_letters() {
        assert_eq!(column_letters_to_index("a"), Some(0));
        assert_eq!(column_letters_to_index("z"), Some(25));
        assert_eq!(column_index_to_letters(0), "A");
        assert_eq!(column_index_to_letters(25), "Z");
    }

    #[test]
    fn converts_double_letters() {
        assert_eq!(column_letters_to_index("aa"), Some(26));
        assert_eq!(column_letters_to_index("ab"), Some(27));
        assert_eq!(column_index_to_letters(26), "AA");
        assert_eq!(column_index_to_letters(27), "AB");
    }

    #[test]
    fn is_case_insensitive() {
        assert_eq!(column_letters_to_index("Ab"), column_letters_to_index("ab"));
    }

    #[test]
    fn rejects_non_alphabetic_input() {
        assert_eq!(column_letters_to_index(""), None);
        assert_eq!(column_letters_to_index("a1"), None);
    }

    #[test]
    fn round_trips_a_range_of_columns() {
        for index in 0..1000 {
            let letters = column_index_to_letters(index);
            assert_eq!(column_letters_to_index(&letters), Some(index));
        }
    }
}
