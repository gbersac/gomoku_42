use std::str::FromStr;

/// Like the c function atoi, trim space and tabs at the beginning of the string
/// and then parse to the T integer type the following number chars.
///
/// Return an error the first non space char is different from a number.
pub fn atoi<T>(s: &str) -> Result<T, &'static str>
        where T: FromStr
{
    let mut number_has_began = false;
    let mut nbr_str = String::new();

    for c in s.chars() {
        if !number_has_began && c.is_numeric() {
            number_has_began = true;
        }
        if number_has_began {
            if c.is_numeric() {
                nbr_str.push(c);
            } else {
                break ;
            }
        }
    }

    if nbr_str.len() > 0 {
        Ok(nbr_str.parse::<T>().ok().unwrap())
    } else {
        Err("Cannot parse the string to integer")
    }
}



#[cfg(test)]
mod test
{
    use super::*;

    fn one_atoi_test(s: &str, expected: i32) {
        let nbr: i32 = atoi(s).unwrap();
        assert!(nbr == expected);
    }

    #[test]
    fn test_atoi() {
        one_atoi_test("5", 5);
        one_atoi_test("20", 20);
        one_atoi_test("555", 555);
        one_atoi_test("    555", 555);
        one_atoi_test("    555pppp", 555);
    }

    #[test]
    #[should_panic]
    fn test_atoi2() {
        one_atoi_test("", 5);
    }

    #[test]
    #[should_panic]
    fn test_atoi3() {
        one_atoi_test("aaa", 5);
    }
}
