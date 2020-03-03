fn main() {
    use std::process::{Command, Stdio};
    use std::str;

    // ioreg -rc AppleSmartBattery
    let echo_child = Command::new("ioreg")
        .arg("-rc")
        .arg("AppleSmartBattery")
        .stdout(Stdio::piped())
        .spawn()
        .expect("[Error]: Battery Widget Failed!");

    let result = echo_child.wait_with_output()
        .unwrap()
        .stdout;

    let print_result = str::from_utf8(&result).unwrap();

    let mut curr = "None";
    let mut max  = "None";

    for line in print_result.lines() {
        let l: Vec<&str> = line.split('=').collect();

        if l[0].contains("CurrentCapacity") {
            curr = l[1].trim();
            if max != "None" {
                break;
            }
        }

        if l[0].contains("MaxCapacity") {
            max = l[1].trim();
            if curr != "None" {
                break;
            }
        }
    }

    let c = curr.parse::<f32>().unwrap();
    let m = max.parse::<f32>().unwrap();

    let charge = c / m;
    let treshold = charge * 10.0;

    let slots = 10;
    let filled = treshold.round() as u32 * (slots / 10);
    let empty = slots - filled;

    fn string_from_char(c: char, mut qty: u32) -> String {
        let mut result: Vec<char> = Vec::new();

        while qty != 0 {
            result.push(c);
            qty -= 1;
        }

        return result.into_iter().collect();
    }

    let color_out: String =  match filled {
        0..=3 => "\x1b[;31m".to_string(), // red
        4..=6 => "\x1b[;33m".to_string(), // yellow
        _ => "\x1b[;32m".to_string()      // green
    };
    let color_reset = "\x1b[0m".to_string();

    let out = [
        color_out,
        string_from_char('◼', filled),
        string_from_char('◻', empty),
        color_reset
    ].join("");

    print!("{}", out);
}
