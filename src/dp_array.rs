use crate::utils::min;

pub fn min_edit_distance(s1: &str, s2: &str) -> isize {
    let m = s1.chars().count();
    let n = s2.chars().count();
    let mut min_distances: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];
    dp_array(&mut min_distances, m, n, &s1, &s2)
}

fn dp_array(min_distances: &mut Vec<Vec<usize>>, m: usize, n: usize, s1: &str, s2: &str) -> isize {
    // base case for s2 is a empty String
    // just delete each letter of s1
    for i in 1..=m {
        min_distances[i][0] = i;
    }
    // base case for s1 is a empty string
    // just insert each letter of s2
    for j in 1..=n {
        min_distances[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                min_distances[i][j] = min_distances[i - 1][j - 1];
            } else {
                min_distances[i][j] = min(
                    min_distances[i - 1][j] as isize + 1,
                    min_distances[i][j - 1] as isize + 1,
                    min_distances[i - 1][j - 1] as isize + 1,
                ) as usize;
            }
        }
    }
    min_distances[m][n] as isize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dp_array_works_base_case() {
        assert_eq!(5, min_edit_distance("", "about"));
        assert_eq!(5, min_edit_distance("about", ""));
        assert_eq!(2, min_edit_distance("你好", ""));
    }

    #[test]
    fn dp_array_works_same_str() {
        assert_eq!(0, min_edit_distance("about", "about"));
    }

    #[test]
    fn dp_array_works() {
        assert_eq!(3, min_edit_distance("horse", "ros"));
        assert_eq!(5, min_edit_distance("intention", "execution"));
        assert_eq!(4, min_edit_distance("石景山", "门头沟灵山"));
    }
}
