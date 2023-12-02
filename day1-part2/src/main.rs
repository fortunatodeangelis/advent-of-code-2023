// Use the std library to read the file
use std::{fs, str::Lines};
use std::collections::HashMap;

fn main() {
    // read data from file
    let data = fs::read_to_string("input/data.txt").expect("Unable to read file");

    // Iterate over the lines of the file with the Lines struct from the std library
    // I use mut because I use the next() method that change the state of the struct
    let mut lines: Lines<'_> = data.lines();

    // Get variable for sum
    let mut sum = 0;

    // Create an HashMap for the char
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    // iterate over the lines and get the line
    while let Some(line) = lines.next() {
        // Get only char AZaz 
        let string = line.chars().collect::<String>();
        let mut temp_string = String::new();

        // Iterate char by char of the string
        for c in string.chars() {
            // Add the char to the temp_string
            temp_string.push(c);

            // Iterate over the map
            for (key, value) in &map {
                // If the key is in the temp_string
                if temp_string.contains(key) {
                    // Add the char to the temp_string to cover cases for example "twone"
                    temp_string.push(c);
                    // Replace the key by the value
                    temp_string = temp_string.replace(key, &value.to_string());
                }
            }
        }

        // extract the number from the temp_string
        temp_string = temp_string.chars().filter(|c| c.is_digit(10)).collect::<String>();
        
        // Get First and last number of the string
        let (first, last) = (temp_string.chars().next().unwrap(), temp_string.chars().last().unwrap());
        format!("{}{}", first, last).parse::<i32>().unwrap();

        // Merge of the two numbers for example first = 1 and last = 5, merge = 15
        let merge = format!("{}{}", first, last).parse::<i32>().unwrap();
        
        // add the merge to the sum
        sum += merge;
    }
    // Print the sum
    println!("Sum: {}", sum);
}