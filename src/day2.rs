fn a(x : Vec<[u32;2]>) -> u32{
    let mut score = 0;
    for item in x {
        score += 1+item[1]; 
        if item[0] == item[1]{
            score+=3;
        }
        else if (item[0]+1) %3 == item[1]{
        // else if ( item[0] == 0 && item[1] == 1 ) ||(  item[0] == 1 && item[1] == 2  )|| ( item[0] == 2 && item[1] == 0 ) {
            score+=6;
        }
    }
    return score;
}

fn part_b(x : Vec<[u32;2]>) -> u32{
    let mut score = 0;
    for item in x {
        score+=1 + item[1] * 3 + (2 + item[0] + item[1]) % 3
    }
    return score;
}

pub fn day2() {
    let x = include_str!("./day2.txt")
        .split("\n")
        .filter(|x| x.len()>1)
        .map(|l| [l.chars().nth(0).unwrap() as u32 - ('A' as u32),l.chars().nth(2).unwrap() as u32 - ('X' as u32)])
        .collect::<Vec<[u32;2]>>();
    // println!("{}",a(x));
    println!("{}",part_b(x));
    // x.iter().for_each(|x| println!("{} {}",x[0],x[1]));
}
    // println!("{}, {}","a b".chars().nth(0).unwrap() as u32,"a b".chars().nth(2).unwrap() as u32 );
