use std::collections::HashMap;

use regex::Regex;

const KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse(input: &str) -> impl Iterator<Item = HashMap<&str, &str>> {
    input.split("\n\n").map(|passport| {
        passport
            .split_whitespace()
            .map(|w| w.split_once(':').unwrap())
            .collect()
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|passport| KEYS.iter().all(|k| passport.contains_key(k)))
        .count()
}

pub fn part2(input: &str) -> usize {
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
    parse(input)
        .filter(|passport| KEYS.iter().all(|k| passport.contains_key(k)))
        .filter(|passport| {
            let byr = passport["byr"].parse::<u32>();
            let iyr = passport["iyr"].parse::<u32>();
            let eyr = passport["eyr"].parse::<u32>();
            let hgt = passport["hgt"][..passport["hgt"].len() - 2].parse::<u32>();
            let hgt_unit = &passport["hgt"][passport["hgt"].len() - 2..passport["hgt"].len()];
            byr.is_ok_and(|byr| (1920..=2002).contains(&byr))
                && iyr.is_ok_and(|iyr| (2010..=2020).contains(&iyr))
                && eyr.is_ok_and(|eyr| (2020..=2030).contains(&eyr))
                && hgt.is_ok_and(|hgt| match hgt_unit {
                    "cm" => (150..=193).contains(&hgt),
                    "in" => (59..=76).contains(&hgt),
                    _ => false,
                })
                && hcl_re.is_match(passport["hcl"])
                && ecls.contains(&passport["ecl"])
                && pid_re.is_match(passport["pid"])
        })
        .count()
}

pub fn tests() {
    let example1 = "
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
    ";
    let invalid = "
        eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946

        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007
    ";
    let valid = "
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f

        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022

        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    ";
    assert_eq!(part1(example1), 2);
    assert_eq!(part2(invalid), 0);
    assert_eq!(part2(valid), 4);
}
