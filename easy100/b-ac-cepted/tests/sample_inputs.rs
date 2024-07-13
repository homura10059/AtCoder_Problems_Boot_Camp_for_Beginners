use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"AtCoder"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "AC\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"ACoder"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"AcycliC"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"AtCoCo"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"Atcoder"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample6() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"ATCoder"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample7() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"AtcodeCr"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "AC\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample8() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"AcCc"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "AC\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample9() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"aACc"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "WA\n");
    assert!(output.stderr_str().is_empty());
}
