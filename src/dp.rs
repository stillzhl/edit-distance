use crate::utils::min;

pub fn min_edit_distance(s1: &str, s2: &str) -> isize {
    let i = s1.chars().count() as isize - 1;
    let j = s2.chars().count() as isize - 1;
    dp(i, j, s1, s2)
}

fn dp(i: isize, j: isize, s1: &str, s2: &str) -> isize {
    if i == -1 {
        return j + 1;
    }
    if j == -1 {
        return i + 1;
    }

    if s1.chars().nth(i as usize) == s2.chars().nth(j as usize) {
        return dp(i - 1, j - 1, s1, s2);
    } else {
        return min(
            dp(i - 1, j, s1, s2) + 1,
            dp(i, j - 1, s1, s2) + 1,
            dp(i - 1, j - 1, s1, s2) + 1,
        );
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dp_rcr_works_base_case() {
        assert_eq!(5, min_edit_distance("", "about"));
        assert_eq!(5, min_edit_distance("about", ""));
    }

    #[test]
    fn dp_rcr_works_same_str() {
        assert_eq!(0, min_edit_distance("about", "about"));
    }

    #[test]
    fn dp_rcr_works() {
        assert_eq!(3, min_edit_distance("horse", "ros"));
        assert_eq!(5, min_edit_distance("intention", "execution"));
    }
}
