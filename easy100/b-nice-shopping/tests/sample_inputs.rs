use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 3 1
3 3
3 3 3
1 2 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "5\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"1 1 2
10
10
1 1 5
1 1 10
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2 2 1
3 5
3 5
2 2 2
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "6\n");
    assert!(output.stderr_str().is_empty());
}
