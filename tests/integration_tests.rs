use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::RegexPredicate;
use std::path::Path;

fn page() -> Command {
    Command::cargo_bin("page").unwrap()
}

fn tempdir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

/* rpassword inserts a new line on windows  */
fn enter_passphrase_and_password(entry: &str) -> RegexPredicate {
    predicate::str::is_match(format!(
        "Enter passphrase: (\n)?Password for '{entry}': (\n)?"
    ))
    .unwrap()
}

fn enter_passphrase_show(value: &str) -> RegexPredicate {
    predicate::str::is_match(format!("Enter passphrase: (\n)?{value}")).unwrap()
}

fn enter_passphrase_and_overwrite_password(entry: &str) -> RegexPredicate {
    predicate::str::is_match(format!(r"Enter passphrase: (\n)?Entry '{entry}' already exists. Overwrite \(y/N\)\?Password for '{entry}': (\n)?")).unwrap()
}

#[test]
fn sanity() {
    page()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("page "));

    page()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "A password manager with age encryption",
        ));
}

#[test]
fn info() {
    let dir = tempdir();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .assert()
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("info")
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("Storage file: ").and(
                predicate::str::contains("entries.toml.age")
                    .and(predicate::str::contains("\n").count(1).trim()),
            ),
        );
}

#[test]
fn switch_storage_folder() {
    let dir = tempdir();

    let entries_file_path = Path::new(dir.path())
        .join("entries.toml.age")
        .display()
        .to_string();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .assert()
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("info")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(format!(
            "Storage file: {entries_file_path}"
        )));
}

#[test]
fn new_show_list() {
    let dir = tempdir();
    let passphrase = "master";
    let entry = "entry";
    let password = "password";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(passphrase)
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{passphrase}\n{password}"))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(password))
        .success();
}

#[test]
fn new_overwrite() {
    let dir = tempdir();
    let passphrase = "master";
    let entry = "entry";
    let password = "password";
    let new_password = "new_password";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(passphrase)
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{passphrase}\n{password}"))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{passphrase}\ny\n{new_password}"))
        .assert()
        .stdout(enter_passphrase_and_overwrite_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(new_password))
        .success();
}

#[test]
fn new_show_attributes() {
    let dir = tempdir();
    let passphrase = "master";
    let entry = "entry";
    let password = "password";
    let username = "username";
    let url = "url";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(passphrase)
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .arg("-u")
        .arg(username)
        .arg("--url")
        .arg(url)
        .write_stdin(format!("{passphrase}\n{password}"))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg(entry)
        .arg("--on-screen")
        .arg("--attribute")
        .arg("username")
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(username))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg(entry)
        .arg("--on-screen")
        .arg("--attribute")
        .arg("url")
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(url))
        .success();
}

#[test]
fn edit_entry() {
    let dir = tempdir();
    let passphrase = "secret";
    let entry = "editable";
    let password = "password";
    let new_password = "password2";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(passphrase)
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{passphrase}\n{password}\n"))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(password))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg(entry)
        .write_stdin(format!("{passphrase}\n{new_password}",))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(new_password))
        .success();
}

#[test]
fn edit_entry_name() {
    let dir = tempdir();
    let passphrase = "secret";
    let entry = "editable";
    let new_entry = "editable2";
    let password = "password";
    let new_password = "password2";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(passphrase)
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{passphrase}\n{password}"))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(password))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg(entry)
        .arg("-n")
        .arg(new_entry)
        .write_stdin(format!("{passphrase}\n{new_password}"))
        .assert()
        .stdout(enter_passphrase_and_password(new_entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(new_entry)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(new_password))
        .success();
}

#[test]
fn edit_entry_overwrite() {
    let dir = tempdir();
    let passphrase = "secret";
    let entry = "editable";
    let entry2 = "editable2";
    let password = "password";
    let new_password = "password2";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(passphrase)
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{passphrase}\n{password}"))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry2)
        .write_stdin(format!("{passphrase}\n{password}"))
        .assert()
        .stdout(enter_passphrase_and_password(entry2))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg(entry)
        .arg("-n")
        .arg(entry2)
        .write_stdin(format!("{passphrase}\ny\n{new_password}"))
        .assert()
        .stdout(enter_passphrase_and_overwrite_password(entry2))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(entry2))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("--on-screen")
        .arg(entry2)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(new_password))
        .success();
}

#[test]
fn remove_entry() {
    let dir = tempdir();
    let passphrase = "donttell";
    let entry = "begone";
    let password = "pw";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(passphrase)
        .assert()
        .stdout(predicate::str::contains("Enter passphrase:"))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg(entry)
        .write_stdin(format!("{passphrase}\n{password}"))
        .assert()
        .stdout(enter_passphrase_and_password(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(entry))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("remove")
        .arg(entry)
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(""))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .write_stdin(passphrase)
        .assert()
        .stdout(enter_passphrase_show(""))
        .success();
}

#[test]
fn fail_list_no_init() {
    let dir = tempdir();
    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: storage not initialized, run `page init`",
        ));
}

#[test]
fn fail_show_no_init() {
    let dir = tempdir();
    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("show")
        .arg("foo")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: storage not initialized, run `page init`",
        ));
}

#[test]
fn fail_new_no_init() {
    let dir = tempdir();
    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("new")
        .arg("entry")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "Error: storage not initialized, run `page init`",
        ));
}

#[test]
fn fail_edit_no_entry() {
    let dir = tempdir();
    let passphrase = "fail";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("edit")
        .arg("404")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .failure()
        .stdout(enter_passphrase_show(""))
        .stderr("Error: entry '404' not found\n");
}

#[test]
fn fail_remove_no_entry() {
    let dir = tempdir();
    let passphrase = "no_entry_no_remove";

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("init")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .stdout(predicate::str::contains("Enter passphrase: "))
        .success();

    page()
        .env("PAGE_STORAGE_FOLDER", dir.path())
        .arg("--no-keyring")
        .arg("remove")
        .arg("no-entry")
        .write_stdin(format!("{}", passphrase))
        .assert()
        .failure()
        .stdout(enter_passphrase_show(""))
        .stderr("Error: entry 'no-entry' not found\n");
}

#[test]
fn shell_completion_help() {
    page()
        .arg("completion")
        .arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "bash, zsh, fish, elvish, powershell, nushell",
        ));
}

#[test]
fn shell_completion_stdout() {
    page()
        .arg("completion")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("_page()"));
}

#[test]
fn shell_completion_invalid_shell() {
    page()
        .arg("completion")
        .arg("invalid_shell")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: invalid value 'invalid_shell' for '<SHELL>'",
        ));
}
