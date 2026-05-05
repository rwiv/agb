use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

const OPENAI_POLICY_CONTENT: &str = "policy:\n  allow_implicit_invocation: false\n";
const SOURCE_POLICY_CONTENT: &str = "policy:\n  allow_implicit_invocation: true\n";

#[test]
fn test_e2e_codex_openai_policy_build_clean_and_sync() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();
    let config_path = setup_fixtures(root);

    run_atb(&["build", "--config"], &config_path);

    assert_policy_eq(root, "policy_cmd", OPENAI_POLICY_CONTENT);
    assert_policy_eq(root, "external_policy_cmd", OPENAI_POLICY_CONTENT);
    assert_policy_eq(root, "generated_policy_skill", OPENAI_POLICY_CONTENT);
    assert_policy_missing(root, "external_false_override_skill");
    assert_policy_missing(root, "external_string_policy_skill");
    assert_policy_eq(root, "source_policy_skill", SOURCE_POLICY_CONTENT);
    assert_policy_eq(root, "source_policy_without_trigger_skill", SOURCE_POLICY_CONTENT);

    fs::write(policy_path(root, "generated_policy_skill"), "stale").unwrap();
    run_atb(&["build", "--clean", "--config"], &config_path);
    assert_policy_eq(root, "generated_policy_skill", OPENAI_POLICY_CONTENT);

    fs::write(
        policy_path(root, "source_policy_skill"),
        "policy:\n  allow_implicit_invocation: false\n",
    )
    .unwrap();
    fs::remove_file(policy_path(root, "source_policy_without_trigger_skill")).unwrap();
    run_atb(&["sync", "--config"], &config_path);

    assert!(
        !source_skill_dir(root, "generated_policy_skill")
            .join("agents/openai.yaml")
            .exists()
    );
    assert_eq!(
        fs::read_to_string(source_skill_dir(root, "source_policy_skill").join("agents/openai.yaml")).unwrap(),
        "policy:\n  allow_implicit_invocation: false\n"
    );
    assert!(
        !source_skill_dir(root, "source_policy_without_trigger_skill")
            .join("agents/openai.yaml")
            .exists()
    );
}

fn run_atb(args: &[&str], config_path: &Path) {
    let mut cmd = Command::new(assert_cmd::cargo_bin!("atb"));
    cmd.args(args).arg(config_path);
    cmd.assert().success();
}

fn setup_fixtures(root: &Path) -> PathBuf {
    write_command(
        root,
        "policy_cmd",
        "---
description: Policy command
disable-model-invocation: true
---
# Policy command",
        None,
    );
    write_command(
        root,
        "external_policy_cmd",
        "---
description: External policy command
---
# External policy command",
        Some(
            r#"codex:
  disable-model-invocation: true
"#,
        ),
    );

    write_skill(
        root,
        "generated_policy_skill",
        "---
description: Generated policy skill
disable-model-invocation: true
---
Generated policy skill",
        None,
        None,
    );
    write_skill(
        root,
        "external_false_override_skill",
        "---
description: External false override skill
disable-model-invocation: true
---
External false override skill",
        Some(
            r#"codex:
  disable-model-invocation: false
"#,
        ),
        None,
    );
    write_skill(
        root,
        "external_string_policy_skill",
        "---
description: External string policy skill
---
External string policy skill",
        Some(
            r#"codex:
  disable-model-invocation: "true"
"#,
        ),
        None,
    );
    write_skill(
        root,
        "source_policy_skill",
        "---
description: Source policy skill
disable-model-invocation: true
---
Source policy skill",
        None,
        Some(SOURCE_POLICY_CONTENT),
    );
    write_skill(
        root,
        "source_policy_without_trigger_skill",
        "---
description: Source policy without trigger skill
---
Source policy without trigger skill",
        None,
        Some(SOURCE_POLICY_CONTENT),
    );

    let codex_dir = root.join(".codex");
    fs::create_dir_all(&codex_dir).unwrap();
    let config_path = codex_dir.join("toolkit.yaml");
    fs::write(
        &config_path,
        format!(
            r#"
source: {}
target: codex
resources:
  commands:
    - plugin_a:policy_cmd
    - plugin_a:external_policy_cmd
  skills:
    - plugin_a:generated_policy_skill
    - plugin_a:external_false_override_skill
    - plugin_a:external_string_policy_skill
    - plugin_a:source_policy_skill
    - plugin_a:source_policy_without_trigger_skill
"#,
            root.display()
        ),
    )
    .unwrap();

    config_path
}

fn write_command(root: &Path, name: &str, markdown: &str, metadata: Option<&str>) {
    let command_dir = root.join("plugin_a/commands");
    fs::create_dir_all(&command_dir).unwrap();
    fs::write(command_dir.join(format!("{name}.md")), markdown).unwrap();

    if let Some(metadata) = metadata {
        fs::write(command_dir.join(format!("{name}.yaml")), metadata).unwrap();
    }
}

fn write_skill(root: &Path, name: &str, markdown: &str, metadata: Option<&str>, policy: Option<&str>) {
    let skill_dir = source_skill_dir(root, name);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), markdown).unwrap();

    if let Some(metadata) = metadata {
        fs::write(skill_dir.join("SKILL.yaml"), metadata).unwrap();
    }

    if let Some(policy) = policy {
        let policy_path = skill_dir.join("agents/openai.yaml");
        fs::create_dir_all(policy_path.parent().unwrap()).unwrap();
        fs::write(policy_path, policy).unwrap();
    }
}

fn source_skill_dir(root: &Path, name: &str) -> PathBuf {
    root.join("plugin_a").join("skills").join(name)
}

fn policy_path(root: &Path, name: &str) -> PathBuf {
    root.join(".agents")
        .join("skills")
        .join(name)
        .join("agents/openai.yaml")
}

fn assert_policy_eq(root: &Path, name: &str, expected: &str) {
    assert_eq!(fs::read_to_string(policy_path(root, name)).unwrap(), expected);
}

fn assert_policy_missing(root: &Path, name: &str) {
    assert!(!policy_path(root, name).exists());
}
