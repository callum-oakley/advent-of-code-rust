use std::{
    collections::BTreeMap,
    env,
    fs::{self, File},
    io::Write,
    process::Command,
};

struct Solution {
    part1: bool,
    part2: bool,
    tests: bool,
}

fn insert_day(solutions: &mut BTreeMap<u16, BTreeMap<u8, Solution>>, year: u16, day: u8) {
    let content =
        fs::read_to_string(format!("src/solutions/year{}/day{:02}.rs", year, day)).unwrap();
    solutions.entry(year).or_default().insert(
        day,
        Solution {
            part1: content.contains("pub fn part1"),
            part2: content.contains("pub fn part2"),
            tests: content.contains("pub fn tests"),
        },
    );
}

fn insert_year(solutions: &mut BTreeMap<u16, BTreeMap<u8, Solution>>, year: u16) {
    match env::var("DAY").as_deref() {
        Ok("") | Err(_) => {
            for entry in fs::read_dir(format!("src/solutions/year{}", year)).unwrap() {
                let name: String = entry.unwrap().file_name().into_string().unwrap();
                if name.len() == 8 && name.starts_with("day") && name.ends_with(".rs") {
                    if let Ok(day) = name[3..5].parse::<u8>() {
                        insert_day(solutions, year, day);
                    }
                }
            }
        }
        Ok(day) => {
            let day = day.parse::<u8>().unwrap();
            insert_day(solutions, year, day);
        }
    }
}

fn main() {
    let mut solutions: BTreeMap<u16, BTreeMap<u8, Solution>> = BTreeMap::new();
    match env::var("YEAR").as_deref() {
        Ok("") | Err(_) => {
            for entry in fs::read_dir("src/solutions").unwrap() {
                let name: String = entry.unwrap().file_name().into_string().unwrap();
                if name.len() == 8 && name.starts_with("year") {
                    if let Ok(year) = name[4..].parse::<u16>() {
                        insert_year(&mut solutions, year);
                    }
                }
            }
        }
        Ok(year) => {
            let year = year.parse::<u16>().unwrap();
            insert_year(&mut solutions, year);
        }
    }

    let mut f = File::create("src/solutions.rs").unwrap();

    writeln!(f, "// generated by build.rs -- don't edit directly\n").unwrap();

    for (year, year_solutions) in &solutions {
        writeln!(f, "pub mod year{} {{", year).unwrap();
        for day in year_solutions.keys() {
            writeln!(f, "pub mod day{:0>2};", day).unwrap();
        }
        writeln!(f, "}}\n").unwrap();
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
        #[expect(clippy::redundant_clone, clippy::too_many_lines)]
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
                solutions.entry({}).or_default().insert(
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
