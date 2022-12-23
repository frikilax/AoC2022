use std::fs;

fn parse_table_input(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    
    let last_line = & input[input.len() - 1];
    for (i, character) in last_line.iter().enumerate() {
        if *character == ' ' {
            continue;
        } else {
            let index = character.to_digit(10).unwrap();
            let index: usize = usize::try_from(index).unwrap();
            output.resize(index, Vec::new());
            for line in input.iter().rev() {
                if line[i] != ' ' {
                    output[index - 1].push(line[i]);
                }
            }
        }
    }

    output
}

fn parse_action_input(input: &mut Vec<&str>) -> Vec<(u32, usize, usize)> {
    let mut output: Vec<(u32, usize, usize)> = Vec::new();

    for line in input {
        let mut words = line.split(' ');
        let quantity: u32 = words.nth(1).unwrap().parse().unwrap();
        let row_init: usize = words.nth(1).unwrap().parse().unwrap();
        let row_end: usize = words.nth(1).unwrap().parse().unwrap();
        output.push((quantity, row_init, row_end));
    }

    output
}


fn cratemover_9000(container: &mut [Vec<char>], action: (u32, usize, usize)) {
    for i in 0..action.0 {
        let value = container[action.1 - 1].pop().unwrap();
        container[action.2 - 1].push(value);
    }
}


fn cratemover_9001(container: &mut [Vec<char>], action: (u32, usize, usize)) {
    let mut bundle: Vec<char>  = Vec::new();
    for i in 0..action.0 {
        let value = container[action.1 - 1].pop().unwrap();
        bundle.push(value);
    }

    bundle.reverse();

    container[action.2 - 1].append(&mut bundle);
}


fn main() {
    let contents = fs::read_to_string("./input").unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut table_input: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<&str> = Vec::new();
    let mut table_finished: bool = false;

    for line in lines {
        if line.is_empty() {
            table_finished = true;
            continue;
        }
        if ! table_finished {
            table_input.push(line.chars().collect());
        } else {
            instructions.push(line);
        }
    }

    let mut table1 = parse_table_input(&table_input);
    let mut table2 = parse_table_input(&table_input);

    let actions = parse_action_input(&mut instructions);

    for action in actions {
        cratemover_9000(&mut table1, action);
        cratemover_9001(&mut table2, action);
    }

    let mut result1 : String = String::new();
    let mut result2 : String = String::new();
    for line in table1 {
        result1.push(*line.last().unwrap());
    }
    for line in table2 {
        result2.push(*line.last().unwrap());
    }

    println!("(step 1) top crates are {result1}");
    println!("(step 2) top crates are {result2}");
}
