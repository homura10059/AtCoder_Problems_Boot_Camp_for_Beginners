use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"atcoderregularcontest"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "b\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"abcdefghijklmnopqrstuvwxyz"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "None\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"fajsonlslfepbjtsaayxbymeskptcumtwrmkkinjxnnucagfrg"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "d\n");
    assert!(output.stderr_str().is_empty());
}
