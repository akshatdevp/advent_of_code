// windows uses \r\n instead of \n
pub fn day1b(){
    let mut mapped_to_lines   = include_str!("./day1.txt")
        .split("\r\n\r\n") //windows uses \r\n instead of \n
        .map(|lines_list_str| lines_list_str.lines()
             .map(|n| n.parse::<u32>().unwrap_or_default()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
    // mapped_to_lines.sort();
    // mapped_to_lines.reverse();
    println!("{}",mapped_to_lines[0][1]);
    // println!("{}",mapped_to_lines[0..3].iter().sum::<u32>());
}
