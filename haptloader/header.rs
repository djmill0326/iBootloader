use std::{sync::Arc, borrow::Cow};

pub struct File(usize);

pub fn file_io() -> File {
    File(0)
}

pub type Semver = Cow<'static, u16>;

static mut SEMVER: Semver = Cow::Owned(0);

pub fn semver() -> Semver {
    Cow::Owned(unsafe { SEMVER.wrapping_add(1) })
}

pub struct Version(u8, u8, Semver);

impl Version {
    pub fn x(i: u8) -> Self {
        Self(i, 0, semver())
    }

    pub fn xx(i: u8, j: u16) -> Self {
        Self(i, (j % 0xFF).try_into().expect("u8 overflow"), semver())
    }

    pub fn xxx(i: u8, j: u8) -> Self {
        Self(i, j, semver())
    }

    pub fn v0() -> Self {
        Self::xxx(1, 0)
    }
}

pub struct HeaderItem {
    name: Arc<str>,
    version: Version
}

impl HeaderItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            version: Version::v0()
        }
    }
}

pub struct HeaderData {
    header: HeaderItem,
    data: HeaderItem
}

impl HeaderData {
    pub fn new(name: &str) -> Self {
        Self {
            header: HeaderItem::new(name),
            data: HeaderItem::new(Arc::new("data-".to_owned() + name + semver().to_string().as_ref()).as_ref())
        }
    }
}

pub struct HeaderFunction {
    header: HeaderItem,
    function: HeaderData
}

impl HeaderFunction {
    pub fn new(name: &str) -> Self {
        Self {
            header: HeaderItem::new(name),
            function: HeaderData::new(Arc::new("fn-".to_owned() + name + semver().to_string().as_ref()).as_ref())
        }
    }
}

pub struct Header(HeaderItem, HeaderFunction, HeaderData);

pub mod parser {
    use super::{Header, HeaderItem, HeaderFunction, HeaderData};

    pub struct ParseState {
        header: HeaderItem,
        intern: HeaderData,
        data: HeaderData,
        functions: HeaderFunction
    }

    impl ParseState {
        pub fn init() -> Self {
            Self {
                header: HeaderItem::new("parser"),
                intern: HeaderData::new("interner"),
                data: HeaderData::new("header-data"),
                functions: HeaderFunction::new("header-fns")
            }
        }
    }

    pub struct Parser(ParseState);

    pub fn parse(_header: Header) -> Parser {
        Parser(ParseState::init())
    }
}

pub mod header {
    use super::parser;

    pub struct Header(super::Header, parser::Parser);
}

pub fn init() {
    println!("[hapt-header] init...");
}