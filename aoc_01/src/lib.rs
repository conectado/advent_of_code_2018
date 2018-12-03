use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_file(filename: &str) -> Result<BufReader<File>, std::io::Error> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}

pub fn find_first_twice(buffer: BufReader<File>) -> Result<i32, std::num::ParseIntError> {
    let mut mapping = HashMap::new();
    let mut accum: i32 = 0;
    let mut accum_vec: Vec<i32> = Vec::new();

    mapping.insert(accum.clone(), true);

    for line in buffer.lines() {
        let val = &line.unwrap().parse()?;
        accum += val;
        accum_vec.push(val.clone());
        if let Some(_) = mapping.get(&accum) {
            println!("second time {}", accum.to_string());
            return Ok(accum);
        } else {
            mapping.insert(accum.clone(), true);
        }
    }

    loop {
        for val in accum_vec.iter() {
            accum += val;
            if let Some(_) = mapping.get(&accum) {
                println!("second time {}", accum.to_string());
                return Ok(accum);
            } else {
                mapping.insert(accum.clone(), true);
            }
        }
    }
}

pub fn sum_buffer(buffer: BufReader<File>) -> Result<i32, std::num::ParseIntError> {
    let mut accum: i32 = 0;

    for line in buffer.lines() {
        accum += &line.unwrap().parse()?;
    }

    Ok(accum)
}
