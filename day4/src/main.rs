use std::fs;

fn main() {
    let contents = fs::read_to_string("./input").unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut count: i32 = 0;

    for line in lines {
        if let [elf_one, elf_two] = &line.split(',').collect::<Vec<&str>>()[..] {
            let mut elf_one_lower: i32 = 0;
            let mut elf_one_upper: i32 = 0;
            let mut elf_two_lower: i32 = 0;
            let mut elf_two_upper: i32 = 0;
            if let [elf_one_left, elf_one_right] = &elf_one.split('-').collect::<Vec<&str>>()[..] {
                elf_one_lower = elf_one_left.parse().unwrap();
                elf_one_upper = elf_one_right.parse().unwrap();
            } else {
                println!("Could not parse values");
            }
            if let [elf_two_left, elf_two_right] = &elf_two.split('-').collect::<Vec<&str>>()[..] {
                elf_two_lower = elf_two_left.parse().unwrap();
                elf_two_upper = elf_two_right.parse().unwrap();
            } else {
                println!("Could not parse values");
            }

            if  elf_two_lower >= elf_one_lower && elf_two_lower <= elf_one_upper ||
                elf_two_upper >= elf_one_lower && elf_two_upper <= elf_one_upper ||
                elf_one_lower >= elf_two_lower && elf_one_lower <= elf_two_upper ||
                elf_one_upper >= elf_two_lower && elf_one_upper <= elf_two_upper {
                count += 1;
            }

        } else {
            println!("Could not parse values");
        }
    }

    println!("final count: {count}");
}
