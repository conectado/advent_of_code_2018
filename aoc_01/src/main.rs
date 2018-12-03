extern crate aoc_01;

fn main() {
    let file = aoc_01::read_file("test.txt").unwrap();
    let total: i32 = aoc_01::sum_buffer(file).unwrap();
    let file = aoc_01::read_file("test2.txt").unwrap();
    let twiced: i32 = aoc_01::find_first_twice(file).unwrap();
    println!("{}", total);
    println!("Twiced {}", twiced);
}
