
use nom::{IResult, be_u16, be_u8, be_u32};

use enums;

const HEADER_STRING: &'static str = "SQLite format 3\0";
const PAGE_SIZE_MAX: u32 = 65536;

#[derive(Debug)]
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


named!(header_parser<Header>,
    chain!(
        tag!(HEADER_STRING) ~
        page_size: page_size_parser ~
        read_version: read_version_parser ~
        write_version: write_version_parser ~
        reserved_space: be_u8 ~
        max_embedded_payload_fraction: be_u8 ~
        min_embedded_payload_fraction: be_u8 ~
        leaf_payload_fraction: be_u8 ~
        file_change_counter: be_u32 ~
        database_size: be_u32 ~
        freelist_trunk_page: be_u32 ~
        freelist_count: be_u32 ~
        schema_cookie: be_u32 ~
        schema_format: schema_format_parser ~
        default_page_cache_size: be_u32 ~
        largest_root_page: be_u32 ~
        text_encoding: text_encoding_parser ~
        user_version: be_u32 ~
        incremental_vacuum_mode: vacuum_mode_parser ~
        application_id: be_u32 ~
        take!(20) ~
        version_valid: be_u32 ~
        sqlite_version: be_u32,

        || Header {
            page_size: page_size,
            read_version: read_version,
            write_version: write_version,
            reserved_space: reserved_space,
            max_embedded_payload_fraction: max_embedded_payload_fraction,
            min_embedded_payload_fraction: min_embedded_payload_fraction,
            leaf_payload_fraction: leaf_payload_fraction,
            file_change_counter: file_change_counter,
            database_size: database_size,
            freelist_trunk_page: freelist_trunk_page,
            freelist_count: freelist_count,
            schema_cookie: schema_cookie,
            schema_format: schema_format,
            default_page_cache_size: default_page_cache_size,
            largest_root_page: largest_root_page,
            text_encoding: text_encoding,
            user_version: user_version,
            incremental_vacuum_mode: incremental_vacuum_mode,
            application_id: application_id,
            version_valid_for: version_valid,
            sqlite_version: sqlite_version,
        }
    )
);

named!(page_size_parser<u32>,
    map_res!(
        be_u16,
        get_page_size
    )
);

named!(read_version_parser<enums::ReadVersion>,
    map_res!(
        be_u8,
        enums::get_read_version
    )
);

named!(write_version_parser<enums::WriteVersion>,
    map_res!(
        be_u8,
        enums::get_write_version
    )
);

named!(schema_format_parser<enums::SchemaFormat>,
    map_res!(
        be_u32,
        enums::get_schema_format
    )
);

named!(text_encoding_parser<enums::TextEncoding>,
    map_res!(
        be_u32,
        enums::get_text_encoding
    )
);

named!(vacuum_mode_parser<bool>,
    map_res!(
        be_u32,
        |x: u32| match x {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Unknown vacuum mode".to_string()),
        }
    )
);


pub fn parse_header(buffer: &[u8]) -> Result<Header, String> {
    match header_parser(buffer) {
        IResult::Done(_, y) => Ok(y),
        IResult::Error(_) => Err("Error".to_string()),
        IResult::Incomplete(_) => Err("Incomplete".to_string()),
    }
}

fn get_page_size(size: u16) -> Result<u32, String> {
    Ok(if size == 1 {
        PAGE_SIZE_MAX
    } else {
        size as u32
    })
}
