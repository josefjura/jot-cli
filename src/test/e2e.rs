use assert_cmd::Command;

use predicate::str::{contains, is_empty};
use predicates::prelude::*;

use crate::web_client::mock::MOCK_TOKEN;

use super::{asserts::contains_login_success_messages, test_context::TestContext};

#[test]
fn test_login() {
    // Arrange
    let ctx = TestContext::new("test_assets/profile/local.toml");

    // Act
    let assert = ctx.command().arg("login").assert();

    // Assert
    assert
        .success()
        .stdout(contains_login_success_messages())
        .stderr(is_empty());

    ctx.assert_key_file_contains(MOCK_TOKEN.as_bytes());
}

#[test]
fn test_note_search_default() {
    let ctx = TestContext::new("test_assets/profile/local.toml");

    ctx.command()
        .args(["note", "search", "--lines", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#1").and(contains("Short note")))
        // By default should only show first line of multi-line notes
        .stdout(predicate::str::contains("Multi-line note..."))
        .stdout(predicate::str::contains("With several").not())
        .stdout(predicate::str::contains("Note with special formatting:..."))
        // Should include timestamps
        .stdout(predicate::str::contains("2024-01-01"))
        .stdout(predicate::str::contains("2024-01-02"));
}

#[test]
fn test_note_search_preview_lines() {
    let ctx = TestContext::new("test_assets/profile/local.toml");

    ctx.command()
        .args(["note", "search", "--lines", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Multi-line note\nWith several"))
        .stdout(predicate::str::contains("Distinct lines").not())
        .stdout(predicate::str::contains(
            "Note with special formatting:\n* bullet point",
        ))
        .stdout(predicate::str::contains("> quote").not());
}

#[test]
fn test_note_search_json() {
    let ctx = TestContext::new("test_assets/profile/local.toml");

    let assert = ctx
        .command()
        .args(["note", "search", "--output", "json"])
        .assert()
        .success();

    let output = assert.get_output();
    let json_str = std::str::from_utf8(&output.stdout).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();

    let notes = parsed.as_array().unwrap();
    assert_eq!(notes.len(), 3);

    // Check only what we know exists in the output
    let note = &notes[0];
    assert!(note["content"].as_str().unwrap().contains("Short note"));
    assert!(notes[1]["content"]
        .as_str()
        .unwrap()
        .contains("Multi-line note"));
    assert!(notes[2]["content"]
        .as_str()
        .unwrap()
        .contains("Note with special formatting"));
}

#[test]
fn test_note_search_plain() {
    let ctx = TestContext::new("test_assets/profile/local.toml");

    ctx.command()
        .args(["note", "search", "--output", "plain"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Short note"))
        .stdout(predicate::str::contains(
            "Multi-line note\nWith several\nDistinct lines\nTo test preview",
        ))
        .stdout(predicate::str::contains(
            "Note with special formatting:\n* bullet point\n> quote",
        ));
}

#[test]
fn test_note_preview_truncation() {
    let ctx = TestContext::new("test_assets/profile/local.toml");

    // Test with 1 preview line
    ctx.command()
        .args(["note", "search", "--output", "plain", "--lines", "1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Short note"))
        .stdout(predicate::str::contains("Multi-line note...")); // Documents the newline bug

    // Test with 2 preview lines
    ctx.command()
        .args(["note", "search", "--output", "plain", "--lines", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Multi-line note\nWith several...")); // Same bug with multiple lines
}

#[test]
fn test_note_search_date() {
    let ctx = TestContext::new("test_assets/profile/local.toml");

    ctx.command()
        .args(["note", "search", "--lines", "1", "--date", "2024-01-03"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("#1")
                .not()
                .and(contains("2024-01-01").not())
                .and(contains("Short note").not()),
        )
        .stdout(
            predicate::str::contains("#3")
                .and(contains("2024-01-03"))
                .and(contains("Note with special formatting:...")),
        );
}
