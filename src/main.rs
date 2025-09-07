use std::fs::read_to_string;
use std::collections::LinkedList;
use std::io::{self, Read, Write};

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
    if !archive_name.ends_with(".bf") && !archive_name.ends_with(".b") {
        panic!("Wrong archive extension");
    }

    // Save the string of data
    let archive_data: String = read_to_string(archive_name).expect("Error opening the file.");

    // We save only the data that is part of brainfuck, and we save it in a vector for easy index.
    let v_data: Vec<char> = archive_data.chars().filter(|&e| ['[',']','.',',','+','-','<','>'].contains(&e)).collect();

    // The index that we are in the vector and the vector
    let mut ind_nums: usize = 0;
    let mut v_nums: Vec<u8> = vec![0];

    // Position in v_data
    let mut ind_data: usize = 0;

    // Stack of indexes of []
    let mut stack: LinkedList<usize> = LinkedList::new();

    while ind_data < v_data.len() {
        let c: char = v_data[ind_data];
        match c {
            // We go to the next bracket if zero, else go into bracket saving the position
            '[' => if v_nums[ind_nums] != 0 {
                stack.push_back(ind_data);
            } else {
                // We go to the next bracket
                let mut cont_aux = 1;
                'next_bracket: while ind_data < v_data.len() {
                    ind_data += 1;
                    if v_data[ind_data] == '[' {
                        cont_aux += 1;
                    } else if v_data[ind_data] == ']' {
                        cont_aux -= 1;
                        if cont_aux == 0 {
                            break 'next_bracket;
                        }
                    }
                }
            },

            // We go back if it isn't zero, or go next
            ']' => if v_nums[ind_nums] != 0 {
                ind_data = *stack.back().unwrap();
            } else {
                stack.pop_back().unwrap();
            },

            // Print in screen
            '.' => print!("{}",v_nums[ind_nums] as char),

            // Take from screen
            ',' => {
                io::stdout().flush().unwrap();
                let mut buffer: [u8;1] = [0u8];
                io::stdin().read_exact(&mut buffer).expect("Error reading the data.");
                let c: u8 = buffer[0];
                if c.is_ascii() {
                    v_nums[ind_nums] = c;
                } else {
                    panic!("No ASCII character inserted.");
                }
            }

            // Add value
            '+' => v_nums[ind_nums] = v_nums[ind_nums].wrapping_add(1),

            // Substrack value
            '-' => v_nums[ind_nums] = v_nums[ind_nums].wrapping_sub(1),

            // Go down a position
            '<' => if ind_nums > 0 {ind_nums -= 1;} else {panic!("Going under position zero, exit.")},

            // Go up one position
            '>' => {
                if ind_nums == v_nums.len() - 1 {
                    v_nums.push(0);
                }
                ind_nums += 1;
            }

            _ => panic!("Something not expected happened reading the file."),
        }

        ind_data += 1;
    }
}
