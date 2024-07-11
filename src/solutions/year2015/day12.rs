use serde_json::Value;

fn sum_numbers(ignore_red: bool, v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| sum_numbers(ignore_red, v)).sum(),
        Value::Object(m) if !ignore_red || m.values().all(|v| v != "red") => {
            m.values().map(|v| sum_numbers(ignore_red, v)).sum()
        }
        _ => 0,
    }
}

pub fn part1(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    sum_numbers(false, &v)
}

pub fn part2(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    sum_numbers(true, &v)
}

pub fn tests() {
    assert_eq!(part1(r"[1,2,3]"), 6);
    assert_eq!(part1(r#"{"a":2,"b":4}"#), 6);
    assert_eq!(part1(r"[[[3]]]"), 3);
    assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
    assert_eq!(part1(r#"{"a":[-1,1]}"#), 0);
    assert_eq!(part1(r#"[-1,{"a":1}]"#), 0);
    assert_eq!(part1(r"[]"), 0);
    assert_eq!(part1(r"{}"), 0);

    assert_eq!(part2(r"[1,2,3]"), 6);
    assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
    assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
    assert_eq!(part2(r#"[1,"red",5]"#), 6);
}
