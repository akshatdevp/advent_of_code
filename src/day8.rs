use std::{  fs, collections::HashSet};
use crate::my_utils::parse_grid;

// copy pasta 4 for loops cuz lazy
fn count_visible_trees(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut count = 0;
    let mut counted_trees = HashSet::new();

    // Check trees in each row from left to right
    for i in 0..n {
        let mut max_height =-1;
        for j in 0..n {
            let tree = grid[i][j];
            if tree > max_height {
                max_height = tree;
                if !counted_trees.contains(&(i, j)) {
                    counted_trees.insert((i, j));
                    count += 1;
                }
            }
        }
        println!("lr {} : {}",i,count);
    }

    // check trees in each row from right to left
    for i in 0..n {
        let mut max_height =-1;
        for j in (0..n).rev() {
            let tree = grid[i][j];
            if tree > max_height {
                max_height = tree;
                if !counted_trees.contains(&(i, j)) {
                    counted_trees.insert((i, j));
                    count += 1;
                }
            }
        }
        println!("rl {} {}",i, count);
    }

    // Check trees in each column from top to bottom
    for j in 0..n {
        let mut max_height =-1;
        for i in 0..n {
            let tree = grid[i][j];
            if tree > max_height {
                max_height = tree;
                if !counted_trees.contains(&(i, j)) {
                    counted_trees.insert((i, j));
                    count += 1;
                }
            }
        }

        println!("ud {} {}", j,count);
    }

    // Check trees in each column from bottom to top
    for j in 0..n {
        let mut max_height =-1;
        for i in (0..n).rev() {
            let tree = grid[i][j];
            if tree > max_height {
                max_height = tree;
                if !counted_trees.contains(&(i, j)) {
                    counted_trees.insert((i, j));
                    count += 1;
                }
            }
        }

        println!("du {} {}", j ,count);
    }

    count
}

fn part_b_get_max_view_count(grid : Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut max_count = 0;
    for i in 0..n {
        for j in 0..n {
            if i == 0 || j == 0 || i == n-1 || j == n-1 {
                continue;
            }
            let mut total_count = 1;
            let mut current_count = 0;
            let curr = grid[i][j];  
            
            //left 
            for k in (0..i).rev(){
                if curr >  grid[k][j] {
                    current_count+=1;
                }
                else {
                    break;
                }
            }
            if current_count > 0 {
                total_count *= current_count;
                current_count = 0;
            }
            //right
            for k in i+1..n{
                if curr >  grid[k][j] {
                    current_count+=1;
                }
                else {
                    break;
                }
            }
            if current_count > 0 {
                total_count *= current_count;
                current_count = 0;
            }
            //up
            for k in j+1..n{
                if curr >  grid[i][k] {
                    current_count+=1;
                }
                else {
                    break;
                }
            }
            if current_count > 0 {
                total_count *= current_count;
                current_count = 0;
            }
            //down
            for k in (0..j).rev(){
                if curr >  grid[i][k] {
                    current_count+=1;
                }
                else {
                    break;
                }
            }
            if current_count > 0 {
                total_count *= current_count;
            }

            max_count = std::cmp::max(max_count,total_count)
        }
    }
    max_count
}

pub fn day8() {
    let input = fs::read_to_string("day8.txt");
    match input {
        Ok(str) => {
            let grid_list = parse_grid(str.as_str());
            println!("len {} {}", grid_list.len(), grid_list.get(0).unwrap().len());
            // println!("The number of trees are : {}",count_visible_trees(grid_list));
            println!("The number of trees are : {}",part_b_get_max_view_count(grid_list));
        },
        Err(err) => {
            print!("unreachable")
        },
    }
}


// 1234123  
// 1245136
// 5346451
