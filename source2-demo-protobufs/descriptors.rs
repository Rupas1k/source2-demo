use protox::prost::Message;
use std::{env, fs, path::PathBuf};

pub fn generate() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("protos");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed={}", root.display());
    for (game, extra) in [
        ("common", vec![]),
        ("dota", vec!["dota_usermessages.proto"]),
        ("citadel", vec!["citadel_usermessages.proto", "citadel_gameevents.proto"]),
        ("cs2", vec!["cstrike15_usermessages.proto", "cs_gameevents.proto"]),
    ] {
        let files = ["demo.proto", "netmessages.proto", "gameevents.proto", "usermessages.proto"]
            .into_iter()
            .chain(extra);
        let mut compiler = protox::Compiler::new([root.join(game), root.join("common")])?;
        compiler.include_imports(true).open_files(files)?;
        fs::write(out.join(format!("{game}.bin")), compiler.file_descriptor_set().encode_to_vec())?;
    }
    Ok(())
}
