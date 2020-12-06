use regex::Regex;

pub fn solution() -> usize {
    let passports = load_data();
    passports.iter().filter(|p| p.validate_1()).filter(|p| p.validate_2()).count()
}

fn load_data() -> Vec<Passport> {
    let br = include_str!("../assets/problem_2020_04.txt");
    br.split("\n\n")
        .map(|s| s.lines().collect::<Vec<_>>())
        .map(|v| {
            let mut passport = Passport::default();
            for line in v {
                line.split_whitespace().for_each(|f| {
                    let field_split: Vec<&str> = f.split(":").collect();
                    let key: &str = field_split[0];
                    let value: &str = field_split[1];

                    match key {
                        "byr" => passport.byr = Some(value),
                        "iyr" => passport.iyr = Some(value),
                        "eyr" => passport.eyr = Some(value),
                        "hgt" => passport.hgt = Some(value),
                        "hcl" => passport.hcl = Some(value),
                        "ecl" => passport.ecl = Some(value),
                        "pid" => passport.pid = Some(value),
                        "cid" => passport.cid = Some(value),
                        _ => unreachable!(),
                    }
                });
            }
            passport
        })
        .collect()
}

#[derive(Debug, Default)]
struct Passport {
    pub byr: Option<&'static str>,
    pub iyr: Option<&'static str>,
    pub eyr: Option<&'static str>,
    pub hgt: Option<&'static str>,
    pub hcl: Option<&'static str>,
    pub ecl: Option<&'static str>,
    pub pid: Option<&'static str>,
    pub cid: Option<&'static str>,
}

impl Passport {
    pub fn validate_1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn validate_2(&self) -> bool {
        let ret = self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid();
        if ret { println!("{}, {:?}", ret, self) };
        ret
    }

    fn validate_byr(&self) -> bool {
        match self.byr {
            None => false,
            Some(byr) => {
                let byr = byr.parse::<usize>().unwrap_or(0);
                byr >= 1920 && byr <= 2002
            },
        }
    }

    fn validate_iyr(&self) -> bool {
        match self.iyr {
            None => false,
            Some(iyr) => {
                let iyr = iyr.parse::<usize>().unwrap_or(0);
                iyr >= 2010 && iyr <= 2020
            },
        }
    }

    fn validate_eyr(&self) -> bool {
        match self.eyr {
            None => false,
            Some(eyr) => {
                let eyr = eyr.parse::<usize>().unwrap_or(0);
                eyr >= 2020 && eyr <= 2030
            },
        }
    }

    fn validate_hgt(&self) -> bool {
        match self.hgt {
            None => false,
            Some(hgt) => {
                let measure = &hgt[hgt.len()-2..];
                let hgt = hgt[..hgt.len()-2].parse::<usize>().unwrap_or(0);
                match measure {
                    "cm" => hgt >= 150 && hgt <= 193,
                    "in" => hgt >= 59 && hgt <= 76,
                    _ => false,
                }
            },
        }
    }

    fn validate_hcl(&self) -> bool {
        match self.hcl {
            None => false,
            Some(hcl) => Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(hcl),
        }
    }

    fn validate_ecl(&self) -> bool {
        match self.ecl {
            None => false,
            Some(ecl) => {
                match ecl {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            },
        }
    }

    fn validate_pid(&self) -> bool {
        match self.pid {
            None => false,
            Some(pid) => Regex::new(r"^\d{9}$").unwrap().is_match(pid),
        }
    }
}
