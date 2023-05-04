
pub fn parse_grid(input : &str) -> Vec<Vec<i32>> {
    input.lines().map(
        |line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
        ).collect()
}
