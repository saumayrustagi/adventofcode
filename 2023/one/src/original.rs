use std::fs;

pub fn main() {
    let mut answer: u32 = 0;
    let input = fs::read_to_string("input.txt").expect("failed to read input");

    for line in input.lines() {
        let char_array: Vec<char> = line.chars().collect();

        let (mut start_ind, mut start_val): (i32, u32) = (-1, 0);
        let mut end_val: u32 = 0;

        for (index, value) in char_array.iter().enumerate() {
            if value.is_numeric() {
                if start_ind == -1 {
                    start_ind = index.try_into().unwrap();
                    start_val = value.to_digit(10).unwrap();
                }
                end_val = value.to_digit(10).unwrap();
            }
        }

        if start_ind != -1 {
            let temp_ans = (10 * start_val) + end_val;
            answer += temp_ans;
            // println!("{}", temp_ans);
        }
    }
    println!("Final Answer = {}", answer);
}
