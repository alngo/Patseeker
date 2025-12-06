pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a < b { a } else { b }
}
