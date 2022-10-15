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
    ];

    let mut results = Vec::new();

    for &(i, _) in input_expected.iter() {
        results.push(run(&lang, i))
    }

    for ((input, expected), output) in input_expected.iter().zip(results.iter()) {
        println!("input: {input}");
        println!("output: {output}");
        println!("expected: {expected}");
    }

    for ((_, expected), output) in input_expected.iter().zip(results.iter()) {
        assert_eq!(output, expected);
    }
}

fn run(language: &Lang, expression: &str) -> String {
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
