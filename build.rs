use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    process::Command,
};

struct Solution {
    part1: bool,
    part2: bool,
    tests: bool,
}

fn main() {
    let mut solutions: BTreeMap<u16, BTreeMap<u8, Solution>> = BTreeMap::new();
    for entry in fs::read_dir("src/solutions").unwrap() {
        let name: String = entry.unwrap().file_name().into_string().unwrap();
        if name.len() == 8 && name.starts_with("year") {
            if let Ok(year) = name[4..].parse::<u16>() {
                for entry in fs::read_dir(format!("src/solutions/year{}", year)).unwrap() {
                    let name: String = entry.unwrap().file_name().into_string().unwrap();
                    if name.len() == 8 && name.starts_with("day") && name.ends_with(".rs") {
                        if let Ok(day) = name[3..5].parse::<u8>() {
                            let content = fs::read_to_string(format!(
                                "src/solutions/year{}/day{:02}.rs",
                                year, day
                            ))
                            .unwrap();
                            solutions.entry(year).or_insert_with(BTreeMap::new).insert(
                                day,
                                Solution {
                                    part1: content.contains("pub fn part1"),
                                    part2: content.contains("pub fn part2"),
                                    tests: content.contains("pub fn tests"),
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    let mut f = File::create("src/solutions.rs").unwrap();

    writeln!(f, "// generated by build.rs -- don't edit directly\n").unwrap();

    for (year, year_solutions) in &solutions {
        writeln!(f, "mod year{} {{", year).unwrap();
        for day in year_solutions.keys() {
            writeln!(f, "pub mod day{:0>2};", day).unwrap();
        }
        writeln!(f, "}}").unwrap();
    }

    writeln!(
        f,
        "
        use std::collections::BTreeMap;

        pub struct Solution {{
            pub part1: Option<fn(&str) -> String>,
            pub part2: Option<fn(&str) -> String>,
            pub tests: Option<fn() -> ()>,
        }}

        // Clippy doesn't like the redundant to_string when the solution is
        // already a String.
        #[allow(clippy::redundant_clone)]
        pub fn build() -> BTreeMap<u16, BTreeMap<u8, Solution>> {{
            let mut solutions: BTreeMap<u16, BTreeMap<u8, Solution>> = BTreeMap::new();
        "
    )
    .unwrap();

    for (year, year_solutions) in &solutions {
        for (day, solution) in year_solutions {
            writeln!(
                f,
                "
                solutions.entry({}).or_insert_with(BTreeMap::new).insert(
                    {},
                    Solution {{
                ",
                year, day,
            )
            .unwrap();

            if solution.part1 {
                writeln!(
                    f,
                    "part1: Some(|input| year{}::day{:0>2}::part1(input).to_string()),",
                    year, day,
                )
                .unwrap();
            } else {
                writeln!(f, "part1: None,").unwrap();
            }

            if solution.part2 {
                writeln!(
                    f,
                    "part2: Some(|input| year{}::day{:0>2}::part2(input).to_string()),",
                    year, day,
                )
                .unwrap();
            } else {
                writeln!(f, "part2: None,").unwrap();
            }

            if solution.tests {
                writeln!(f, "tests: Some(year{}::day{:0>2}::tests),", year, day).unwrap();
            } else {
                writeln!(f, "tests: None,").unwrap();
            }

            writeln!(
                f,
                "
                    }},
                );
                "
            )
            .unwrap();
        }
    }

    writeln!(
        f,
        "
            solutions
        }}
        "
    )
    .unwrap();

    Command::new("rustfmt")
        .arg("src/solutions.rs")
        .output()
        .unwrap();
}
