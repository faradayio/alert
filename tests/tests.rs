extern crate cli_test_dir;

use cli_test_dir::*;
use std::env;

#[test]
fn no_subcommand_shows_help() {
    env::set_var("ALERT_NOTIFIER", "console");
    let testdir = TestDir::new("alert", "no_subcommand_shows_help");
    let output = testdir.cmd().output().expect("could not run command");
    assert!(!output.status.success(), "zero args should return an error");
    assert!(output.stderr_str().contains("USAGE"));
}

#[test]
fn subcommand_run_reports_success() {
    env::set_var("ALERT_NOTIFIER", "console");
    let testdir = TestDir::new("alert", "subcommand_run_reports_success");
    let output = testdir
        .cmd()
        .args(&["run", "true"])
        .output()
        .expect_success();
    assert!(output.stderr_str().contains("NOTIFICATION: SUCCESS: true"));
}

#[test]
fn subcommand_run_reports_failue() {
    env::set_var("ALERT_NOTIFIER", "console");
    let testdir = TestDir::new("alert", "subcommand_run_reports_failue");
    let output = testdir
        .cmd()
        .args(&["run", "false"])
        .output()
        .expect("could not run command");
    assert!(!output.status.success(), "error should be detected");
    assert!(output.stderr_str().contains("Could not run"));
    assert!(output.stderr_str().contains("NOTIFICATION: FAILURE: false"));
}

#[test]
fn subcommand_run_handles_command_and_args() {
    env::set_var("ALERT_NOTIFIER", "console");
    let testdir = TestDir::new("alert", "subcommand_run_handles_command_and_args");
    let output = testdir
        .cmd()
        .args(&["run", "touch", "test.txt"])
        .output()
        .expect_success();
    testdir.expect_path("test.txt");
    assert!(output.stderr_str().contains("NOTIFICATION: SUCCESS: touch test.txt"));
}
