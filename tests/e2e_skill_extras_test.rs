use assert_cmd::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_e2e_skill_extra_files_inclusion() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 1. Setup fixtures with extra files in a skill
    let plugins = root.join("plugins");
    let skill_dir = plugins.join("my_plugin/skills/heavy_skill");
    fs::create_dir_all(&skill_dir).unwrap();

    fs::write(skill_dir.join("SKILL.md"), "Heavy Skill Content").unwrap();
    fs::write(skill_dir.join("logic.py"), "print('hello')").unwrap();
    fs::write(skill_dir.join("data.json"), r#"{"key": "value"}"#).unwrap();

    // 2. Create agb.yaml
    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  skills:
    - my_plugin:heavy_skill
"#,
        root.display()
    );
    fs::write(root.join("agb.yaml"), config).unwrap();

    // 3. Run build
    let mut cmd = Command::new(assert_cmd::cargo_bin!("agb"));
    cmd.arg("build").arg("--config").arg(root.join("agb.yaml"));
    cmd.assert().success();

    // 4. Verify outputs
    let output_skill_dir = root.join("skills/heavy_skill");
    assert!(output_skill_dir.join("SKILL.md").exists());
    assert!(output_skill_dir.join("logic.py").exists());
    assert!(output_skill_dir.join("data.json").exists());

    assert_eq!(
        fs::read_to_string(output_skill_dir.join("logic.py")).unwrap(),
        "print('hello')"
    );
    assert_eq!(
        fs::read_to_string(output_skill_dir.join("data.json")).unwrap(),
        r#"{"key": "value"}"#
    );
}

#[test]
fn test_e2e_skill_extra_files_clean_behavior() {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    // 1. Setup fixtures
    let plugins = root.join("plugins");
    let skill_dir = plugins.join("my_plugin/skills/simple_skill");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), "content").unwrap();

    // 2. Pre-create some junk in the output directory
    let output_skill_dir = root.join("skills/simple_skill");
    fs::create_dir_all(&output_skill_dir).unwrap();
    fs::write(output_skill_dir.join("junk.txt"), "obsolete").unwrap();

    // 3. Create agb.yaml
    let config = format!(
        r#"
source: {}
target: gemini-cli
resources:
  skills:
    - my_plugin:simple_skill
"#,
        root.display()
    );
    fs::write(root.join("agb.yaml"), config).unwrap();

    // 4. Run build
    let mut cmd = Command::new(assert_cmd::cargo_bin!("agb"));
    cmd.arg("build").arg("--config").arg(root.join("agb.yaml"));
    cmd.assert().success();

    // 5. Verify junk is gone and skill is present
    assert!(!output_skill_dir.join("junk.txt").exists());
    assert!(output_skill_dir.join("SKILL.md").exists());
}
