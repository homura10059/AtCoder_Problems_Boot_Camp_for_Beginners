use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1
10
2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "4\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
9
3 6
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "12\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
20
11 12 9 17 12
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "74\n");
    assert!(output.stderr_str().is_empty());
}
