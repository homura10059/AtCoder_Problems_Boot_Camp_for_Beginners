use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"xyz
abc
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "xaybzc\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"atcoderbeginnercontest
atcoderregularcontest
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(
        output.stdout_str(),
        "aattccooddeerrbreeggiunlnaerrccoonntteesstt\n"
    );
    assert!(output.stderr_str().is_empty());
}
