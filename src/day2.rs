use std::cmp::min;
use util::load;

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
    let total_paper = sizes.iter()
        .map(|s| dim2vec(s))
        .map(|d| wrapping_paper(d))
        .fold(0, |acc, s| acc + s);
    assert_eq!(total_paper, 1586300);

    assert_eq!(ribbon(dim2vec("2x3x4")), 34);
    assert_eq!(ribbon(dim2vec("1x1x10")), 14);
    let total_ribbon = sizes.iter()
        .map(|s| dim2vec(s))
        .map(|d| ribbon(d))
        .fold(0, |acc, s| acc + s);
    assert_eq!(total_ribbon, 3737498);
}
