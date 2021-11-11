#[derive(Copy, Clone)]
enum Operation {
    Insert(char),
    Delete(char),
    Replace(char, char),
    Skip(char),
}

#[derive(Copy, Clone)]
struct Node {
    opt: Operation,
    distance: usize,
}

fn min_node(options: &[Node; 3]) -> Node {
    let mut min = options[0];

    if min.distance > options[1].distance {
        min = options[1];
    }

    if min.distance > options[2].distance {
        min = options[2];
    }
    min
}

pub fn min_edit_distance(s1: &str, s2: &str) -> isize {
    let m = s1.chars().count();
    let n = s2.chars().count();

    if s1 == "" {
        for j in (0..n).rev() {
            println!("s[0] insert {}", s2.chars().nth(j).unwrap());
        }
        return n as isize;
    }

    if s2 == "" {
        for i in (0..m).rev() {
            println!("s[i] delete {}", s1.chars().nth(i).unwrap());
        }
        return m as isize;
    }

    if s1 == s2 {
        println!("The two strings are identical, need no editing!");
        return 0;
    }

    let mut min_distances: Vec<Vec<Node>> = vec![
        vec![
            Node {
                distance: 0,
                opt: Operation::Skip('.')
            };
            n + 1
        ];
        m + 1
    ];
    dp_array_opt(&mut min_distances, m, n, &s1, &s2)
}

fn print_opt(min_distances: &Vec<Vec<Node>>, row: usize, col: usize, s1: &str, s2: &str) -> () {
    let mut i = row;
    let mut j = col;

    println!("Change s1={} to s2={}", s1, s2);
    while i != 0 && j != 0 {
        match min_distances[i][j].opt {
            Operation::Insert(c) => {
                println!("s1[{}]: insert {}", i - 1, c);
                j -= 1;
            }
            Operation::Delete(c) => {
                println!("s1[{}]: delete {}", i - 1, c);
                i -= 1;
            }
            Operation::Replace(c1, c2) => {
                println!("s1[{}]: replace {} with {}", i - 1, c1, c2);
                i -= 1;
                j -= 1;
            }
            Operation::Skip(c) => {
                println!("s1[{}]: skip {}", i - 1, c);
                i -= 1;
                j -= 1;
            }
        }
    }

    // if there are still some chars in s1, delete them all
    while i > 0 {
        println!("s1[{}]: delete {}", i - 1, s1.chars().nth(i - 1).unwrap());
        i -= 1;
    }
    // if there are still some chars in s2, insert them all
    while j > 0 {
        println!("s1[0]: insert {}", s2.chars().nth(j - 1).unwrap());
        j -= 1;
    }
}

fn dp_array_opt(
    min_distances: &mut Vec<Vec<Node>>,
    m: usize,
    n: usize,
    s1: &str,
    s2: &str,
) -> isize {
    for i in 1..=m {
        min_distances[i][0] = Node {
            opt: Operation::Delete(s1.chars().nth(i - 1).unwrap()),
            distance: i,
        }
    }

    for j in 1..=n {
        min_distances[0][j] = Node {
            opt: Operation::Insert(s2.chars().nth(j - 1).unwrap()),
            distance: j,
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                min_distances[i][j] = Node {
                    opt: Operation::Skip(s1.chars().nth(i - 1).unwrap()),
                    distance: min_distances[i - 1][j - 1].distance,
                };
            } else {
                let options = [
                    Node {
                        distance: min_distances[i - 1][j].distance + 1,
                        opt: Operation::Delete(s1.chars().nth(i - 1).unwrap()),
                    },
                    Node {
                        distance: min_distances[i][j - 1].distance + 1,
                        opt: Operation::Insert(s2.chars().nth(j - 1).unwrap()),
                    },
                    Node {
                        distance: min_distances[i - 1][j - 1].distance + 1,
                        opt: Operation::Replace(
                            s1.chars().nth(i - 1).unwrap(),
                            s2.chars().nth(j - 1).unwrap(),
                        ),
                    },
                ];
                min_distances[i][j] = min_node(&options);
            }
        }
    }
    println!("Gonna print the operations...");
    print_opt(&min_distances, m, n, s1, s2);
    min_distances[m][n].distance as isize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dp_array_opt_works_base_case() {
        assert_eq!(5, min_edit_distance("", "about"));
        assert_eq!(5, min_edit_distance("about", ""));
        assert_eq!(2, min_edit_distance("你好", ""));
    }

    #[test]
    fn dp_array_opt_works_same_str() {
        assert_eq!(0, min_edit_distance("about", "about"));
    }

    #[test]
    fn dp_array_opt_works() {
        assert_eq!(3, min_edit_distance("horse", "ros"));
        assert_eq!(5, min_edit_distance("intention", "execution"));
        assert_eq!(4, min_edit_distance("石景山", "门头沟灵山"));
    }
}
