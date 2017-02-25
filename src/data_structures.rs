extern crate std;

use enums;

#[derive(Debug,Copy,Clone)]
pub struct Header {
    pub page_size: u32,
    pub read_version: enums::ReadVersion,
    pub write_version: enums::WriteVersion,
    pub reserved_space: u8,
    pub max_embedded_payload_fraction: u8,
    pub min_embedded_payload_fraction: u8,
    pub leaf_payload_fraction: u8,
    pub file_change_counter: u32,
    pub database_size: u32,
    pub freelist_trunk_page: u32,
    pub freelist_count: u32,
    pub schema_cookie: u32,
    pub schema_format: enums::SchemaFormat,
    pub default_page_cache_size: u32,
    pub largest_root_page: u32,
    pub text_encoding: enums::TextEncoding,
    pub user_version: u32,
    pub incremental_vacuum_mode: bool,
    pub application_id: u32,
    pub version_valid_for: u32,
    pub sqlite_version: u32,
}


#[derive(Debug,Copy,Clone)]
pub enum BTreePageType {
    InteriorIndexPage,
    InteriorTablePage,
    LeafIndexPage,
    LeafTablePage,
    NullPage,
}

#[derive(Debug,Copy,Clone)]
pub struct BTreePageHeader {
    pub page_type: BTreePageType,
    pub freeblock_offset: Option<u16>,
    pub cell_count: u16,
    pub cell_content_offset: u32,
    pub fragmented_free_byte_count: u8,
    pub right_most_pointer: Option<u32>,
}

#[derive(Debug,Clone)]
pub struct BTreePage {
    pub header: BTreePageHeader,
    pub cell_pointer: Vec<u16>,
}

#[derive(Debug)]
pub struct SqliteFile {
    pub header: Header,
    buffer: Vec<u8>,
}

impl SqliteFile {
    pub fn new(header: Header, buffer: Vec<u8>) -> SqliteFile {
        SqliteFile {
            header: header,
            buffer: buffer,
        }
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{:?}",
               self
        )
    }
}

impl std::fmt::Display for SqliteFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{}",
               self.header
        )
    }
}


#[cfg(test)]
mod tests {
    use enums;
    #[test]
    fn test_header_print() {
        let header = super::Header {
            page_size: 1,
            read_version: enums::ReadVersion::Legacy,
            write_version: enums::WriteVersion::Legacy,
            reserved_space: 0,
            max_embedded_payload_fraction: 64,
            min_embedded_payload_fraction: 32,
            leaf_payload_fraction: 32,
            file_change_counter: 4223,
            database_size: 100012020,
            freelist_trunk_page: 0,
            freelist_count: 0,
            schema_cookie: 15,
            schema_format: enums::SchemaFormat::Format1,
            default_page_cache_size: 1_000_000_000,
            largest_root_page: 0,
            text_encoding: enums::TextEncoding::UTF8,
            user_version: 123123,
            incremental_vacuum_mode: true,
            application_id: 111_111,
            version_valid_for: 1234567,
            sqlite_version: 3008008,
        };
        let display = format!("{}", header);
        assert_eq!(
            display,
            "Header { \
            page_size: 1, \
            read_version: Legacy, \
            write_version: Legacy, \
            reserved_space: 0, \
            max_embedded_payload_fraction: 64, \
            min_embedded_payload_fraction: 32, \
            leaf_payload_fraction: 32, \
            file_change_counter: 4223, \
            database_size: 100012020, \
            freelist_trunk_page: 0, \
            freelist_count: 0, \
            schema_cookie: 15, \
            schema_format: Format1, \
            default_page_cache_size: 1000000000, \
            largest_root_page: 0, \
            text_encoding: UTF8, \
            user_version: 123123, \
            incremental_vacuum_mode: true, \
            application_id: 111111, \
            version_valid_for: 1234567, \
            sqlite_version: 3008008 }"
        )
    }
}