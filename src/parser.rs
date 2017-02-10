
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
        leaf_payload_fraction: be_u8,

        || Header {
        	page_size: page_size,
        	read_version: read_version,
        	write_version: write_version,
        	reserved_space: reserved_space,
        	max_embedded_payload_fraction: max_embedded_payload_fraction,
        	min_embedded_payload_fraction: min_embedded_payload_fraction,
        	leaf_payload_fraction: leaf_payload_fraction,
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
