use adventofcode2024::input::AocInput;

fn part1(i: &mut AocInput) {
    let (mut first_values, mut second_values) = i.read_vector_of_number_pairs();
    first_values.sort();
    second_values.sort();
    let both = first_values.iter().zip(second_values.iter());
    let res = both
        .map(|(first, second)| (first - second).abs())
        .sum::<i64>();
    println!("Part1: {}", res);
}

fn part2(i: &mut AocInput) {
    let (first_values, second_values) = i.read_vector_of_number_pairs();
    let mut res: i64 = 0;
    for i in first_values {
        let occ = second_values.iter().filter(|j| i == **j).count() as i64;
        res += occ * i;
    }
    println!("Part2: {}", res);
}

fn main() {
    let mut i = AocInput::new("inputs/day01.txt");
    part1(&mut i);
    i.reset();
    part2(&mut i);
}
