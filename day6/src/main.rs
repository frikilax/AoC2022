use std::{fs, io::empty};

fn is_start_of_packet_marker(input: &str, must_not_contain: &str) -> bool {
    if input.is_empty() {
        true
    } else if must_not_contain.contains(input.chars().next().unwrap()) {
        false
    } else {
        let mut new_must_not_contain = String::from(must_not_contain);
        new_must_not_contain.push(input.chars().next().unwrap());
        is_start_of_packet_marker(&input[1..], &new_must_not_contain)
    }
}

fn main() {
    let buffer = fs::read_to_string("./input").unwrap();

    println!("buffer size is {}", buffer.len());

    for (i, char) in buffer.as_bytes().iter().enumerate() {
        if buffer[i..].len() >= 4 && is_start_of_packet_marker(&buffer[i..i+4], &String::new()[..]) {
            println!("buffer at position {} is the end of a packet marker: {}", i + 4, &buffer[i..i+4]);
            break;
        }
    }
    for (i, char) in buffer.as_bytes().iter().enumerate() {
        if buffer[i..].len() >= 14 && is_start_of_packet_marker(&buffer[i..i+14], &String::new()[..]) {
            println!("buffer at position {} is the end of a message marker: {}", i + 14, &buffer[i..i+14]);
            break;
        }
    }
}
