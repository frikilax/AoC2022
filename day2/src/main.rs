use std::fs;

fn main() {
    let contents = fs::read_to_string("./input").unwrap();
    let lines = contents.split('\n');
    
    let mut total_score: i32 = 0;

    for line in lines {
        if let [opponent, result] = line.split_whitespace().take(2).collect::<Vec<&str>>()[..] {
            let score = match result {
                "X" => 0 + {
                    match opponent {
                        "A" => 3,
                        "B" => 1,
                        "C" => 2,
                        _ => panic!("Wrong opponent value!")
                    }
                },
                "Y" => 3 + {
                    match opponent {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => panic!("Wrong opponent value!")
                    }
                },
                "Z" => 6 + {
                    match opponent {
                        "A" => 2,
                        "B" => 3,
                        "C" => 1,
                        _ => panic!("Wrong opponent value!")
                    }
                },
                _ => panic!("Wrong result value!")
            };
            total_score += score;
        } else {
            panic!("WTF is happening ?!");
        }

    }

    println!("{total_score}");

}