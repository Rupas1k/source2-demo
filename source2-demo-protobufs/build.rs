use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const GITHUB_RAW_URL: &str = "https://raw.githubusercontent.com/SteamDatabase/Protobufs/master";

fn main() -> std::io::Result<()> {
    let update = std::env::var_os("UPDATE_PROTOBUFS").map(|v| v == "1").unwrap_or(false);
    let fetch_from_github = std::env::var_os("FETCH_PROTOBUFS_FROM_GITHUB").map(|v| v == "1").unwrap_or(false);

    if update {
        if fetch_from_github {
            fetch_protobufs_from_github()?;
        }
        let mut config = prost_build::Config::new();
        config.out_dir(".");
        config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

        config.default_package_filename("common");
        config.compile_protos(
            &[
                "./protos/common/base_gcmessages.proto",
                "./protos/common/demo.proto",
                "./protos/common/gameevents.proto",
                "./protos/common/gcsdk_gcmessages.proto",
                "./protos/common/netmessages.proto",
                "./protos/common/network_connection.proto",
                "./protos/common/networkbasetypes.proto",
                "./protos/common/source2_steam_stats.proto",
                "./protos/common/steammessages.proto",
                "./protos/common/steammessages_steamlearn.steamworkssdk.proto",
                "./protos/common/steammessages_unified_base.steamworkssdk.proto",
                "./protos/common/usermessages.proto",
                "./protos/common/valveextensions.proto",
            ],
            &["./protos/common"],
        )?;

        config.default_package_filename("dota");
        config.compile_protos(
            &[
                "./protos/dota/dota_commonmessages.proto",
                "./protos/dota/dota_modifiers.proto",
                "./protos/dota/dota_shared_enums.proto",
                "./protos/dota/dota_usermessages.proto",
            ],
            &["./protos/dota", "./protos/common"],
        )?;

        config.default_package_filename("citadel");
        config.compile_protos(
            &[
                "./protos/citadel/citadel_gameevents.proto",
                "./protos/citadel/citadel_gcmessages_common.proto",
                "./protos/citadel/citadel_usermessages.proto",
                "./protos/citadel/base_modifier.proto",
            ],
            &["./protos/common", "./protos/citadel"],
        )?;

        config.default_package_filename("cs2");
        config.compile_protos(
            &[
                "./protos/cs2/cs_gameevents.proto",
                "./protos/cs2/cstrike15_gcmessages.proto",
                "./protos/cs2/cstrike15_usermessages.proto",
                "./protos/cs2/engine_gcmessages.proto",
            ],
            &["./protos/common", "./protos/cs2"],
        )?;

        clean_rust_file("dota.rs")?;
        clean_rust_file("common.rs")?;
        clean_rust_file("citadel.rs")?;
        clean_rust_file("cs2.rs")?;

        clean_blocks("dota.rs", "common.rs")?;
        clean_blocks("citadel.rs", "common.rs")?;
        clean_blocks("cs2.rs", "common.rs")?;

        format_rust_file("dota.rs")?;
        format_rust_file("common.rs")?;
        format_rust_file("citadel.rs")?;
        format_rust_file("cs2.rs")?;
    }
    Ok(())
}

fn format_rust_file(filename: &str) -> std::io::Result<()> {
    use std::process::Command;

    let output = Command::new("rustfmt").arg(filename).output()?;

    if !output.status.success() {
        eprintln!("Warning: rustfmt failed for {}: {}", filename, String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

fn clean_impl_blocks(rust_code: &str) -> String {
    let mut cleaned_code = String::new();
    let mut in_impl_block = false;
    let mut brace_count = 0;

    for line in rust_code.lines() {
        let stripped_line = line.trim();

        if stripped_line.starts_with("impl") && stripped_line.contains("{") {
            in_impl_block = true;
            brace_count = stripped_line.matches('{').count() - stripped_line.matches('}').count();
            continue;
        }

        if in_impl_block {
            let new_brace_count = brace_count + stripped_line.matches('{').count() - stripped_line.matches('}').count();
            if new_brace_count == 0 {
                in_impl_block = false;
                brace_count = 0;
            } else {
                brace_count = new_brace_count;
            }
            continue;
        }

        cleaned_code.push_str(line);
        cleaned_code.push('\n');
    }

    cleaned_code
}

fn clean_comments(rust_code: &str) -> String {
    rust_code
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

fn clean_rust_file(filename: &str) -> std::io::Result<()> {
    let rust_code = fs::read_to_string(filename)?;
    let cleaned_code = clean_impl_blocks(&rust_code);
    let cleaned_code = clean_comments(&cleaned_code);

    let cleaned_code = cleaned_code
        .replace("Cnet", "CNet")
        .replace("Csvc", "CSvc")
        .replace("Cdota", "CDota")
        .replace("Ccs", "CCs")
        .replace("Cso", "CSo")
        .replace("Cgc", "CGc")
        .replace("Cip", "CIp");

    fs::write(filename, cleaned_code)
}

fn extract_blocks(rust_code: &str) -> HashMap<String, String> {
    let mut blocks = HashMap::new();
    let mut current_block = String::new();
    let mut current_name = String::new();
    let mut brace_count = 0;

    for line in rust_code.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.contains("{") {
            brace_count += line.matches('{').count();
            if (line.contains("pub struct ") || line.contains("pub enum ") || line.contains("pub mod ")) && current_name.is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                current_name = parts[2].to_string();
            }
        }

        if line.contains("}") {
            brace_count -= line.matches('}').count();
            if brace_count == 0 {
                current_block.push_str(line);
                blocks.insert(current_name.clone(), current_block.clone());
                current_name.clear();
                current_block.clear();
                continue;
            }
        }

        current_block.push_str(line);
        current_block.push('\n');
    }

    blocks
}

fn clean_matching_blocks(input_blocks: &HashMap<String, String>, common_blocks: &HashMap<String, String>) -> Vec<String> {
    input_blocks
        .iter()
        .sorted()
        .filter_map(|(name, block)| if !common_blocks.contains_key(name) { Some(block.clone()) } else { None })
        .collect()
}

fn clean_blocks(input_file: &str, common_file: &str) -> std::io::Result<()> {
    let input_code = fs::read_to_string(input_file)?;
    let common_code = fs::read_to_string(common_file)?;

    let input_blocks = extract_blocks(&input_code);
    let common_blocks = extract_blocks(&common_code);

    let cleaned_blocks = clean_matching_blocks(&input_blocks, &common_blocks);

    let mut result = String::from("pub use crate::common::*;\n\n");
    result.push_str(&cleaned_blocks.join("\n\n"));

    fs::write(input_file, result)
}

fn fetch_protobufs_from_github() -> std::io::Result<()> {
    let proto_files = vec![
        (
            "dota2",
            "common",
            vec![
                "base_gcmessages.proto",
                "demo.proto",
                "gameevents.proto",
                "gcsdk_gcmessages.proto",
                "netmessages.proto",
                "network_connection.proto",
                "networkbasetypes.proto",
                "source2_steam_stats.proto",
                "steammessages.proto",
                "steammessages_steamlearn.steamworkssdk.proto",
                "steammessages_unified_base.steamworkssdk.proto",
                "usermessages.proto",
                "valveextensions.proto",
            ],
        ),
        (
            "dota2",
            "dota",
            vec![
                "dota_commonmessages.proto",
                "dota_modifiers.proto",
                "dota_shared_enums.proto",
                "dota_usermessages.proto",
                "events.proto",
            ],
        ),
        (
            "deadlock",
            "citadel",
            vec![
                "citadel_gameevents.proto",
                "citadel_gcmessages_common.proto",
                "citadel_usermessages.proto",
                "base_modifier.proto",
            ],
        ),
        (
            "csgo",
            "cs2",
            vec![
                "cs_gameevents.proto",
                "cstrike15_gcmessages.proto",
                "cstrike15_usermessages.proto",
                "engine_gcmessages.proto",
            ],
        ),
    ];

    for (github_dir, local_dir, files) in proto_files {
        let proto_dir = format!("./protos/{}", local_dir);
        fs::create_dir_all(&proto_dir)?;

        for file in files {
            let url = format!("{}/{}/{}", GITHUB_RAW_URL, github_dir, file);

            match ureq::get(&url).call() {
                Ok(mut response) => {
                    let content = response
                        .body_mut()
                        .read_to_string()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to read response: {}", e)))?
                        .replace("optional string player_name = 2;", "optional bytes player_name = 2;");

                    let file_path = format!("{}/{}", proto_dir, file);
                    fs::write(&file_path, content)?;
                }
                Err(e) => {
                    eprintln!("Failed to fetch {}: {}", file, e);
                    return Err(std::io::Error::other(format!("Failed to fetch {}: {}", file, e)));
                }
            }
        }
    }

    Ok(())
}
