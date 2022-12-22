use std::fs;

fn main() {
    let mut elves_calories: Vec<i32> = vec![];
    let filepath = "./input";
    let contents = fs::read_to_string(filepath).expect("Could not read file");
    let lines = contents.split('\n');
    
    let mut calories: i32 = 0;
    for line in lines {
        calories = match line.parse::<i32>() {
            Ok(num) => calories + num,
            Err(_) => {
                elves_calories.push(calories);
                0
            }
        }
    }

    elves_calories.sort();
    elves_calories.reverse();
    let max = elves_calories.first().unwrap();
    let slice = &elves_calories[..3];
    let sum: i32 = slice.iter().sum();

    println!("max: {max}");
    println!("three most: {slice:?}");
    println!("total three: {sum}");
}
