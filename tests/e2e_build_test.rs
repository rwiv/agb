use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
#[allow(deprecated)]
fn test_e2e_build_gemini_cli() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // Setup fixtures
    setup_fixtures(root);

    // Create agb.yaml
    let config = r#"
target: gemini-cli
resources:
  commands:
    - plugin_a:foo
  skills:
    - plugin_c:python_expert
"#;
    fs::write(root.join("agb.yaml"), config).unwrap();

    // Run build
    let mut cmd = Command::cargo_bin("agb").unwrap();
    cmd.arg("build").arg("--config").arg(root.join("agb.yaml"));
    cmd.assert().success();

    // Verify outputs
    assert!(root.join("commands/foo.toml").exists());
    assert!(
        root.join("skills/python_expert/python_expert.toml")
            .exists()
    );

    let content = fs::read_to_string(root.join("commands/foo.toml")).unwrap();
    assert!(content.contains("prompt = \"# Foo Command\""));
    assert!(content.contains("model = \"gemini-1.5-pro\""));
}

#[test]
#[allow(deprecated)]
fn test_e2e_build_claude_code() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_fixtures(root);

    let config = r#"
target: claude-code
resources:
  commands:
    - plugin_a:foo
"#;
    fs::write(root.join("agb.yaml"), config).unwrap();

    let mut cmd = Command::cargo_bin("agb").unwrap();
    cmd.arg("build").arg("--config").arg(root.join("agb.yaml"));
    cmd.assert().success();

    assert!(root.join("commands/foo.md").exists());
    let content = fs::read_to_string(root.join("commands/foo.md")).unwrap();
    assert!(content.contains("description: Foo command description"));
    assert!(content.contains("# Foo Command"));
}

#[test]
#[allow(deprecated)]
fn test_e2e_build_opencode() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    setup_fixtures(root);

    let config = r#"
target: opencode
resources:
  commands:
    - plugin_a:foo
"#;
    fs::write(root.join("agb.yaml"), config).unwrap();

    let mut cmd = Command::cargo_bin("agb").unwrap();
    cmd.arg("build").arg("--config").arg(root.join("agb.yaml"));
    cmd.assert().success();

    assert!(root.join("commands/foo.md").exists());
    let content = fs::read_to_string(root.join("commands/foo.md")).unwrap();
    assert!(content.contains("metadata:"));
    assert!(content.contains("# Foo Command"));
}

fn setup_fixtures(root: &Path) {
    let plugins = root.join("plugins");
    let plugin_a_cmds = plugins.join("plugin_a/commands");
    let plugin_c_skills = plugins.join("plugin_c/skills/python_expert");

    fs::create_dir_all(&plugin_a_cmds).unwrap();
    fs::create_dir_all(&plugin_c_skills).unwrap();

    fs::write(plugin_a_cmds.join("foo.md"), "# Foo Command").unwrap();
    fs::write(
        plugin_a_cmds.join("foo.json"),
        r#"{"model": "gemini-1.5-pro", "description": "Foo command description"}"#,
    )
    .unwrap();

    fs::write(
        plugin_c_skills.join("python_expert.md"),
        "Python Expert Content",
    )
    .unwrap();
    fs::write(
        plugin_c_skills.join("METADATA.json"),
        r#"{"type": "expert"}"#,
    )
    .unwrap();

    fs::write(root.join("AGENTS.md"), "# Global Instructions").unwrap();
}
