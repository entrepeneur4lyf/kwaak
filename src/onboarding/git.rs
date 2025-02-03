use serde_json::json;

use crate::{
    config::defaults::{default_main_branch, default_owner_and_repo},
    onboarding::util::{prompt_api_key, prompt_text},
};

pub fn git_questions(context: &mut tera::Context) {
    let (default_owner, default_repository) = default_owner_and_repo().unzip();
    let default_branch = default_main_branch();
    let branch_input = prompt_text("Default git branch", Some(&default_branch))
        .prompt()
        .unwrap();

    println!("\nWith a github token, Kwaak can create pull requests, search github code, and automatically push to the remote.");
    let github_api_key = prompt_api_key(
        "GitHub api key (optional, <esc> to skip)",
        Some("env:GITHUB_TOKEN"),
    )
    .prompt_skippable()
    .unwrap();

    let auto_push_remote =
        inquire::Confirm::new("Push to git remote after changes? (requires github token)")
            .with_default(github_api_key.is_some())
            .prompt()
            .unwrap();

    let owner_input = prompt_text(
        "Git owner (optional, <esc> to skip)",
        default_owner.as_deref(),
    )
    .prompt_skippable()
    .unwrap();
    let repository_input = prompt_text(
        "Git repository (optional, <esc> to skip)",
        default_repository.as_deref(),
    )
    .prompt_skippable()
    .unwrap();

    context.insert("github_api_key", &github_api_key);
    context.insert(
        "git",
        &json!({
            "owner": owner_input,
            "repository": repository_input,
            "main_branch": branch_input,
            "auto_push_remote": auto_push_remote,

        }),
    );
}
