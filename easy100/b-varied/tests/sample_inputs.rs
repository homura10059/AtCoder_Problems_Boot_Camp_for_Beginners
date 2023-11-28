use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"uncopyrightable"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "yes\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"different"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "no\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"no"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "yes\n");
    assert!(output.stderr_str().is_empty());
}
