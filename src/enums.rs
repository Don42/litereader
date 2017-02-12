extern crate std;


#[derive(Debug)]
pub enum TextEncoding {
    UTF8,
    UTF16le,
    UTF16be,
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
