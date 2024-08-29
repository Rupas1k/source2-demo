/// Main error type
#[derive(thiserror::Error, Debug)]
pub enum ParserError {
    #[error(transparent)]
    ProtobufDecode(#[from] crate::proto::prost::DecodeError),

    #[error(transparent)]
    SnapDecompress(#[from] snap::Error),

    #[error(transparent)]
    StringTable(#[from] StringTableError),

    #[error(transparent)]
    Class(#[from] ClassError),

    #[error(transparent)]
    Entity(#[from] EntityError),

    #[error(transparent)]
    FieldValue(#[from] FieldValueError),

    #[error(transparent)]
    GameEvent(#[from] GameEventError),

    #[error(transparent)]
    ObserverError(#[from] anyhow::Error),

    #[error("Wrong CDemoFileInfo offset")]
    ReplayEncodingError,

    #[error("Supports only Source 2 replays")]
    WrongMagic,

    #[cfg(feature = "dota")]
    #[error(transparent)]
    CombatLog(#[from] CombatLogError),

    #[cfg(feature = "deadlock")]
    #[error("CCitadelUserMsgPostMatchDetails not found")]
    MatchDetailsNotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum ClassError {
    #[error("Class not found for the given id {0}")]
    ClassNotFoundById(i32),

    #[error("Class not found for the given name {0}")]
    ClassNotFoundByName(String),
}

#[derive(thiserror::Error, Debug)]
pub enum EntityError {
    #[error("No entities found at index {0}")]
    IndexNotFound(usize),

    #[error("No entities found for handle {0}")]
    HandleNotFound(usize),

    #[error("No entities found for class with id {0}")]
    ClassIdNotFound(i32),

    #[error("No entities found for class with name {0}")]
    ClassNameNotFound(String),

    #[error("No property found for name {0} (Class: {1}, FieldPath: {2})")]
    PropertyNameNotFound(String, String, String),

    #[error(transparent)]
    FieldPathNotFound(#[from] SerializerError),
}

#[derive(thiserror::Error, Debug)]
pub enum FieldValueError {
    #[error("Cannot convert {0} into {1}")]
    ConversionError(String, String),
}

#[derive(thiserror::Error, Debug)]
pub enum GameEventError {
    #[error("Unknown key: {0}")]
    UnknownKey(String),
    #[error("Conversion error: {0} -> {1}")]
    ConversionError(String, String),
}

#[derive(thiserror::Error, Debug)]
pub enum SerializerError {
    #[error("No field path for given name {0}")]
    NoFieldPath(String),
}

#[derive(thiserror::Error, Debug)]
pub enum StringTableError {
    #[error("String table not found for the given id {0}")]
    TableNotFoundById(i32),

    #[error("String table not found for the given name {0}")]
    TableNotFoundByName(String),

    #[error("String table entry not found for the given index {0} ({1})")]
    RowNotFoundByIndex(i32, String),
}

#[derive(thiserror::Error, Debug)]
pub enum CombatLogError {
    #[error("No {0} for {1}")]
    EmptyProperty(String, String),
    #[error("No {0} for {1}")]
    EmptyName(String, String),
}
