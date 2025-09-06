use std::fs::read_to_string;

fn main() {
    // We collect the data
    let v_args: Vec<String> = std::env::args().collect();

    // If not correct size, we exit
    if v_args.len() != 2 {
        panic!("Wrong number of arguments");
    }

    // Name of the archive
    let archive_name: String = v_args[1].clone();

    // Wrong extension
    if !archive_name.ends_with(".bf") {
        panic!("Wrong archive extension");
    }

    // Save the string of data
    let archive_data: String = read_to_string(archive_name).expect("Error opening the file.");

    // We save only the data that is part of brainfuck, and we save it in a vector for easy index.
    let v_data: Vec<char> = archive_data.chars().filter(|&e| [',',',','+','-','<','>'].contains(&e)).collect();

    // The index that we are in the vector and the vector
    let mut index: usize = 0;
    let mut v_nums: Vec<u8> = vec![0];

}
