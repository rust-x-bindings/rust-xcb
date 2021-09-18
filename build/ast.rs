use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ExtInfo {
    pub name: String,
    pub xname: String,
    pub major_version: u32,
    pub minor_version: u32,
}

#[derive(Debug, Clone)]
pub struct DocField {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub brief: String,
    pub text: String,
    pub fields: Vec<DocField>,
}

#[derive(Debug, Clone)]
pub enum Expr<T>
where
    T: FromStr,
    T: Clone,
{
    FieldRef(String),
    ParamRef(String),
    EnumRef { name: String, item: String },
    Value(T),
    Op(String, Box<Expr<T>>, Box<Expr<T>>),
    Unop(String, Box<Expr<T>>),
    Popcount(Box<Expr<T>>),
    SumOf(String),
}

#[derive(Debug, Clone)]
pub struct XidUnion {
    pub name: String,
    pub xidtypes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EnumItem {
    pub id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub items: Vec<EnumItem>,
    pub doc: Option<Doc>,
}

#[derive(Debug, Clone)]
pub struct SwitchCase {
    pub bit: bool,
    pub name: Option<String>,
    pub exprs: Vec<Expr<usize>>,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub enum StructField {
    Pad(String, usize),
    AlignPad(String, usize),
    Field {
        name: String,
        typ: String,
        enu: Option<String>,
    },
    List {
        name: String,
        typ: String,
        len_expr: Expr<usize>,
    },
    ListNoLen {
        name: String,
        typ: String,
    },
    Expr {
        name: String,
        typ: String,
        expr: Expr<usize>,
    },
    ValueParam {
        mask_typ: String,
        mask_name: String,
        list_name: String,
    },
    Fd(String),
    Switch(String, Expr<usize>, Vec<SwitchCase>),
    NamedCase(String, String),
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<StructField>,
    pub doc: Option<Doc>,
}

// a copy type for error or event
#[derive(Debug, Clone)]
pub struct OpCopy {
    pub number: i32,
    pub name: String,
}

pub type OpCopyMap = HashMap<String, Vec<OpCopy>>;

#[derive(Debug, Clone)]
pub struct Reply {
    pub fields: Vec<StructField>,
    pub doc: Option<Doc>,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub name: String,
    pub opcode: i32,
    pub params: Vec<StructField>,
    pub reply: Option<Reply>,
    pub doc: Option<Doc>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum Event {
    Info(String, Option<ExtInfo>),
    Error(i32, Struct),
    ErrorCopy {
        name: String,
        number: i32,
        ref_: String,
    },
    Import(String),
    Typedef {
        oldname: String,
        newname: String,
    },
    XidType(String),
    XidUnion(XidUnion),
    Enum(Enum),
    Struct(Struct),
    Union(Struct),
    Event {
        number: i32,
        stru: Struct,
        no_seq_number: bool,
        xge: bool,
    },
    EventCopy {
        name: String,
        number: i32,
        ref_: String,
    },
    Request(Request),
    Ignore,
}
