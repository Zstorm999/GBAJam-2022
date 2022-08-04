pub fn abs(x: i32) -> u32 {
    if x >= 0 {
        x as u32
    } else {
        (-x) as u32
    }
}
