use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"29
20
7
35
120
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "215\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"101
86
119
108
57
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "481\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"123
123
123
123
123
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "643\n");
    assert!(output.stderr_str().is_empty());
}
