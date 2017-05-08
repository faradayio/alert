extern crate cli_test_dir;

use cli_test_dir::*;

#[test]
fn no_subcommand_shows_help() {
    let testdir = TestDir::new("alert", "no_subcommand_shows_help");
    let output = testdir.cmd().output()
        .expect("could not run command");
    assert!(!output.status.success(), "zero args should return an error");
    assert!(output.stderr_str().contains("USAGE"));
}

#[test]
fn subcommand_run_reports_success() {
    let testdir = TestDir::new("alert", "subcommand_run_reports_success");
    testdir.cmd().args(&["run", "true"]).expect_success();
}

#[test]
fn subcommand_run_reports_failue() {
    let testdir = TestDir::new("alert", "subcommand_run_reports_failue");
    let output = testdir.cmd()
        .args(&["run", "false"])
        .output()
        .expect("could not run command");
    assert!(!output.status.success(), "error should be detected");
    assert!(output.stderr_str().contains("Error running"));
}

#[test]
fn subcommand_run_handles_command_and_args() {
    let testdir = TestDir::new("alert", "subcommand_run_handles_command_and_args");
    testdir.cmd()
        .args(&["run", "touch", "test.txt"])
        .expect_success();
    testdir.expect_path("test.txt");
}
