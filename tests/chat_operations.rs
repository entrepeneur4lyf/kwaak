use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::TempDir;
use uuid::Uuid;

struct ChatEnvironment {
    cmd: Command,
    dir: TempDir,
    uuids: Vec<Uuid>,
}

fn setup_chat_env() -> ChatEnvironment {
    let dir = tempfile::Builder::new()
        .prefix("kwaak-chat-test")
        .tempdir()
        .unwrap();

    let mut cmd = Command::cargo_bin("kwaak").unwrap();
    cmd.current_dir(&dir);

    ChatEnvironment {
        cmd,
        dir,
        uuids: Vec::new(),
    }
}

#[test_log::test(tokio::test)]
async fn test_delete_single_chat() {
    let mut env = setup_chat_env();

    // Simulate adding a chat and storing its UUID
    let chat_uuid = Uuid::new_v4();
    env.uuids.push(chat_uuid);

    // Command for deleting a chat
    env.cmd
        .args(["delete_chat", &chat_uuid.to_string()])
        .assert()
        .success();

    // Verify chat is deleted - attempting further interaction
    env.cmd
        .args(["get_chat", &chat_uuid.to_string()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("could not find chat"));
}

#[test_log::test(tokio::test)]
async fn test_delete_all_chats_and_verify_default() {
    let mut env = setup_chat_env();

    // Simulate adding and deleting all chats
    for _ in 0..3 {
        let chat_uuid = Uuid::new_v4();
        env.uuids.push(chat_uuid);
        env.cmd
            .args(["add_chat", &chat_uuid.to_string()])
            .assert()
            .success();
        env.cmd
            .args(["delete_chat", &chat_uuid.to_string()])
            .assert()
            .success();
    }

    // Check if a new default chat is created
    env.cmd
        .args(["list_chats"])
        .assert()
        .stdout(predicate::str::contains("Chat #1"));
}
