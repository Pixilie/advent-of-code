use std::fs;

fn main() {
    let mut result = Vec::<i32>::new();
    
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

    while !list_one.is_empty() && !list_two.is_empty() {
        // Find the minimum in list_one
        let min_one = *list_one.iter().min().unwrap();
        let min_two = *list_two.iter().min().unwrap();
        let mut difference = 0;
        
        if min_two.abs() > min_one.abs() {                           
            difference = min_two.abs() - min_one.abs();
        } else {
            difference = min_one.abs() - min_two.abs();
        }
       
        result.push(difference);

        println!("{} {} {}", min_one, min_two, difference);

        let mut removed_one = false;
        let mut removed_two = false;

        list_one.retain(|&x| {
            if !removed_one && x == min_one {
                removed_one = true;
                false
            } else { true }
        });

        list_two.retain(|&x| {
            if !removed_two && x == min_two {
                removed_two = true;
                false
            } else { true }
        });
    }

    let result_sum = result.iter().sum::<i32>();

    println!("Results: {}", result_sum);
}
