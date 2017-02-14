
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
    pub pages: Vec<BTreePage>,
}