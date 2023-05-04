pub fn day10() {
    let input = std::fs::read_to_string("day10.txt");
    let marks= [20,60,100,140,180,220];
    match input {
        Ok(is_ok) => {
            let parsed_input = is_ok.lines();
            let mut current_cycle = 0;
            let mut total_value = 1;
            let mut answer = 0;
            parsed_input.for_each(
                  |line|
                  {
                      let mut line_iter = line.split_whitespace();
                      let _operation = line_iter.next().unwrap();
                      let op_value  = line_iter.next();
                      match op_value {
                            Some(value) => {
                                current_cycle+=2; 
                                if marks.contains(&current_cycle) || marks.contains(&(&current_cycle -1))  {
                                    let mut actual_cycle = current_cycle;
                                    if actual_cycle % 10 != 0 {
                                        actual_cycle-=1;
                                    }
                                    // answer +=actual_cycle*total_value;
                                }
                                total_value += value.parse::<i32>().unwrap();   
                            }
                            None => {
                                current_cycle+=1;
                                if [20,60,100,140,180,220].contains(&current_cycle) { 
                                    // answer += current_cycle*total_value; 
                                }
                            }

                      }
                  }
                );
                print!("part 1 : {}",answer);

        },
        Err(_) => {},
    }  
}
