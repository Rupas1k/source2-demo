use std::any::Any;

/// Script for updating protobufs
fn main() -> std::io::Result<()> {
    let mut config = prost_build::Config::new();
    config.out_dir("./generated");
    config.default_package_filename("proto");
    config.compile_protos(
        &[
            "./protos/demo.proto",
            "./protos/dota_commonmessages.proto",
            "./protos/dota_shared_enums.proto",
            "./protos/dota_usermessages.proto",
            "./protos/gameevents.proto",
            "./protos/netmessages.proto",
            "./protos/network_connection.proto",
            "./protos/networkbasetypes.proto",
            "./protos/usermessages.proto",
        ],
        &["./protos"],
    )?;
    Ok(())
}
