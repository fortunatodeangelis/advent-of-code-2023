// Use the std library to read the file
use std::{fs, str::Lines};

fn main() {
    // read data from file
    let data = fs::read_to_string("input/data.txt").expect("Unable to read file");

    // Iterate over the lines of the file with the Lines struct from the std library
    // I use mut because I use the next() method that change the state of the struct
    let mut lines: Lines<'_> = data.lines();

    // Get variable for sum
    let mut sum = 0;

    // iterate over the lines and get the line
    while let Some(line) = lines.next() {
        // Extract the number from the line
        let string = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
        // if the string is length 1 multiply by 11
        if string.len() == 1 {
            // add the number to the sum
            sum += string.parse::<i32>().unwrap() * 11;
        } 
        // if the string is length > 1
        else if string.len() > 1 {
            // Get First and last number of the string
            let (first, last) = (string.chars().next().unwrap(), string.chars().last().unwrap());
            format!("{}{}", first, last).parse::<i32>().unwrap();

            // Merge of the two numbers for example first = 1 and last = 5, merge = 15
            let merge = format!("{}{}", first, last).parse::<i32>().unwrap();

            // add the merge to the sum
            sum += merge;
        }
    }
    // Print the sum
    println!("Sum: {}", sum);
}