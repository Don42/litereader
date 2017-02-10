extern crate std;

#[derive(Debug)]
pub enum ParserError {
    UnknownValue(u32),
}


#[derive(Debug)]
pub enum TextEncoding {
    UTF8,
    UTF16le,
    UTF16be,
}

pub fn get_text_encoding(val: u32) -> Result<TextEncoding, ParserError> {
    match val {
        1 => Ok(TextEncoding::UTF8),
        2 => Ok(TextEncoding::UTF16le),
        3 => Ok(TextEncoding::UTF16be),
        x => Err(ParserError::UnknownValue(x)),
    }
}

impl std::fmt::Display for TextEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}",
               match self {
                   &TextEncoding::UTF8 => "UTF-8",
                   &TextEncoding::UTF16le => "UTF-16le",
                   &TextEncoding::UTF16be => "UTF-16be",
               })
    }
}


#[derive(Debug)]
pub enum SchemaFormat {
    Format1,
    Format2,
    Format3,
    Format4,
}

pub fn get_schema_format(val: u32) -> Result<SchemaFormat, ParserError> {
    match val {
        1 => Ok(SchemaFormat::Format1),
        2 => Ok(SchemaFormat::Format2),
        3 => Ok(SchemaFormat::Format3),
        4 => Ok(SchemaFormat::Format4),
        x => Err(ParserError::UnknownValue(x)),
    }
}

impl std::fmt::Display for SchemaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}",
               match self {
                   &SchemaFormat::Format1 => "1",
                   &SchemaFormat::Format2 => "2",
                   &SchemaFormat::Format3 => "3",
                   &SchemaFormat::Format4 => "4",
               })
    }
}


#[derive(Debug)]
pub enum WriteVersion {
    Legacy,
    WAL,
}

pub fn get_write_version(val: u8) -> Result<WriteVersion, ParserError> {
    match val {
        1 => Ok(WriteVersion::Legacy),
        2 => Ok(WriteVersion::WAL),
        x => Err(ParserError::UnknownValue(x as u32)),
    }
}

impl std::fmt::Display for WriteVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}",
               match self {
                   &WriteVersion::Legacy => "Legacy",
                   &WriteVersion::WAL => "WAL",
               })
    }
}


#[derive(Debug)]
pub enum ReadVersion {
    Legacy,
    WAL,
}

pub fn get_read_version(val: u8) -> Result<ReadVersion, ParserError> {
    match val {
        1 => Ok(ReadVersion::Legacy),
        2 => Ok(ReadVersion::WAL),
        x => Err(ParserError::UnknownValue(x as u32)),
    }
}

impl std::fmt::Display for ReadVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}",
               match self {
                   &ReadVersion::Legacy => "Legacy",
                   &ReadVersion::WAL => "WAL",
               })
    }
}

