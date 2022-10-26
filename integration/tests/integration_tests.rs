use std::process::Command;

enum Lang {
    Python,
    Rust,
}

#[test]
fn test_python() {
    test_cases(Lang::Python);
}

#[test]
fn test_rust() {
    test_cases(Lang::Rust);
}

fn test_cases(lang: Lang) {
    let input_expected = vec![
        ("hi", "Unknown Variable: hi\n"),
        ("x *2", "Unknown Variable: x\n"),
        ("5 *2", "x0 = 10\n"),
        (
            "5.2= *2",
            "Unexpected character = at position 4\n5.2= *2\n   ^\n",
        ),
        ("(2 -9.0) / 3.0", "x0 = -2.3333333333333335\n"),
        (
            "10 + 7.( + 2",
            "Unexpected character ( at position 8\n10 + 7.( + 2\n       ^\n",
        ),
        ("10.2", "x0 = 10.2\n"),
        ("10.2+", "Invalid Expression\n"),
    ];
    run_tests(input_expected, &lang);
}

fn run_tests(input_expected: Vec<(&str, &str)>, lang: &Lang) {
    let mut results = Vec::new();
    let line = "-".repeat(50);

    for &(input, _) in input_expected.iter() {
        results.push(run_command(&lang, input))
    }

    for ((input, expected), output) in input_expected.iter().zip(results.iter()) {
        println!("{}", line);
        println!("input: {input}");
        println!("output: {output}");
        println!("expected: {expected}");
        if output != expected {
            println!("X");
        } else {
            println!("\u{2713}");
        }
    }
    println!("{}", line);

    for ((_, expected), output) in input_expected.iter().zip(results.iter()) {
        assert_eq!(output, expected);
    }
}

fn run_command(language: &Lang, expression: &str) -> String {
    let mut executable = match language {
        Lang::Python => Command::new("../python/src/cli_calculator.py"),
        Lang::Rust => Command::new("../rust/target/debug/cli_calculator"),
    };

    let output = executable
        .arg("--expression")
        .arg(format!("{}", expression))
        .output()
        .unwrap();
    String::from(String::from_utf8_lossy(&output.stdout))
}
