use nom::{IResult, be_u16, be_u8, be_u32};

use parser::{HEADER_STRING, ParserError};
use data_structures::{BTreePageHeader, BTreePageType, BTreePage};

const CONTENT_OFFSET_MAX: u32 = 65536;

pub fn btree_page_header_parser(i: &[u8]) -> IResult<&[u8], BTreePageHeader> {
    chain!(i,
        page_type: btree_page_type_parser ~
        freeblock_offset: freeblock_offset_parser ~
        cell_count: be_u16 ~
        cell_content_offset: map_res!(
            be_u16,
            |x: u16| -> Result<u32, ParserError> {
                match x {
                    0 => Ok(CONTENT_OFFSET_MAX),
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
            fragmented_free_byte_count: fragmented_free_byte_count,
            right_most_pointer: right_most_pointer,
        }
    )
}

pub fn btree_page_parser(i: &[u8]) -> IResult<&[u8], Option<BTreePage>> {
    alt!(i, chain!(
            tag!("\0\0"),
            || None) |
        chain!(
            opt!(ignore_file_header) ~
            header: btree_page_header_parser ~
            cell_pointer: count!(be_u16, header.cell_count as usize),
        || {
                // let mut cell_pointer = cell_pointer.clone();
                // cell_pointer.sort();
                Some(BTreePage {
                header: header,
                cell_pointer: cell_pointer,
                })
        }
    ))
}

named!(ignore_file_header<()>,
    do_parse!(
        tag!(HEADER_STRING) >>
        take!(84) >>
        ()
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
                0x00 => Ok(BTreePageType::NullPage),
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
        BTreePageType::LeafIndexPage | BTreePageType::LeafTablePage => IResult::Done(i, None),
        BTreePageType::NullPage => IResult::Done(i, None),
    }
}
