use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
7 1
2
5
10"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "8\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
8 20
1
10"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "29\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"5
30 44
26
18
81
18
6
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "56\n");
    assert!(output.stderr_str().is_empty());
}
