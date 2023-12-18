fn ascii_value(c: char) -> u32 {
    c as u32
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, v| ((acc + ascii_value(v)) * 17) % 256)
}

pub fn sum_hashes(mut it: impl Iterator<Item = String>) -> u32 {
    it.next().unwrap().as_str().split(",").map(hash).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn full_example() {
        let example = indoc! {"
             rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "};
        assert_eq!(sum_hashes(example.lines().map(String::from)), 1320);
    }
}
