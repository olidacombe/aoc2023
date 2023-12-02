use color_eyre::{eyre::eyre, Result};

pub fn get_line_calibration_value(input: &str) -> Result<u32> {
    let first_digit = input
        .chars()
        .find(|c| c.is_numeric())
        .ok_or(eyre!("No digits!"))?
        .to_digit(10)
        .unwrap();
    let last_digit = input
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .ok_or(eyre!("No digits!"))?
        .to_digit(10)
        .unwrap();
    Ok(first_digit * 10 + last_digit)
}

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;

    #[test]
    fn example1() -> Result<()> {
        assert_eq!(get_line_calibration_value("1abc2")?, 12);
        Ok(())
    }

    #[test]
    fn example2() -> Result<()> {
        assert_eq!(get_line_calibration_value("pqr3stu8vwx")?, 38);
        Ok(())
    }

    #[test]
    fn example3() -> Result<()> {
        assert_eq!(get_line_calibration_value("a1b2c3d4e5f")?, 15);
        Ok(())
    }

    #[test]
    fn example4() -> Result<()> {
        assert_eq!(get_line_calibration_value("treb7uchet")?, 77);
        Ok(())
    }
}
