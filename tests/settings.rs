use achievement_hunter::settings::{Settings, Steam};
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;

#[test]
fn settings_parse_correctly() {
    let tmp_dir = TempDir::new("achievement_hunter").unwrap();
    let file_path = tmp_dir.path().join("config.toml");
    let mut tmp_file = File::create(file_path.clone()).unwrap();
    write!(
        tmp_file,
        "[steam]\napi_key = \"test_api_key\"\nuser_id = \"test_user_id\""
    )
    .unwrap();

    let settings = Settings::new(&file_path).unwrap();

    let steam = Steam {
        api_key: "test_api_key".to_string(),
        user_id: "test_user_id".to_string(),
    };
    assert_eq!(settings, Settings { steam: steam });
}
