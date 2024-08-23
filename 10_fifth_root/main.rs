fn main() {
    let odd_nums = get_100_odd_nums();
    let squared_nums = get_squares(odd_nums);
    let sum = get_sum(squared_nums);
    let fifth_root = get_fifth_root(sum);
    println!(
        "the Fifth root of the sum of the squares of the first 100 ODD numbers is {}",
        fifth_root,
    );
}

fn get_100_odd_nums() -> Vec<u32> {
    let mut odd_nums = Vec::new();
    let mut num = 0;
    while odd_nums.len() < 100 {
        if num % 2 != 0 {
            odd_nums.push(num);
        }
        num += 1;
    }

    odd_nums
}

fn get_squares(mut nums: Vec<u32>) -> Vec<u32> {
    for num in &mut nums {
        *num *= *num;
    }

    nums
}

fn get_sum(nums: Vec<u32>) -> u32 {
    let sum = nums.into_iter().fold(0, |acc, n| acc + n);
    sum
}

fn get_fifth_root(num: u32) -> f64 {
    f64::powf(num.into(), 1.0 / 5.0)
}