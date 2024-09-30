use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    version: String,
}

#[derive(Deserialize)]
struct Schema {
    info: Info,
}

const MAJOR_VERSION: &str = "3";

fn get_upstream_version() -> String {
    let schema_raw = std::fs::read_to_string("../schema.yml").unwrap();
    let schema: Schema = serde_yml::from_str(&schema_raw).unwrap();

    let version = schema.info.version;
    let version_split: Vec<&str> = version.split('.').collect();
    format!(
        "{}{:02}{}",
        version_split[0],
        version_split[1].parse::<i32>().unwrap(),
        version_split[2],
    )
}

fn get_patch(upstream_version: &str) -> i32 {
    let last_tag_output = std::process::Command::new("git")
        .args(&["describe", "--abbrev=0", "--tags"])
        .output()
        .unwrap();
    let last_tag = String::from_utf8(last_tag_output.stdout).unwrap();
    let last_tag = last_tag.trim();

    if last_tag.len() == 0 {
        return 0;
    }

    let last_tag_split: Vec<&str> = last_tag.split('.').collect();
    let last_upstream = last_tag_split[1];
    if last_upstream != upstream_version {
        return 0;
    }

    let last_rev = last_tag_split[2].parse::<i32>().unwrap(); 
    return last_rev + 1;
}

fn main() {
    let upstream_version = get_upstream_version();
    let patch = get_patch(&upstream_version);
    println!("{MAJOR_VERSION}.{upstream_version}.{patch}");
}
