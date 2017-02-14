use nom::{IResult, be_u16, be_u8, be_u32, ErrorKind, Needed};

use enums;
use data_structures::{Header, BTreePageHeader, BTreePageType, SqliteFile, BTreePage};

const HEADER_STRING: &'static str = "SQLite format 3\0";
const PAGE_SIZE_MAX: u32 = 65536;


#[derive(Debug)]
pub enum ParserError {
    UnknownValueU8(u8),
    UnknownValueU16(u16),
    UnknownValueU32(u32),
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

named!(ignore_file_header<()>,
    do_parse!(
        tag!(HEADER_STRING) >>
        take!(84) >>
        ()
    )
);

named!(btree_page_parser<BTreePage>,
    chain!(
        opt!(ignore_file_header) ~
        header: btree_page_header_parser ~
        cell_pointer: count!(be_u16, header.cell_count as usize),
        || {
                // let mut cell_pointer = cell_pointer.clone();
                // cell_pointer.sort();
                BTreePage {
                header: header,
                cell_pointer: cell_pointer,
                }
        }
    )
);

named!(btree_page_header_parser<BTreePageHeader>,
    chain!(
        page_type: btree_page_type_parser ~
        freeblock_offset: freeblock_offset_parser ~
        cell_count: be_u16 ~
        cell_content_offset: map_res!(
            be_u16,
            |x: u16| -> Result<u32, ParserError> {
                match x {
                    0 => Ok(65536),
                    x => Ok(x as u32),
                }
            }
        ) ~
        fragmented_free_byte_count: be_u8 ~
        right_most_pointer: apply!(parse_right_most_pointer, &page_type),

        || BTreePageHeader {
            page_type: page_type,
            freeblock_offset: freeblock_offset,
            cell_count: cell_count,
            cell_content_offset: cell_content_offset,
            fragmented_free_byte_count: 0,
            right_most_pointer: right_most_pointer,
        }
    )
);

named!(freeblock_offset_parser<Option<u16>>,
    map_res!(
        be_u16,
        |x: u16| -> Result<Option<u16>, ParserError> {
            match x {
                0 => Ok(None),
                x => Ok(Some(x)),
            }
        }
    )
);

named!(btree_page_type_parser<BTreePageType>,
    map_res!(
        be_u8,
        |x: u8| {
            match x {
                0x02 => Ok(BTreePageType::InteriorIndexPage),
                0x05 => Ok(BTreePageType::InteriorTablePage),
                0x0a => Ok(BTreePageType::LeafIndexPage),
                0x0d => Ok(BTreePageType::LeafTablePage),
                x => Err(ParserError::UnknownValueU8(x)),
            }
        }
    )
);

/*
 * Parse the pointer for interior btree pages and skip for leaf pages
 */
fn parse_right_most_pointer<'a>(i: &'a [u8], page_type: &BTreePageType)
                                -> IResult<&'a [u8], Option<u32>> {
    match *page_type {
        BTreePageType::InteriorIndexPage | BTreePageType::InteriorTablePage => {
            do_parse!(i,
                pointer: be_u32 >>
                (Some(pointer)))
        },
        BTreePageType::LeafIndexPage | BTreePageType::LeafTablePage => IResult::Done(i, None)
    }
}

pub fn parse_sqlite_file(i: &[u8]) -> Result<SqliteFile, String> {
    let (o1, file_header) = match header_parser(i) {
        IResult::Done(x, y) => (x, y),
        IResult::Error(_) => {
            return Err("Error".to_string())
        },
        IResult::Incomplete(_) => { return Err("Incomplete".to_string()) },
    };
    let (o2, btree_page) = match btree_page_parser(o1) {
        IResult::Done(x, y) => (x, y),
        IResult::Error(_) => {
            return Err("Error".to_string())
        },
        IResult::Incomplete(_) => { return Err("Incomplete".to_string()) },
    };
    let mut page_list: Vec<BTreePage> = vec![btree_page.clone()];
    for cell_pointer in btree_page.cell_pointer {
        let (cell_size, row_id) = match do_parse!(&i[cell_pointer as usize..],
            cell_size: parse_varint >>
            row_id: parse_varint >>
            (cell_size, row_id)) {
            IResult::Done(x, y) => y,
            IResult::Error(_) => { return Err("Error".to_string()) },
            IResult::Incomplete(_) => { return Err("Incomplete".to_string()) },
        };
        println!("{}_{}", cell_size, row_id);
    };

    Ok(SqliteFile {
        header: file_header,
        pages: page_list,
    })
}

pub fn parse_header(buffer: &[u8]) -> Result<Header, String> {
    match header_parser(buffer) {
        IResult::Done(_, y) => Ok(y),
        IResult::Error(_) => Err("Error".to_string()),
        IResult::Incomplete(_) => Err("Incomplete".to_string()),
    }
}

pub fn parse_btree_page_header(buffer: &[u8]) -> Result<BTreePageHeader, String> {
    match btree_page_header_parser(buffer) {
        IResult::Done(_, y) => Ok(y),
        IResult::Error(_) => Err("Error".to_string()),
        IResult::Incomplete(_) => Err("Incomplete".to_string()),
    }
}

fn parse_varint(i: &[u8]) -> IResult<&[u8], u64> {
    let mut content: u64 = 0;
    let mut count: usize = 0;
    for &c in i {
        if count < 8 {
            content <<= 7;
            content += (c & 0x7Fu8) as u64
        } else {
            content <<= 8;
            content += c as u64;
        }
        if (0x80u8 & c) == 0 || count == 8 {
            return IResult::Done(&i[count..], content);
        }
        count += 1;
    }
    return IResult::Incomplete(Needed::Size(9 - count))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parser_it_works() {
        let buffer: [u8; 16] = [0x81, 0x3e, 0x05, 0x07, 0x17, 0x29, 0x29, 0x01, 0x82,
            0x37, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x6f];
        let (i, res) = super::parse_varint(&buffer).unwrap();
        assert_eq!(190, res);
    }
}
