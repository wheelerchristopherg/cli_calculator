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
    assert_eq!(run(&lang, "hi"), "Unknown Variable: hi\n");
    assert_eq!(run(&lang, "x *2"), "Unknown Variable: x\n");
    assert_eq!(run(&lang, "5 *2"), "x0 = 10\n");
    assert_eq!(
        run(&lang, "5.2= *2"),
        "Unexpected character = at position 4\n5.2= *2\n   ^\n"
    );
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
