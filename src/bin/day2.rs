use std::cmp::min;
use aoc_2015::load;

fn dim2vec(dim: &str) -> Vec<i32> {
    dim.split('x')
        .map(|d| d.parse())
        .map(|d| d.unwrap())
        .collect()
}

fn wrapping_paper(dim: Vec<i32>) -> i32 {
    let a = dim[0] * dim[1];
    let b = dim[1] * dim[2];
    let c = dim[2] * dim[0];
    2 * a + 2 * b + 2 * c + min(a, min(b, c))
}

fn ribbon(dim: Vec<i32>) -> i32 {
    let a = 2 * dim[0] + 2 * dim[1];
    let b = 2 * dim[1] + 2 * dim[2];
    let c = 2 * dim[2] + 2 * dim[0];
    min(a, min(b, c)) + dim[0] * dim[1] * dim[2]
}

fn main() {
    assert_eq!(wrapping_paper(dim2vec("2x3x4")), 58);
    assert_eq!(wrapping_paper(dim2vec("1x1x10")), 43);

    let sizes = load("data/day2.txt");
    let total_paper: i32 = sizes.iter()
        .map(|s| dim2vec(s))
        .map(wrapping_paper)
        .sum();
    assert_eq!(total_paper, 1_586_300);

    assert_eq!(ribbon(dim2vec("2x3x4")), 34);
    assert_eq!(ribbon(dim2vec("1x1x10")), 14);
    let total_ribbon: i32 = sizes.iter()
        .map(|s| dim2vec(s))
        .map(ribbon)
        .sum();
    assert_eq!(total_ribbon, 3_737_498);
}
