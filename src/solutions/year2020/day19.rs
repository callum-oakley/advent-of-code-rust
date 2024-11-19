use std::collections::HashMap;

use regex::Regex;

fn parse(input: &str) -> (HashMap<&str, &str>, &str) {
    let (rules, messages) = input.split_once("\n\n").unwrap();
    (
        rules
            .trim()
            .lines()
            .map(|line| line.trim().split_once(": ").unwrap())
            .collect(),
        messages,
    )
}

fn build_regex(rules: &HashMap<&str, &str>) -> Regex {
    fn build_regex_string(s: &mut String, rules: &HashMap<&str, &str>, max_depth: u32, rule: &str) {
        if max_depth > 0 {
            if rules[rule].starts_with('"') {
                s.push_str(rules[&rule].trim_matches('"'));
            } else {
                s.push('(');
                for token in rules[&rule].split_whitespace() {
                    if token == "|" {
                        s.push('|');
                    } else {
                        build_regex_string(s, rules, max_depth - 1, token);
                    }
                }
                s.push(')');
            }
        } else {
            // To handle the infinite loop, cut the recursion short with a pattern that won't match
            // anything.
            s.push('x');
        }
    }
    let mut s = String::new();
    s.push_str("\\b");
    build_regex_string(&mut s, rules, 14, "0");
    s.push_str("\\b");
    Regex::new(&s).unwrap()
}

pub fn part1(input: &str) -> usize {
    let (rules, messages) = parse(input);
    build_regex(&rules).find_iter(messages).count()
}

pub fn part2(input: &str) -> usize {
    let (mut rules, messages) = parse(input);
    rules.insert("8", "42 | 42 8");
    rules.insert("11", "42 31 | 42 11 31");
    build_regex(&rules).find_iter(messages).count()
}

pub fn tests() {
    let example1 = r#"
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb
    "#;
    let example2 = r#"
        42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: "a"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: "b"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1

        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
    "#;
    assert_eq!(part1(example1), 2);
    assert_eq!(part2(example2), 12);
}
