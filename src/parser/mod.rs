mod generic;
mod header;
mod page;

use nom::{IResult, ErrorKind};

use data_structures::{Header, BTreePageHeader};
pub use parser::header::header_parser;
use parser::page::{btree_page_parser, btree_page_header_parser};

const HEADER_STRING: &'static str = "SQLite format 3\0";
const PAGE_SIZE_MAX: u32 = 65536;

#[derive(Debug)]
pub enum ParserError {
    UnknownValueU8(u8),
    UnknownValueU16(u16),
    UnknownValueU32(u32),
}

/*
pub fn parse_sqlite_file(i: &[u8]) -> Result<SqliteFile, String> {
    let (_, file_header) = match header_parser(i) {
        IResult::Done(x, y) => (x, y),
        IResult::Error(ErrorKind::Tag) => {
            return Err("File is not SQLite Database".to_string())
        },
        IResult::Error(x) => {
            println!("{:?}", x);
            return Err("Error parsing header".to_string())
        },
        IResult::Incomplete(_) => { return Err("Incomplete header".to_string()) },
    };
    println!("{:?}", file_header);
    let mut page_list: Vec<BTreePage> = vec![];
    for page_id in 0..(file_header.database_size - 1) {
        let start: usize = (page_id * file_header.page_size) as usize;
        let end: usize = ((page_id + 1) * file_header.page_size) as usize;
        let btree_page = match btree_page_parser(&i[start..end]) {
            IResult::Done(_, Some(y)) => Some(y),
            IResult::Done(_, None) => None,
            IResult::Error(ErrorKind::MapRes) => {
                println!("{} | MapRes | {:?}", page_id, &i[start..(start+8)]);
                None
            }
            IResult::Error(e) => {
                println!("{} | {}", page_id, e.description());
                None
            },
            IResult::Incomplete(x) => {
                println!("Incomplete {:?} {}: {:?}", &i[start..start+9], page_id, x);
                return Err(format!("Incomplete {}: {:?}", page_id, x).to_string())
            },
        };
        match btree_page {
            Some(page) => page_list.push(page.clone()),
            None => (),
        }
    }
    /*for cell_pointer in btree_page.cell_pointer {
        let (cell_size, row_id) = match do_parse!(&i[cell_pointer as usize..],
            cell_size: parse_varint >>
            row_id: parse_varint >>
            (cell_size, row_id)) {
            IResult::Done(x, y) => y,
            IResult::Error(_) => { return Err("Error".to_string()) },
            IResult::Incomplete(_) => { return Err("Incomplete".to_string()) },
        };
        println!("{}_{}", cell_size, row_id);
    };*/

    Ok(SqliteFile {
        header: file_header,
        buffer: i.to_vec(),
    })
}*/

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

