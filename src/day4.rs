use std::{collections::HashMap, str::FromStr};

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum PassportField {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

impl PassportField {
    fn new(key: &str) -> Self {
        match key {
            "byr" => PassportField::BirthYear,
            "iyr" => PassportField::IssueYear,
            "eyr" => PassportField::ExpirationYear,
            "hgt" => PassportField::Height,
            "hcl" => PassportField::HairColor,
            "ecl" => PassportField::EyeColor,
            "pid" => PassportField::PassportID,
            "cid" => PassportField::CountryID,
            _ => panic!("invalid passport field"),
        }
    }

    fn is_valid_value(&self, value: &str) -> bool {
        match self {
            PassportField::BirthYear if is_valid_birth_year(value) => true,
            PassportField::IssueYear if is_valid_issue_year(value) => true,
            PassportField::ExpirationYear if is_valid_expiration_year(value) => true,
            PassportField::Height if is_valid_height(value) => true,
            PassportField::HairColor if is_valid_hair_color(value) => true,
            PassportField::EyeColor if is_valid_eye_color(value) => true,
            PassportField::PassportID if is_valid_passport_id(value) => true,
            PassportField::CountryID => true,
            _ => false,
        }
    }
}

fn is_valid_birth_year(value: &str) -> bool {
    value.len() == 4 && is_in_range(value, 1920, 2002)
}

fn is_valid_issue_year(value: &str) -> bool {
    value.len() == 4 && is_in_range(value, 2010, 2020)
}

fn is_valid_expiration_year(value: &str) -> bool {
    value.len() == 4 && is_in_range(value, 2020, 2030)
}

fn is_valid_height(value: &str) -> bool {
    if value.ends_with("cm") {
        value.len() == 5 && is_in_range(&value[..3], 150, 193)
    } else if value.ends_with("in") {
        value.len() == 4 && is_in_range(&value[..2], 59, 76)
    } else {
        false
    }
}

fn is_valid_hair_color(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && u32::from_str_radix(&value[1..], 16).is_ok()
}

fn is_valid_eye_color(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn is_valid_passport_id(value: &str) -> bool {
    value.len() == 9 && value.chars().all(char::is_numeric)
}

fn is_in_range<T>(value: &str, min: T, max: T) -> bool
where
    T: FromStr + PartialOrd,
{
    if let Ok(x) = value.parse::<T>() {
        if x >= min && x <= max {
            return true;
        }
    }
    return false;
}

#[derive(Debug, PartialEq)]
struct Passport<'a> {
    fields: HashMap<PassportField, &'a str>,
}

const REQUIRED_FIELDS: [PassportField; 7] = [
    PassportField::BirthYear,
    PassportField::IssueYear,
    PassportField::ExpirationYear,
    PassportField::Height,
    PassportField::HairColor,
    PassportField::EyeColor,
    PassportField::PassportID,
];

impl<'a> Passport<'a> {
    fn new(fields: &'a str) -> Self {
        let kv_iter = fields.split('\n').map(|line| line.split(' ')).flatten();
        let mut fields = HashMap::new();
        for kv in kv_iter {
            let mut split = kv.split(':');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            fields.insert(PassportField::new(key), value);
        }
        Passport { fields }
    }

    fn has_required_fields(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|field| self.fields.contains_key(field))
    }

    fn is_valid(&self) -> bool {
        if !self.has_required_fields() {
            return false;
        }

        for (field, &value) in &self.fields {
            if !field.is_valid_value(value) {
                return false;
            }
        }
        true
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::new).collect()
}

fn solve_1(passports: &Vec<Passport>) -> u32 {
    passports.iter().fold(0, |acc, passport| {
        if passport.has_required_fields() {
            acc + 1
        } else {
            acc
        }
    })
}

fn solve_2(passports: &Vec<Passport>) -> u32 {
    passports.iter().fold(
        0,
        |acc, passport| {
            if passport.is_valid() {
                acc + 1
            } else {
                acc
            }
        },
    )
}

pub fn run() {
    let input = std::fs::read_to_string("inputs/day4").unwrap();
    let passports = parse_input(&input);
    println!("day 4 solution 1 : {}", solve_1(&passports));
    println!("day 4 solution 2 : {}", solve_2(&passports));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_passport_field() {
        assert_eq!(PassportField::BirthYear, PassportField::new("byr"));
    }

    #[test]
    fn test_new_passport() {
        assert_eq!(
            Passport::new("byr:2020\neyr:2030 hcl:#000000"),
            Passport {
                fields: [
                    (PassportField::BirthYear, "2020"),
                    (PassportField::ExpirationYear, "2030"),
                    (PassportField::HairColor, "#000000"),
                ]
                .iter()
                .cloned()
                .collect()
            }
        );
    }

    #[test]
    fn test_solution_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
        assert_eq!(solve_1(&parse_input(&input)), 2);
    }

    #[test]
    fn test_solution_2_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(solve_2(&parse_input(&input)), 4);
    }

    #[test]
    fn test_solution_2_invalid() {
        let input = "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007";
        assert_eq!(solve_2(&parse_input(&input)), 0);
    }
}
