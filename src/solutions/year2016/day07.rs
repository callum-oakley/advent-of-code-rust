use std::collections::HashSet;

fn supports_tls(ip: &str) -> bool {
    let mut in_hypernet = false;
    let mut has_supernet_abba = false;
    for section in ip.split(&['[', ']']) {
        for quad in section.as_bytes().windows(4) {
            if quad[0] != quad[1] && quad[0] == quad[3] && quad[1] == quad[2] {
                if in_hypernet {
                    return false;
                }
                has_supernet_abba = true;
            }
        }
        in_hypernet = !in_hypernet;
    }
    has_supernet_abba
}

fn supports_ssl(ip: &str) -> bool {
    let mut in_hypernet = false;
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();
    for section in ip.split(&['[', ']']) {
        for triple in section.as_bytes().windows(3) {
            if triple[0] != triple[1] && triple[0] == triple[2] {
                if in_hypernet {
                    if abas.contains(&(triple[1], triple[0])) {
                        return true;
                    }
                    babs.insert((triple[0], triple[1]));
                } else {
                    if babs.contains(&(triple[1], triple[0])) {
                        return true;
                    }
                    abas.insert((triple[0], triple[1]));
                }
            }
        }
        in_hypernet = !in_hypernet;
    }
    false
}

pub fn part1(input: &str) -> usize {
    input.lines().filter(|ip| supports_tls(ip)).count()
}

pub fn part2(input: &str) -> usize {
    input.lines().filter(|ip| supports_ssl(ip)).count()
}

pub fn tests() {
    assert!(supports_tls("abba[mnop]qrst"));
    assert!(!supports_tls("abcd[bddb]xyyx"));
    assert!(!supports_tls("aaaa[qwer]tyui"));
    assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));

    assert!(supports_ssl("aba[bab]xyz"));
    assert!(!supports_ssl("xyx[xyx]xyx"));
    assert!(supports_ssl("aaa[kek]eke"));
    assert!(supports_ssl("zazbz[bzb]cdb"));
}
