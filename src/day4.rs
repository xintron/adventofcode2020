#[derive(Debug, PartialEq, Default)]
pub struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiration_year: Option<u32>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<u32>,
}

#[derive(Debug, PartialEq)]
enum Height {
    Centimeters(u32),
    Inches(u32),
}

impl From<&str> for Passport {
    fn from(data: &str) -> Self {
        let mut pass = Self::default();
        data.lines().for_each(|r| {
            r.split(' ').for_each(|l| {
                let mut fields = l.split(':');
                match fields.next().unwrap() {
                    "byr" => pass.birth_year = fields.next().map(|b| b.parse().unwrap()),
                    "iyr" => pass.issue_year = fields.next().map(|b| b.parse().unwrap()),
                    "eyr" => pass.expiration_year = fields.next().map(|b| b.parse().unwrap()),
                    "hgt" => {
                        pass.height = fields.next().and_then(|s| {
                            let mut height = String::new();
                            let mut cm = String::new();
                            for c in s.chars() {
                                if c.is_numeric() {
                                    height.push(c);
                                } else {
                                    cm.push(c);
                                }
                            }
                            let uheight = height.parse().unwrap();
                            match cm.as_str() {
                                "cm" => Some(Height::Centimeters(uheight)),
                                "in" => Some(Height::Inches(uheight)),
                                _ => None,
                            }
                        })
                    }

                    "hcl" => pass.hair_color = fields.next().map(|s| s.to_string()),
                    "ecl" => pass.eye_color = fields.next().map(|s| s.to_string()),
                    "pid" => pass.passport_id = fields.next().map(|s| s.to_string()),
                    "cid" => pass.country_id = fields.next().map(|b| b.parse().unwrap()),
                    _ => unreachable!(),
                }
            })
        });
        pass
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn is_valid_p2(&self) -> bool {
        self.is_birth_year_valid()
            && self.is_issue_year_valid()
            && self.is_expiration_year_valid()
            && self.is_height_valid()
            && self.is_hair_color_valid()
            && self.is_eye_color_valid()
            && self.is_passport_id_valid()
    }

    fn is_birth_year_valid(&self) -> bool {
        self.birth_year.is_some() && self.birth_year >= Some(1920) && self.birth_year <= Some(2002)
    }

    fn is_issue_year_valid(&self) -> bool {
        self.issue_year.is_some() && self.issue_year >= Some(2010) && self.issue_year <= Some(2020)
    }

    fn is_expiration_year_valid(&self) -> bool {
        self.expiration_year.is_some()
            && self.expiration_year >= Some(2020)
            && self.expiration_year <= Some(2030)
    }

    fn is_height_valid(&self) -> bool {
        if self.height.is_none() {
            return false;
        }
        match self.height.as_ref().unwrap() {
            Height::Centimeters(y) => return *y >= 150 && *y <= 193,
            Height::Inches(y) => return *y >= 59 && *y <= 76,
        }
    }

    fn is_hair_color_valid(&self) -> bool {
        if self.hair_color.is_none() {
            return false;
        }
        let mut color = self.hair_color.as_ref().unwrap().chars();
        color.next().unwrap() == '#' && color.take_while(|&c| c.is_ascii_hexdigit()).count() == 6
    }

    fn is_eye_color_valid(&self) -> bool {
        if self.eye_color.is_none() {
            return false;
        }
        match self.eye_color.as_ref().unwrap().as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
            _ => return false,
        }
    }

    fn is_passport_id_valid(&self) -> bool {
        if self.passport_id.is_none() {
            return false;
        }
        self.passport_id
            .as_ref()
            .unwrap()
            .chars()
            .take_while(|&c| c.is_numeric())
            .count()
            == 9
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::from).collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|p| p.is_valid() && p.is_valid_p2())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let pass = Passport::from(input);

        assert_eq!(
            pass,
            Passport {
                birth_year: Some(1937),
                issue_year: Some(2017),
                expiration_year: Some(2020),
                height: Some(Height::Centimeters(183)),
                hair_color: Some("#fffffd".to_string()),
                eye_color: Some("gry".to_string()),
                passport_id: Some("860033327".to_string()),
                country_id: Some(147),
            }
        );
        assert_eq!(pass.is_valid(), true);
    }

    #[test]
    fn missing_country() {
        let input = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        let pass = Passport::from(input);
        assert_eq!(
            pass,
            Passport {
                birth_year: Some(1931),
                issue_year: Some(2013),
                expiration_year: Some(2024),
                height: Some(Height::Centimeters(179)),
                hair_color: Some("#ae17e1".to_string()),
                eye_color: Some("brn".to_string()),
                passport_id: Some("760753108".to_string()),
                country_id: None,
            }
        );
        assert_eq!(pass.is_valid(), true);
    }

    #[test]
    fn invalid_passport() {
        let input = "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let pass = Passport::from(input);
        assert_eq!(
            pass,
            Passport {
                birth_year: None,
                issue_year: Some(2011),
                expiration_year: Some(2025),
                height: Some(Height::Inches(59)),
                hair_color: Some("#cfa07d".to_string()),
                eye_color: Some("brn".to_string()),
                passport_id: Some("166559648".to_string()),
                country_id: None,
            }
        );
        assert_eq!(
            pass.is_valid(),
            false,
            "verifying that {:?} is invalid",
            pass
        );
    }

    #[test]
    fn invalid_passport_p2_invalid() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        for line in input.split("\n\n") {
            let pass = Passport::from(line);
            assert_eq!(
                pass.is_valid_p2(),
                false,
                "verifying that all fields {:?} are invalid",
                pass
            );
        }
    }

    #[test]
    fn invalid_passport_p2_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        for line in input.split("\n\n") {
            let pass = Passport::from(line);
            assert_eq!(
                pass.is_valid_p2(),
                true,
                "verifying that all fields {:?} are valid",
                pass
            );
        }
    }
}
