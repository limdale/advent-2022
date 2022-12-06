use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();
    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut sum = 0;
    for line in lines_vec.iter() {
        let compartment1 = &line[0..line.len()/2];
        let compartment2 = &line[line.len()/2..line.len()];

        println!("1: {}  2:{}", compartment1, compartment2);

        let char_vec: Vec<char> = line.trim().chars().collect();
        'outer: for i in 0..line.len()/2 {
            for j in 0..line.len()/2 {
                if (char_vec[i] == char_vec[j + line.len()/2]) {
                    let to_add = if char_vec[i].is_uppercase() {
                        char_vec[i] as u32 - (('A' as u32) - 27)
                    } else {
                        char_vec[i] as u32 - 'a' as u32 + 1
                    };

                    println!("common char: {} {}", char_vec[i], to_add);

                    sum += to_add;
                    break 'outer;
                }
            }
        }

    }
    println!("Sum: {}", sum);

    Ok(())
}
