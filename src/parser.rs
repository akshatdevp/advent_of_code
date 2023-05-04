fn nested_parse(){
    let mut mapped_to_lines   = include_str!("./day1.txt")
        .split("\r\n\r\n") //windows uses \r\n instead of \n
        .map(|lines_list_str| lines_list_str.lines()
             .map(|n| n.parse::<u32>().unwrap_or_default()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();
}


// fs::read_to_string instead of include_str
// lines instead of split("\n")
// split_whitespace instead of split(" ")
