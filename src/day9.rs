use std::collections::{HashMap, HashSet};

fn diff_x(position_head : [i32;2],position_tail : [i32;2]) -> i32 {
    return position_head[0] - position_tail[0];
}
fn diff_y(position_head : [i32;2],position_tail : [i32;2]) -> i32 {
    return position_head[1] - position_tail[1];
}

fn get_new_tail( x_diff : i32, y_diff : i32, hd : [i32;2], tail : [i32;2]) -> [i32;2]{
    if x_diff == 2 {
        return [ hd[0] - 1, hd[1] ]
    }
    if x_diff == -2 {
        return [ hd[0] + 1, hd[1] ]
    }

    if y_diff == -2 {
        return [ hd[0] , hd[1] + 1 ]
    }
    if y_diff == 2 {
        return [ hd[0] , hd[1] - 1 ]
    }
    return tail;

}

fn position_before_tail (part : u8 ,dx : i32, dy : i32 , position_head : [i32;2], position_tail : [i32;2]) -> [i32;2] {
    if part == 1 {
        return position_head;
    }
    let x_diff = diff_x(position_head,position_tail);
    let y_diff = diff_y(position_head,position_tail);
    position_before_tail(part-1, dx, dy, [position_head[0] + dx, position_head[1] + dy],get_new_tail(x_diff,y_diff,position_head,position_tail))
}

pub fn day9(){
    let input = std::fs::read_to_string("day9.txt");
    let part = 1; // change to 10 to make it part 2
    match input {
        Ok(ok_input) => {
            let split_input = ok_input.lines();
            let mut position_head = [0,0];
            let mut position_tail= [0,0];
            let direction_operation = HashMap::from([
            ("U", [1,0]),
            ("D", [-1,0]),
            ("L", [0,-1]),
            ("R", [0,1]),
            ]);            
            let mut answer_set = HashSet :: from( [[0,0]] );
            split_input.for_each(
                    |direction_with_count| {
                        // println!("{} line", direction_with_count);
                        let mut d = direction_with_count.split(' ');
                        let actual_direction = d.nth(0).unwrap(); 
                        let direction_count  = String :: from(d.rev().nth(0).unwrap()).parse::<i32>().unwrap();

                        // let direction_count  = String :: from(d.nth(1).unwrap()).parse::<i32>().unwrap();
                        for _i in 0..direction_count {
                            let [dx,dy] = direction_operation[actual_direction];
                            position_head = [position_head[0] + dx, position_head[1] + dy];
                            let x_diff = diff_x(position_head,position_tail);
                            let y_diff = diff_y(position_head,position_tail);
                            let before_tail = position_before_tail(part, dx, dy, position_head, position_tail);
                            position_tail = get_new_tail(x_diff,y_diff,before_tail,position_tail);
                            answer_set.insert(position_tail); 
                        }
                            
                    }
                ); 
            println!("{}",answer_set.len());
                
        },
        Err(_err) => {
            print!("unn rich bel")
        },
    }
}
