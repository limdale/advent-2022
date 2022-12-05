use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let lines = BufReader::new(file).lines();
    let lines_vec: Vec<String> = lines.map(|s| s.unwrap()).collect();

    let mut weight_list: Vec<i32> = Vec::new();
    let mut sum = 0;
    for line in lines_vec.iter() {
        if line.trim().is_empty() {
            weight_list.push(sum);
            sum = 0;
        } else {
            let weight = line.trim().parse::<i32>().unwrap();
            sum += weight;
        }
    }

    weight_list.sort_by(|a, b| b.cmp(a));

    println!("Largest: {:?}", weight_list[0]);
    println!("Sum of 3 largest: {:?}", weight_list[0] + weight_list[1] + weight_list[2]);

    Ok(())
}
