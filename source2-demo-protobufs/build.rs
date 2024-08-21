fn main() -> std::io::Result<()> {
    let update = std::env::var_os("UPDATE_PROTOBUFS")
        .map(|v| v == "1")
        .unwrap_or(false);
    if update {
        let mut config = prost_build::Config::new();
        config.out_dir(".");
        config.default_package_filename("dota");
        config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
        config.compile_protos(
            &[
                "./protos/common/demo.proto",
                "./protos/common/gameevents.proto",
                "./protos/common/netmessages.proto",
                "./protos/common/network_connection.proto",
                "./protos/common/networkbasetypes.proto",
                "./protos/common/usermessages.proto",
                "./protos/dota/dota_commonmessages.proto",
                "./protos/dota/dota_modifiers.proto",
                "./protos/dota/dota_shared_enums.proto",
                "./protos/dota/dota_usermessages.proto",
            ],
            &["./protos/common", "./protos/dota"],
        )?;

        let mut config = prost_build::Config::new();
        config.out_dir(".");
        config.default_package_filename("citadel");
        config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
        config.compile_protos(
            &[
                "./protos/common/demo.proto",
                "./protos/common/gameevents.proto",
                "./protos/common/netmessages.proto",
                "./protos/common/network_connection.proto",
                "./protos/common/networkbasetypes.proto",
                "./protos/common/usermessages.proto",
                "./protos/citadel/citadel_gameevents.proto",
                "./protos/citadel/citadel_gcmessages_common.proto",
                "./protos/citadel/citadel_usermessages.proto",
            ],
            &["./protos/common", "./protos/citadel"],
        )?;
    }
    Ok(())
}
