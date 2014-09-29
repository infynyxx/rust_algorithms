pub fn gcd(x: int, y: int) -> int {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
