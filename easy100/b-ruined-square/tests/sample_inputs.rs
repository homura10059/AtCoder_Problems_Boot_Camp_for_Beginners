use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"0 0 0 1"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-1 1 -1 0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2 3 6 6"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3 10 -1 7\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"31 -41 -59 26"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-126 -64 -36 -131\n");
    assert!(output.stderr_str().is_empty());
}
