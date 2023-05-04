// incomplete need to modify to use Chinese Remainder Theorem


use itertools::Itertools;

#[derive(Debug,Clone, Copy)]
enum Operation {
    Add(Operand,Operand),
    Multiply(Operand,Operand),
}

#[derive(Debug,Clone, Copy)]
enum Operand {
    Const(i128),
    Old
}
fn get_operand_value(op : Operand, old : i128) -> i128 {
    match op {
        Operand::Const(i) => { return i; },
        Operand::Old => { return old; }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items : Vec<i128>,
    operation : Operation, 
    denominator : i128,
    throw_true : u32,
    throw_false : u32 ,
    activity : i128
}

fn create_operand(operand : &str) -> Operand {
    if operand == "old" { return Operand::Old }
    return Operand::Const(operand.parse::<i128>().unwrap()) 
}


fn create_operation(operand1 : &str, operator: &str, operand2 : &str) -> Operation {
    let op1 = create_operand(operand1);
    let op2 = create_operand(operand2);
    if operator == "*" { return Operation :: Multiply(op1, op2) }
    return Operation::Add(op1,op2 )

}


fn operate(op : Operation, old : i128) -> i128 {
    match op {
        Operation::Add(o1,o2) => { return (get_operand_value(o1, old) + get_operand_value(o2, old)) / 3 },
        Operation::Multiply(o1,o2) => { return (get_operand_value(o1, old) * get_operand_value(o2, old)) / 3; }
    }
}


pub fn day11(){
    let input = std::fs::read_to_string("day11.txt").unwrap();
    let mut all_monkeys : Vec<Monkey> = vec![];
    let raw_monkeys = input.lines();
    for raw_current_monkey in raw_monkeys.collect_vec().chunks(7)  {
        let mut iter = raw_current_monkey.iter();
        let id = iter.next().unwrap().chars().nth(7).unwrap().to_digit(10).unwrap();

        let items_str = iter.next().unwrap().split(":").nth(1).unwrap();
        let items = items_str.split(",").map(
            |item| item.trim().parse::<i128>().unwrap()).collect_vec();

        let mut operation_iterator = iter.next().unwrap().split_whitespace().rev();
        let operand1 = operation_iterator.next().unwrap();
        let operator = operation_iterator.next().unwrap();
        let operand2 = operation_iterator.next().unwrap();
        let operation = create_operation(operand1, operator, operand2) ;

        let denominator = iter.next().unwrap().split_whitespace().rev().next().unwrap().parse::<i128>().unwrap();
        let throw_true = iter.next().unwrap().split_whitespace().rev().next().unwrap().parse::<u32>().unwrap();
        let throw_false = iter.next().unwrap().split_whitespace().rev().next().unwrap().parse::<u32>().unwrap();

        let actual_current_monkey = Monkey { items,  operation, denominator, throw_true, throw_false , activity : 0};
        all_monkeys.push(actual_current_monkey);
        // println!("{} {:?} {:?} {} {} {}", id, items, operation, denominator, throw_true, throw_false);
    }
    for _ in 0..20 {
        all_monkeys = simulate(all_monkeys);
        all_monkeys.sort_by(|m_a, m_b| m_b.activity.cmp(&m_a.activity));
    }
    println!("{}",all_monkeys[0].activity * all_monkeys[1].activity);
}

fn simulate(all_monkeys : Vec<Monkey>) -> Vec<Monkey> {
    let mut new_monkeys = all_monkeys.clone();  
    for monkey_index in 0..8 {
        let monkey = new_monkeys[monkey_index].clone();
        // println!("Monkey {} {:?}",monkey_index,monkey.items);
        new_monkeys[monkey_index].activity+= monkey.items.len() as i128;
        for item in monkey.items {
            // println!("{:?}",monkey.operation);
            let transformed_item =  operate(monkey.operation,item);
            // println!("{} {}",item,transformed_item);
            if transformed_item % monkey.denominator == 0 {
                new_monkeys[ monkey.throw_true as usize].items.push(transformed_item);
            }
            else {
                new_monkeys[ monkey.throw_false as usize].items.push(transformed_item);
            }
        }
    }
    new_monkeys
   
}
