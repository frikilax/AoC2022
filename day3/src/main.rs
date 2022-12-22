use std::fs;

fn main() {
    let contents = fs::read_to_string("./input").unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut priorities_sum: u32 = 0;

    let mut three_chunk_iterator = lines.chunks_exact(3);

    for three_lines in three_chunk_iterator {

        match three_lines {
            [first, second, third] => {
                for letter in first.chars() {
                    if second.contains(letter) && third.contains(letter) {
                        let value: u32 = letter.into();
                        let result: u32 = match value > 90 {
                            true => value - 96,
                            false => value - 38,
                        };
        
                        println!("{first} <> {second} <> {third} = {letter} ({result})");
                        priorities_sum += result;
                        break;
                    }
                }
            },
            _ => panic!("Unhandled case!")
        }

    }

    println!("{priorities_sum}");
}