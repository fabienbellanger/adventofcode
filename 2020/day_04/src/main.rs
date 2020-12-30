use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Passport {
    ecl: String,
    pid: String,
    eyr: usize,
    hcl: String,
    byr: usize,
    iyr: usize,
    cid: Option<usize>,
    hgt: String,
}

impl Passport {
    fn new() -> Self {
        Self {
            ecl: "".to_owned(),
            pid: "".to_owned(),
            eyr: 0,
            hcl: "".to_owned(),
            byr: 0,
            iyr: 0,
            cid: None,
            hgt: "".to_owned(),
        }
    }

    fn is_valid(&self) -> bool {
        self.ecl != *""
            && self.pid != *""
            && self.eyr != 0
            && self.hcl != *""
            && self.byr != 0
            && self.iyr != 0
            && self.hgt != *""
    }

    fn is_valid2(&self) -> bool {
        // x  ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        // x  pid (Passport ID) - a nine-digit number, including leading zeroes.
        // x  eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        // x  hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        // x  byr (Birth Year) - four digits; at least 1920 and at most 2002.
        // x  iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        // x  cid (Country ID) - ignored, missing or not.
        //    hgt (Height) - a number followed by either cm or in:
        //        If cm, the number must be at least 150 and at most 193.
        //        If in, the number must be at least 59 and at most 76.

        self._valid_ecl()
            && self._valid_eyr()
            && self._valid_byr()
            && self._valid_iyr()
            && self._valid_hcl()
            && self._valid_pid()
            && self._valid_hgt()
    }

    fn _valid_ecl(&self) -> bool {
        let colors = vec![
            String::from("amb"),
            String::from("blu"),
            String::from("brn"),
            String::from("gry"),
            String::from("grn"),
            String::from("hzl"),
            String::from("oth"),
        ];
        colors.contains(&self.ecl)
    }

    fn _valid_eyr(&self) -> bool {
        self.eyr >= 2020 && self.eyr <= 2030
    }

    fn _valid_byr(&self) -> bool {
        self.byr >= 1920 && self.byr <= 2002
    }

    fn _valid_iyr(&self) -> bool {
        self.iyr >= 2010 && self.iyr <= 2020
    }

    fn _valid_hcl(&self) -> bool {
        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        re.is_match(&self.hcl)
    }

    fn _valid_pid(&self) -> bool {
        let re = Regex::new(r"^[0-9]{9}$").unwrap();
        re.is_match(&self.pid)
    }

    fn _valid_hgt(&self) -> bool {
        let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();

        if !re.is_match(&self.hgt) {
            return false;
        }

        let cap = re.captures(&self.hgt).unwrap();
        if cap.len() != 3 {
            return false;
        }
        let value: usize = (&cap[1]).parse().unwrap();
        let unit: &str = &cap[2];

        match unit {
            "cm" => value >= 150 && value <= 193,
            _ => value >= 59 && value <= 76,
        }
    }
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
    println!("Part 2 result: {}", part2(get_data()));
}

fn part1(passports: Vec<Passport>) -> usize {
    let mut valid = 0;
    for passport in passports.iter() {
        if passport.is_valid() {
            valid += 1;
        }
    }

    valid
}

fn part2(passports: Vec<Passport>) -> usize {
    let mut valid = 0;
    for passport in passports.iter() {
        if passport.is_valid2() {
            valid += 1;
        }
    }

    valid
}

fn get_passports(data: String) -> Vec<Passport> {
    let lines: Vec<&str> = data.split("\n\n").collect();

    let mut passports: Vec<Passport> = Vec::new();
    for line in lines.iter() {
        let line = line.replace("\n", " ");
        let parts = line.split_whitespace();

        let mut passport = Passport::new();
        for part in parts {
            let mut field = part.split(':');
            let field_name = field.next().unwrap();
            let field_value = field.next().unwrap();
            match field_name {
                "ecl" => passport.ecl = field_value.to_owned(),
                "pid" => passport.pid = field_value.to_owned(),
                "eyr" => passport.eyr = field_value.parse().expect("invalid eyr"),
                "hcl" => passport.hcl = field_value.to_owned(),
                "byr" => passport.byr = field_value.parse().expect("invalid byr"),
                "iyr" => passport.iyr = field_value.parse().expect("invalid iyr"),
                "cid" => passport.cid = Some(field_value.parse().expect("invalid cid")),
                "hgt" => passport.hgt = field_value.to_owned(),
                _ => (),
            };
        }

        passports.push(passport);
    }

    passports
}

fn get_data() -> Vec<Passport> {
    let data = fs::read_to_string("input.txt").expect("Cannot read the file input.txt");
    get_passports(data)
}

fn _get_data_test() -> Vec<Passport> {
    let data = fs::read_to_string("test.txt").expect("Cannot read the file test.txt");
    get_passports(data)
}

#[test]
fn test_part1() {
    assert_eq!(2, part1(_get_data_test()));
    assert_eq!(239, part1(get_data()));
}

#[test]
fn test_part2() {
    assert_eq!(2, part2(_get_data_test()));
    assert_eq!(188, part2(get_data()));
}
