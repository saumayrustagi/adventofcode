use std::fs;

pub fn main() {
    let mut answer: u32 = 0;
    let input = fs::read_to_string("input.txt").expect("failed to read input");

    for line in input.lines() {
        let mut start_val: Option<u32> = None;
        let mut end_val: Option<u32> = None;

        for value in line.chars() {
            if let Some(digit) = value.to_digit(10) {
                if start_val.is_none() {
                    start_val = Some(digit);
                }
                end_val = Some(digit);
            }
        }

        // match start_val {
        //     Some(_) => {
        //         let temp_ans = (10 * start_val.unwrap()) + end_val.unwrap();
        //         answer += temp_ans;
        //         // println!("{}", temp_ans);
        //     }
        //     _ => {}
        // }

		if let (Some(start), Some(end)) = (start_val, end_val){
                let temp_ans = (10 * start) + end;
                answer += temp_ans;
                // println!("{}", temp_ans);
		}
    }
    println!("Final Answer = {}", answer);
}
