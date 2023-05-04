fn get_arr(char_str : &str) -> [u32;53]{
    let mut arr = [0;53];
    for c in char_str.trim().chars(){
            if c as u32 - 'A' as u32 > 25
            {
                let diff = c as u32 - 'a' as u32;
                arr[(diff+1) as usize]+=1;
            }
            else {
                let diff = c as u32 - 'A' as u32;
                arr[(diff+27) as usize]+=1;
            }
    }
    return arr;
}
fn print_ranges(){
   // print!(" "); 
    // for i in 'a'..'{'{ print!("{i}"); }
    // for i in 'A'..'['{ print!("{i}"); }
    // println!("");
}

fn get_common_char_priority(arr1 : [u32;53],arr2 : [u32;53]) -> i32 {
    for i in 1..53 {
            if arr2[i as usize] > 0 && arr1[i as usize] > 0 {
                return i;
            }
    }
    return 0;
}

fn get_common_char_priority_2(vec_of_arr : Vec<[u32;53]>)-> i32 {
    for i in 1..53 {
        let mut flag = true;
        for arr  in &vec_of_arr {
            if arr[i as usize] == 0  {
                flag = false;
                break;
            }
    }
        if flag { return i;}
    }
    return 0;
}

fn part_a(vector_of_pairs : Vec<(&str,&str)>) -> i32{
    
    let mut ans = 0;
    for (e1,e2) in &vector_of_pairs {
        // println!("{}.{}",e1,e2); 
        let arr1=get_arr(e1);
        let arr2=get_arr(e2);
        // print_ranges();
        // arr1.iter().for_each(|x| print!("{}", x));
        ans+=get_common_char_priority(arr1, arr2);
    }
    return ans;
}
fn get_overall_common(arr : [&str;3]) -> i32{
    let (a1,a2,a3) = (get_arr(arr[0]), get_arr(arr[1]), get_arr(arr[2]));
    arr.iter().for_each(|x|  println!("{x}"));
    return get_common_char_priority_2(vec![a1,a2,a3]) ;
}
fn part_b(strs : Vec<&str>) -> i32{
   let mut ans = 0; 
   ans += strs.chunks(3)
       .filter(|arr| arr.len()==3)
       .map( |arr|  get_overall_common([arr[0],arr[1],arr[2]]) ).sum :: <i32>();
       // .map ( | (a,b) | equal_or_0(a,b)).sum :: <i32>();
    return ans;       
}

pub fn day3(){
    let input = include_str!("./day3.txt").split('\n') ;
    // let vector_of_pairs = (&input).map(|line| line.split_at(line.len()/2)).collect::<Vec<(&str,&str)>>();
    // println!("part A{}",part_a(vector_of_pairs));
    // println!(" part B {}",part_b(    input.collect::<Vec<&str>>()));
    let t = [1,2,3];
    println!("{}",t[0..3] == [t[0],t[1],t[2]]);
}
// }
