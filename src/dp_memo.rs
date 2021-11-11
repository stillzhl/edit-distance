use crate::utils::min;

use std::collections::HashMap;

type MinDistances = HashMap<(isize, isize), isize>;

pub fn min_edit_distance(s1: &str, s2: &str) -> isize {
    let i = s1.chars().count() as isize - 1;
    let j = s2.chars().count() as isize - 1;
    let mut memo: MinDistances = HashMap::new();
    dp_memo(&mut memo, i, j, s1, s2)
}

fn dp_memo(memo: &mut MinDistances, i: isize, j: isize, s1: &str, s2: &str) -> isize {
    if i == -1 {
        return j + 1;
    }
    if j == -1 {
        return i + 1;
    }

    match memo.get(&(i, j)) {
        Some(min_distance) => {
            return *min_distance;
        }
        None => {
            if s1.chars().nth(i as usize) == s2.chars().nth(j as usize) {
                let min_distance = dp_memo(memo, i - 1, j - 1, s1, s2);
                memo.insert((i, j), min_distance);
                return min_distance;
            } else {
                let min_distance = min(
                    dp_memo(memo, i - 1, j, s1, s2) + 1,
                    dp_memo(memo, i, j - 1, s1, s2) + 1,
                    dp_memo(memo, i - 1, j - 1, s1, s2) + 1,
                );
                memo.insert((i, j), min_distance);
                return min_distance;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dp_memo_works_base_case() {
        assert_eq!(5, min_edit_distance("", "about"));
        assert_eq!(5, min_edit_distance("about", ""));
        assert_eq!(2, min_edit_distance("你好", ""));
    }

    #[test]
    fn dp_memo_works_same_str() {
        assert_eq!(0, min_edit_distance("about", "about"));
    }

    #[test]
    fn dp_memo_works() {
        assert_eq!(3, min_edit_distance("horse", "ros"));
        assert_eq!(5, min_edit_distance("intention", "execution"));
    }
}
