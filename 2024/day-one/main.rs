use std::fs;

fn main() {
    // Input parsing
    let content = fs::read_to_string("./input")
        .expect("Should have been able to read the file");

    let split_content = content.split("\n");

    let mut list_one = Vec::<i32>::new();
    let mut list_two = Vec::<i32>::new();

    for value in split_content {
        let line: Vec<&str> = value.split_whitespace().collect();
        if line.len() >= 2 {
            let num1: i32 = line[0].parse().expect("Failed to parse string to integer");
            let num2: i32 = line[1].parse().expect("Failed to parse string to integer");
            list_one.push(num1);
            list_two.push(num2);
        }
    }

    list_one.sort();
    list_two.sort();

    // Part 1
    let mut result_sum = Vec::<i32>::new();
    let mut index = 0;

    while index < list_one.len() {
        let distance = list_one[index] - list_two[index];       
        result_sum.push(distance.abs());
        index += 1;
    }
    
    let result_part_one = result_sum.iter().sum::<i32>();
    println!("Part 1: {result_part_one}");

    // Part 2
    let mut result_similarity = Vec::<i32>::new();
       
    for value in list_one {
        let mut n = 0;
        
        for elt in &list_two {
            if value == *elt { n += 1; }    
        }

        result_similarity.push(value * n);
    }

    let result_part_two = result_similarity.iter().sum::<i32>();
    println!("Part 2: {result_part_two}");
}
