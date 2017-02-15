
use nom::{IResult, be_u16, be_u8, be_u32};

use enums;
use data_structures::{Header};
use parser::{ParserError, PAGE_SIZE_MAX, HEADER_STRING};


pub fn header_parser(i: &[u8]) -> IResult<&[u8], Header> {
    chain!(i,
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
}

named!(page_size_parser<u32>,
    map_res!(
        be_u16,
        |x: u16| -> Result<u32, String> {
            match x {
                1 => Ok(PAGE_SIZE_MAX),
                _ => Ok(x as u32),
            }
        }
    )
);

named!(read_version_parser<enums::ReadVersion>,
    map_res!(
        be_u8,
        |x: u8| -> Result<enums::ReadVersion, ParserError> {
            match x {
                1 => Ok(enums::ReadVersion::Legacy),
                2 => Ok(enums::ReadVersion::WAL),
                x => Err(ParserError::UnknownValueU8(x)),
            }
        }
    )
);

named!(write_version_parser<enums::WriteVersion>,
    map_res!(
        be_u8,
        |x: u8| -> Result<enums::WriteVersion, ParserError> {
            match x {
                1 => Ok(enums::WriteVersion::Legacy),
                2 => Ok(enums::WriteVersion::WAL),
                x => Err(ParserError::UnknownValueU8(x)),
            }
        }
    )
);

named!(schema_format_parser<enums::SchemaFormat>,
    map_res!(
        be_u32,
        |x: u32| -> Result<enums::SchemaFormat, ParserError> {
            match x {
                1 => Ok(enums::SchemaFormat::Format1),
                2 => Ok(enums::SchemaFormat::Format2),
                3 => Ok(enums::SchemaFormat::Format3),
                4 => Ok(enums::SchemaFormat::Format4),
                x => Err(ParserError::UnknownValueU32(x)),
            }
        }
    )
);

named!(text_encoding_parser<enums::TextEncoding>,
    map_res!(
        be_u32,
        |x: u32| -> Result<enums::TextEncoding, ParserError> {
            match x {
                1 => Ok(enums::TextEncoding::UTF8),
                2 => Ok(enums::TextEncoding::UTF16le),
                3 => Ok(enums::TextEncoding::UTF16be),
                x => Err(ParserError::UnknownValueU32(x)),
            }
        }
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

