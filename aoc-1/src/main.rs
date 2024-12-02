mod input;

fn main() {
    let mut left_column: Vec<i32> = Vec::new();
    let mut right_comun: Vec<i32> = Vec::new();

    input::INPUT
        .lines()
        .map(|line| line.split_whitespace())
        .for_each(|mut line| {
            left_column.push(line.next().unwrap().parse().unwrap());
            right_comun.push(line.next().unwrap().parse().unwrap());
        });

    left_column.sort();
    right_comun.sort();

    let result_1 = left_column
        .iter()
        .zip(right_comun.iter())
        .fold(0, |acc, (left, right)| {
            acc + (left - right).abs()
        });


    println!("result 1 is: {}", result_1);

    let result_2 = left_column
        .iter()
        .fold(0, |acc, left| {
            acc + right_comun.iter().filter(|x| x == &left).count() as i32
        });

    println!("result 2 is: {}", result_2);
}
