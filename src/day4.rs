use std::collections::HashSet;

/*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
*/

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum PassportField<'a> {
    BirthYear(&'a str),
    IssueYear(&'a str),
    ExpirationYear(&'a str),
    Height(&'a str),
    HairColor(&'a str),
    EyeColor(&'a str),
    PassportID(&'a str),
    CountryID(&'a str),
}

impl<'a> PassportField<'a> {
    fn new(kv: &'a str) -> Self {
        let mut split = kv.split(':');
        let key = split.next().unwrap();
        let value = split.next().unwrap();
        match key {
            "byr" => PassportField::BirthYear(value),
            "iyr" => PassportField::IssueYear(value),
            "eyr" => PassportField::ExpirationYear(value),
            "hgt" => PassportField::Height(value),
            "hcl" => PassportField::HairColor(value),
            "ecl" => PassportField::EyeColor(value),
            "pid" => PassportField::PassportID(value),
            "cid" => PassportField::CountryID(value),
            _ => panic!("invalid password field"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Passport<'a> {
    fields: Vec<PassportField<'a>>,
}

impl<'a> Passport<'a> {
    fn new(fields: &'a str) -> Self {
        let split = fields.split('\n').map(|line| line.split(' ')).flatten();
        let mut fields = vec![];
        for kv in split {
            fields.push(PassportField::new(kv));
        }
        Passport { fields }
    }

    fn is_valid_1(&self) -> bool {
        self.fields.len() >= 7
            && self.fields.iter().any(|pf| {
                if let PassportField::BirthYear(_) = pf {
                    true
                } else {
                    false
                }
            })
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::new).collect()
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day4").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_passport_field() {
        assert_eq!(
            PassportField::BirthYear("2020"),
            PassportField::new("byr:2020")
        );
    }

    #[test]
    fn test_new_passport() {
        assert_eq!(
            Passport::new("byr:2020\neyr:2030 hcl:#000000"),
            Passport {
                fields: [
                    PassportField::BirthYear("2020"),
                    PassportField::ExpirationYear("2030"),
                    PassportField::HairColor("#000000"),
                ]
                .iter()
                .cloned()
                .collect()
            }
        );
    }
}
