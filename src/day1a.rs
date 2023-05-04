pub fn day1a(){
    let max_value : u32   = include_str!("./day1.txt")
        .split("\r\n\r\n") //windows uses \r\n instead of \n
        .map(|lines_list_str| lines_list_str.lines()
             .map(|n| n.parse::<u32>().unwrap()).sum()).max().unwrap();
        // .collect::<Vec<u32>>().;
    println!("Day 1A : {}",max_value);

}

// .map(|number| number.parse::<u32>()
//      .unwrap_or_default()).sum :: <u32>())
