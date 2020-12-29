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
}

fn main() {
    println!("Part 1 result: {}", part1(get_data()));
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

#[test]
fn test_part1() {
    assert_eq!(2, part1(_get_data_test()));
    assert_eq!(239, part1(get_data()));
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
