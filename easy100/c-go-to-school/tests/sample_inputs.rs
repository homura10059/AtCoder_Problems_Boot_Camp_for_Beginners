use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
2 3 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3 1 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
1 2 3 4 5
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1 2 3 4 5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"8
8 2 7 3 4 5 6 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8 2 4 5 6 7 3 1\n");
    assert!(output.stderr_str().is_empty());
}
