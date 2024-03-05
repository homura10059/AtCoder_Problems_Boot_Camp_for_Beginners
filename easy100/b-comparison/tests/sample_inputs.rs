use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"36
24
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "GREATER\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"850
3777
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "LESS\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"9720246
22516266
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "LESS\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"123456789012345678901234567890
234567890123456789012345678901
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "LESS\n");
    assert!(output.stderr_str().is_empty());
}
