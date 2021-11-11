pub fn min(x: isize, y: isize, z: isize) -> isize {
    let mut min = x;
    if min > y {
        min = y;
    }
    if min > z {
        min = z;
    }
    min
}
