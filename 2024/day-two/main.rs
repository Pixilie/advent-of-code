fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 {
            decreasing = false;
        } else if diff < 0 {
            increasing = false;
        }
    }

    increasing || decreasing
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let report_list: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|elt| elt.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let safe_count = report_list.iter().filter(|report| is_safe(report)).count();

    println!("Number of safe reports: {}", safe_count);

    let safe_count_dampener = report_list
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();

    println!("Number of safe reports: {safe_count_dampener}");
}
