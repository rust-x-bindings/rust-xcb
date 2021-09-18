use quick_xml::events::attributes::{Attribute, Attributes};
use quick_xml::events::{BytesStart, Event as XmlEv};
use quick_xml::Reader as XmlReader;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::result;
use std::str::{self, FromStr, Utf8Error};

use crate::ast::{
    Doc, DocField, Enum, EnumItem, Event, Expr, ExtInfo, Reply, Request, Struct, StructField,
    SwitchCase, XidUnion,
};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Xml(quick_xml::Error),
    Utf8(Utf8Error),
    Parse(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        match err {
            quick_xml::Error::Io(e) => Error::IO(e),
            quick_xml::Error::Utf8(e) => Error::Utf8(e),
            e => Error::Xml(e),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub struct Parser<B: BufRead> {
    xml: XmlReader<B>,
    buf: Vec<u8>,
}

impl Parser<BufReader<File>> {
    pub fn from_file(xml_file: &Path) -> Self {
        let mut xml = XmlReader::from_file(xml_file).unwrap();
        xml.trim_text(true);

        Parser {
            xml,
            buf: Vec::with_capacity(8 * 1024),
        }
    }
}

impl<B: BufRead> Parser<B> {
    fn expect_text(&mut self) -> Result<String> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Text(e) | XmlEv::CData(e)) => {
                let txt = e.unescaped()?;
                Ok(str::from_utf8(&txt)?.into())
            }
            Ok(ev) => Err(Error::Parse(format!("expected text, found {:?}", ev))),
            Err(e) => Err(e.into()),
        }
    }

    fn expect_text_trim(&mut self, close_tag: &[u8]) -> Result<String> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Text(e) | XmlEv::CData(e)) => {
                let txt = e.unescaped()?;
                let txt = str::from_utf8(&txt)?.trim().into();
                if !close_tag.is_empty() {
                    match self.xml.read_event(&mut self.buf) {
                        Ok(XmlEv::End(e)) => {
                            if e.name() == close_tag {
                                return Ok(txt);
                            }
                            Err(Error::Parse(format!(
                                "expected </{}> after text",
                                str::from_utf8(close_tag).unwrap()
                            )))
                        }
                        Ok(ev) => Err(Error::Parse(format!(
                            "expected </{}>, found {:?}",
                            str::from_utf8(close_tag).unwrap(),
                            ev
                        ))),
                        Err(e) => Err(e.into()),
                    }
                } else {
                    Ok(txt)
                }
            }
            Ok(XmlEv::End(e)) => {
                if e.name() == close_tag {
                    Ok(String::new())
                } else {
                    Err(Error::Parse(format!(
                        "expected text, found </{}>",
                        str::from_utf8(e.name()).unwrap()
                    )))
                }
            }
            Ok(ev) => Err(Error::Parse(format!("expected text, found {:?}", ev))),
            Err(e) => Err(e.into()),
        }
    }

    fn expect_start(&mut self) -> Result<Vec<u8>> {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Start(e) | XmlEv::Empty(e)) => Ok(Vec::from(e.name())),
            Ok(ev) => Err(Error::Parse(format!("expected start tag, found {:?}", ev))),
            Err(e) => Err(e.into()),
        }
    }

    fn expect_close_tag(&mut self, tag: &[u8]) -> Result<()> {
        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::End(e)) => {
                    if e.name() == tag {
                        return Ok(());
                    } else {
                        return Err(Error::Parse(format!(
                            "expected </{}>, got </{}>",
                            str::from_utf8(tag).unwrap(),
                            str::from_utf8(e.name())?
                        )));
                    }
                }
                Ok(XmlEv::Comment(_)) => {}
                Ok(ev) => {
                    return Err(Error::Parse(format!(
                        "expected </{}>, found {:?}",
                        str::from_utf8(tag).unwrap(),
                        ev
                    )))
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn expect_text_element(&mut self) -> Result<(Vec<u8>, String)> {
        let tag = self.expect_start()?;
        let txt = self.expect_text()?;
        self.expect_close_tag(&tag)?;
        Ok((tag, txt))
    }

    // fn expect_start_tag(&mut self, tag: &[u8]) -> Result<()> {
    //     match self.xml.read_event(&mut self.buf) {
    //         Ok(XmlEv::Start(e) | XmlEv::Empty(e)) => {
    //             if e.name() == tag {
    //                 Ok(())
    //             } else {
    //                 Err(Error::Parse(format!(
    //                     "expected <{}>, got <{}>",
    //                     str::from_utf8(tag).unwrap(),
    //                     str::from_utf8(e.name())?
    //                 )))
    //             }
    //         }
    //         Ok(ev) => Err(Error::Parse(format!(
    //             "expected <{}>, found {:?}",
    //             str::from_utf8(tag).unwrap(),
    //             ev
    //         ))),
    //         Err(e) => Err(e)?,
    //     }
    // }

    // fn expect_text_element_with_tag(&mut self, tag: &[u8]) -> Result<String> {
    //     self.expect_start_tag(tag)?;
    //     let txt = self.expect_text()?;
    //     self.expect_close_tag(tag)?;
    //     Ok(txt)
    // }

    fn parse_doc(&mut self) -> Result<Doc> {
        let mut brief = String::new();
        let mut text = String::new();
        let mut fields = Vec::new();

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Start(ref e)) => match e.name() {
                    b"brief" => {
                        brief = self.expect_text_trim(b"brief")?;
                    }
                    b"description" => {
                        text = self.expect_text_trim(b"description")?;
                    }
                    b"field" => {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        let text = self.expect_text_trim(b"field")?;
                        fields.push(DocField { name, text });
                    }
                    b"error" => {
                        self.xml.read_to_end(b"error", &mut self.buf)?;
                    }
                    b"example" => {
                        self.xml.read_to_end(b"example", &mut self.buf)?;
                    }
                    b"see" => {
                        self.xml.read_to_end(b"see", &mut self.buf)?;
                    }
                    _ => {}
                },
                Ok(XmlEv::Empty(ref e)) => {
                    if e.name() == b"field" {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        fields.push(DocField {
                            name,
                            text: String::new(),
                        });
                    }
                }
                Ok(XmlEv::Text(_) | XmlEv::CData(_)) => {
                    return Err(Error::Parse("Unexpected doc text out of element".into()));
                }
                Ok(XmlEv::End(ref e)) => {
                    if e.name() == b"doc" {
                        break;
                    }
                }
                Ok(_) => {}
                Err(err) => return Err(err.into()),
            }
        }

        Ok(Doc {
            brief,
            text: text.trim().into(),
            fields,
        })
    }

    fn parse_expr_content<T>(&mut self, attrs: Attributes, tag: &[u8]) -> Result<Expr<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
        T: Clone,
    {
        match tag {
            b"fieldref" => {
                let fr = self.expect_text_trim(b"fieldref")?;
                Ok(Expr::FieldRef(fr))
            }
            b"paramref" => {
                let pr = self.expect_text_trim(b"paramref")?;
                Ok(Expr::ParamRef(pr))
            }
            b"enumref" => {
                let name = expect_attribute(attrs, b"ref")?;
                let item = self.expect_text_trim(b"enumref")?;
                Ok(Expr::EnumRef { name, item })
            }
            b"value" => {
                let val = self.expect_text_trim(b"value")?;
                let val: T = val.parse().map_err(|e| {
                    Error::Parse(format!("could not parse expr <value> tag: {}", e))
                })?;
                Ok(Expr::Value(val))
            }
            b"op" => {
                let op = expect_attribute(attrs, b"op")?;
                let lhs = self.parse_expr::<T>(b"")?.unwrap();
                let rhs = self.parse_expr::<T>(b"")?.unwrap();
                self.expect_close_tag(b"op")?;
                Ok(Expr::Op(op, Box::new(lhs), Box::new(rhs)))
            }
            b"unop" => {
                let unop = expect_attribute(attrs, b"op")?;
                let expr = self.parse_expr::<T>(b"")?.unwrap();
                self.expect_close_tag(b"unop")?;
                Ok(Expr::Unop(unop, Box::new(expr)))
            }
            b"popcount" => {
                let expr = self.parse_expr::<T>(b"")?.unwrap();
                self.expect_close_tag(b"popcount")?;
                Ok(Expr::Popcount(Box::new(expr)))
            }
            b"sumof" => {
                let list_ref = expect_attribute(attrs, b"ref")?;
                Ok(Expr::SumOf(list_ref))
            }
            tag => Err(Error::Parse(format!(
                "Unexpected <{}> in expression",
                str::from_utf8(tag)?
            ))),
        }
    }

    fn parse_expr<T>(&mut self, empty_end_tag: &[u8]) -> Result<Option<Expr<T>>>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
        T: Clone,
    {
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Start(ref e) | XmlEv::Empty(ref e)) => {
                let e = e.to_owned();
                Ok(Some(self.parse_expr_content(e.attributes(), e.name())?))
            }
            Ok(XmlEv::Comment(_)) => self.parse_expr::<T>(empty_end_tag), // in case of comment, we just parse the next one
            Ok(XmlEv::End(e)) => {
                if e.name() == empty_end_tag {
                    Ok(None)
                } else {
                    Err(Error::Parse(format!(
                        "Unexpected </{}> while parsing expression",
                        str::from_utf8(e.name()).unwrap()
                    )))
                }
            }
            Ok(ev) => Err(Error::Parse(format!(
                "Unexpected XML while parsing expression: {:?}",
                ev
            ))),
            Err(err) => Err(err.into()),
        }
    }

    fn parse_import(&mut self) -> Result<String> {
        let imp = self.expect_text()?;
        self.expect_close_tag(b"import")?;
        Ok(imp)
    }

    fn parse_xidunion(&mut self, name: String) -> Result<XidUnion> {
        let mut xidtypes = Vec::new();

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Start(ref e)) => match e.name() {
                    b"type" => {
                        let typ = self.expect_text_trim(b"type")?;
                        xidtypes.push(typ);
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected <{}> in <xidunion>",
                            str::from_utf8(tag)?
                        )));
                    }
                },
                Ok(XmlEv::End(ref e)) => match e.name() {
                    b"xidunion" => break,
                    _ => unreachable!(),
                },
                Ok(ev) => {
                    return Err(Error::Parse(format!(
                        "Unexpected XML in <xidunion>: {:?}",
                        ev
                    )));
                }
                Err(err) => {
                    return Err(err.into());
                }
            }
        }

        Ok(XidUnion { name, xidtypes })
    }

    fn parse_enum(&mut self, name: String) -> Result<Enum> {
        let mut items = Vec::new();
        let mut doc = None;

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Empty(ref e) | XmlEv::Start(ref e)) => match e.name() {
                    b"item" => {
                        let name = expect_attribute(e.attributes(), b"name")?;
                        let (tag, value) = self.expect_text_element()?;
                        if tag != b"bit" && tag != b"value" {
                            return Err(Error::Parse(format!(
                                "expected <bit> or <value> for enum {}, got <{}>",
                                name,
                                str::from_utf8(&tag)?
                            )));
                        }
                        let value: u32 = value.parse().map_err(|e| {
                            Error::Parse(format!(
                                "failed to parse {} of enum {}: {}",
                                str::from_utf8(&tag).unwrap(),
                                name,
                                e
                            ))
                        })?;
                        let is_bit = tag == b"bit";
                        let value = if is_bit { 1 << value } else { value };
                        items.push(EnumItem {
                            id: name.clone(),
                            name,
                            value,
                        });
                    }
                    b"doc" => {
                        doc = Some(self.parse_doc()?);
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected tag in enum: {}",
                            str::from_utf8(tag)?
                        )));
                    }
                },
                Ok(XmlEv::End(ref e)) => match e.name() {
                    b"enum" => break,
                    b"item" => continue,
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected </{}> in enum {}",
                            str::from_utf8(tag)?,
                            name,
                        )))
                    }
                },
                Ok(XmlEv::Comment(_)) => {}
                Ok(ev) => {
                    return Err(Error::Parse(format!("unexpected XML in enum: {:?}", ev)));
                }
                Err(err) => return Err(err.into()),
            }
        }

        Ok(Enum { name, items, doc })
    }

    fn parse_field_content(
        &mut self,
        e: &BytesStart,
        mut padnum: &mut u32,
        empty_tag: bool,
    ) -> Result<StructField> {
        if empty_tag {
            match e.name() {
                b"field" => {
                    let names: [&[u8]; 3] = [b"type", b"name", b"enum"];
                    let mut vals: [Option<String>; 3] = [None, None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [typ, nam, enu] = vals;

                    if let (Some(typ), Some(name)) = (typ, nam) {
                        Ok(StructField::Field { name, typ, enu })
                    } else {
                        Err(Error::Parse("struct field without type and/or name".into()))
                    }
                }
                b"pad" => {
                    let names: [&[u8]; 2] = [b"bytes", b"align"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [bytes, align] = vals;
                    if bytes.is_some() && align.is_some() {
                        return Err(Error::Parse("<pad> with both align and bytes attr".into()));
                    }
                    if let Some(bytes) = bytes {
                        let val: usize = bytes.parse().map_err(|e| {
                            Error::Parse(format!("failed to parse pad bytes of struct: {}", e))
                        })?;
                        let name = format!("pad{}", padnum);
                        (*padnum) += 1;
                        Ok(StructField::Pad(name, val))
                    } else if let Some(align) = align {
                        let val: usize = align.parse().map_err(|e| {
                            Error::Parse(format!("failed to parse pad bytes of struct: {}", e))
                        })?;
                        let name = format!("pad{}", padnum);
                        (*padnum) += 1;
                        Ok(StructField::AlignPad(name, val))
                    } else {
                        Err(Error::Parse(
                            "<pad> with neither align and bytes attr".into(),
                        ))
                    }
                }
                b"list" => {
                    let names: [&[u8]; 2] = [b"type", b"name"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [typ, nam] = vals;
                    if let (Some(typ), Some(name)) = (typ, nam) {
                        Ok(StructField::ListNoLen { name, typ })
                    } else {
                        Err(Error::Parse(
                            "<list> tag without type and/or name attribute".into(),
                        ))
                    }
                }
                b"valueparam" => {
                    let names: [&[u8]; 3] =
                        [b"value-mask-type", b"value-mask-name", b"value-list-name"];
                    let mut vals: [Option<String>; 3] = [None, None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [mask_typ, mask_name, list_name] = vals;
                    if let (Some(mask_typ), Some(mask_name), Some(list_name)) =
                        (mask_typ, mask_name, list_name)
                    {
                        Ok(StructField::ValueParam {
                            mask_typ,
                            mask_name,
                            list_name,
                        })
                    } else {
                        Err(Error::Parse(
                                "<valueparam> tag without value-mask-type, value-mask-name or value-list-name attribute".into(),
                            ))
                    }
                }
                b"fd" => {
                    let name = expect_attribute(e.attributes(), b"name")?;
                    Ok(StructField::Fd(name))
                }
                tag => {
                    return Err(Error::Parse(format!(
                        "Unexpected <{} /> in field",
                        str::from_utf8(tag)?
                    )))
                }
            }
        } else {
            match e.name() {
                b"list" => {
                    let names: [&[u8]; 2] = [b"type", b"name"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [typ, nam] = vals;
                    if let (Some(typ), Some(name)) = (typ, nam) {
                        let len_expr = self.parse_expr::<usize>(b"list")?;
                        Ok(match len_expr {
                            Some(len_expr) => StructField::List {
                                name,
                                typ,
                                len_expr,
                            },
                            None => StructField::ListNoLen { name, typ },
                        })
                    } else {
                        Err(Error::Parse(
                            "<list> tag without type and/or name attribute".into(),
                        ))
                    }
                }
                b"exprfield" => {
                    let names: [&[u8]; 2] = [b"type", b"name"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [typ, nam] = vals;
                    if let (Some(typ), Some(name)) = (typ, nam) {
                        let expr = self.parse_expr::<usize>(b"")?.unwrap();
                        self.xml.read_to_end(b"exprfield", &mut self.buf)?;
                        Ok(StructField::Expr { name, typ, expr })
                    } else {
                        Err(Error::Parse(
                            "<exprfield> tag without type and/or name attribute".into(),
                        ))
                    }
                }
                b"switch" => {
                    let name = expect_attribute(e.attributes(), b"name")?;
                    let (expr, cases) = self.parse_switch(&mut padnum)?;
                    Ok(StructField::Switch(name, expr, cases))
                }
                tag => {
                    return Err(Error::Parse(format!(
                        "Unexpected <{}> in field",
                        str::from_utf8(tag)?
                    )))
                }
            }
        }
    }

    fn parse_struct_content(&mut self, end_tag: &[u8]) -> Result<StructContent> {
        let mut fields = Vec::new();
        let mut reply = None;
        let mut doc = None;
        let mut padnum = 0;

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Empty(ref e)) => {
                    let e = e.to_owned();
                    fields.push(self.parse_field_content(&e, &mut padnum, true)?);
                }
                Ok(XmlEv::Start(ref e)) => match e.name() {
                    b"list" | b"exprfield" | b"switch" => {
                        let e = e.to_owned();
                        fields.push(self.parse_field_content(&e, &mut padnum, false)?);
                    }
                    b"doc" => {
                        doc = Some(self.parse_doc()?);
                    }
                    b"reply" => {
                        let StructContent { fields, doc, .. } =
                            self.parse_struct_content(b"reply")?;
                        reply = Some(Reply { fields, doc })
                    }
                    tag => {
                        return Err(Error::Parse(format!(
                            "Unexpected <{} /> in struct",
                            str::from_utf8(tag)?
                        )))
                    }
                },
                Ok(XmlEv::End(ref e)) => {
                    if e.name() == end_tag {
                        break;
                    }
                }
                Ok(XmlEv::Comment(_)) => {}
                Ok(ev) => {
                    return Err(Error::Parse(format!("unexpected XML in struct: {:?}", ev)));
                }
                Err(err) => return Err(err.into()),
            }
        }

        Ok(StructContent { fields, reply, doc })
    }

    fn parse_struct(&mut self, name: String, end_tag: &[u8]) -> Result<Struct> {
        let StructContent { fields, doc, .. } = self.parse_struct_content(end_tag)?;

        Ok(Struct { name, fields, doc })
    }

    fn parse_op_struct(&mut self, start: BytesStart, end_tag: &[u8]) -> Result<OpStruct> {
        let names: [&[u8]; 5] = [b"name", b"number", b"ref", b"no-sequence-number", b"xge"];
        let mut vals: [Option<String>; 5] = [None, None, None, None, None];
        get_attributes(start.attributes(), &names, &mut vals)?;
        let [name, number, ref_, no_seq_number, xge] = vals;
        // FIXME: check if true or false
        let no_seq_number = no_seq_number.is_some();
        let xge = xge.is_some();
        match (name, number) {
            (Some(name), Some(number)) => {
                let number = str::parse::<i32>(&number)
                    .map_err(|_| Error::Parse(format!("can't parse {} as number", number)))?;
                let stru = if end_tag.is_empty() {
                    Struct {
                        name,
                        fields: Vec::new(),
                        doc: None,
                    }
                } else {
                    self.parse_struct(name, end_tag)?
                };
                Ok(OpStruct {
                    number,
                    stru,
                    ref_,
                    no_seq_number,
                    xge,
                })
            }
            _ => Err(Error::Parse(format!(
                "<{}> without name or number",
                str::from_utf8(start.name())?
            ))),
        }
    }

    fn parse_request(&mut self, start: BytesStart, is_empty: bool) -> Result<Request> {
        let names: [&[u8]; 2] = [b"name", b"opcode"];
        let mut vals: [Option<String>; 2] = [None, None];
        get_attributes(start.attributes(), &names, &mut vals)?;
        let [name, opcode] = vals;
        if name.is_none() && opcode.is_none() {
            return Err(Error::Parse(
                "<request> without name or opcode attributes".into(),
            ));
        }
        let name = name.unwrap();
        let opcode = opcode.unwrap();
        let opcode: i32 = str::parse(&opcode)
            .map_err(|_| Error::Parse(format!("cannot parse {} as int", &opcode)))?;
        if is_empty {
            Ok(Request {
                name,
                opcode,
                params: Vec::new(),
                doc: None,
                reply: None,
            })
        } else {
            let StructContent { fields, doc, reply } = self.parse_struct_content(b"request")?;
            Ok(Request {
                name,
                opcode,
                params: fields,
                doc,
                reply,
            })
        }
    }

    fn parse_switch_case(
        &mut self,
        name: Option<String>,
        mut padnum: &mut u32,
        end_tag: &[u8],
    ) -> Result<SwitchCase> {
        let bit = end_tag == b"bitcase";

        let mut exprs = Vec::new();
        let mut fields = Vec::new();

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Start(ref e)) => {
                    let e = e.to_owned();
                    if is_expr_tag(e.name()) {
                        let expr = self.parse_expr_content(e.attributes(), e.name())?;
                        exprs.push(expr);
                    } else {
                        let field = self.parse_field_content(&e, &mut padnum, false)?;
                        fields.push(field);
                    }
                }
                Ok(XmlEv::Empty(ref e)) => {
                    let e = e.to_owned();
                    if is_expr_tag(e.name()) {
                        let expr = self.parse_expr_content(e.attributes(), e.name())?;
                        exprs.push(expr);
                    } else {
                        let field = self.parse_field_content(&e, &mut padnum, true)?;
                        fields.push(field);
                    }
                }
                Ok(XmlEv::Comment(_)) => {}
                Ok(XmlEv::End(ref e)) => {
                    if e.name() == end_tag {
                        break;
                    }
                }
                ev => {
                    return Err(Error::Parse(format!(
                        "unexpected XML in switch case: {:?}",
                        ev
                    )))
                }
            }
        }

        Ok(SwitchCase {
            bit,
            name,
            exprs,
            fields,
        })
    }

    fn parse_switch(&mut self, padnum: &mut u32) -> Result<(Expr<usize>, Vec<SwitchCase>)> {
        let expr = self.parse_expr(b"")?.unwrap();

        let mut cases = Vec::new();

        loop {
            match self.xml.read_event(&mut self.buf) {
                Ok(XmlEv::Start(ref e)) => {
                    let names: [&[u8]; 1] = [b"name"];
                    let mut vals: [Option<String>; 1] = [None];
                    get_attributes(e.attributes(), &names, &mut vals)?;
                    let [name] = vals;
                    match e.name() {
                        b"bitcase" => {
                            cases.push(self.parse_switch_case(name, padnum, b"bitcase")?);
                        }
                        b"case" => {
                            cases.push(self.parse_switch_case(name, padnum, b"case")?);
                        }
                        name => {
                            return Err(Error::Parse(format!(
                                "unexpected <{}> in <switch>",
                                str::from_utf8(name).unwrap()
                            )));
                        }
                    }
                }
                Ok(XmlEv::End(ref e)) => {
                    if e.name() == b"switch" {
                        break;
                    }
                }
                Ok(_) => {}
                Err(err) => return Err(err.into()),
            }
        }

        Ok((expr, cases))
    }
}

struct OpStruct {
    number: i32,
    stru: Struct,
    ref_: Option<String>,
    no_seq_number: bool,
    xge: bool,
}

struct StructContent {
    fields: Vec<StructField>,
    reply: Option<Reply>,
    doc: Option<Doc>,
}

fn is_expr_tag(tag: &[u8]) -> bool {
    matches!(
        tag,
        b"fieldref"
            | b"paramref"
            | b"op"
            | b"unop"
            | b"popcount"
            | b"value"
            | b"enumref"
            | b"sumof"
    )
}

impl<B: BufRead> Iterator for &mut Parser<B> {
    type Item = Result<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();
        match self.xml.read_event(&mut self.buf) {
            Ok(XmlEv::Empty(ref e)) => match e.name() {
                b"typedef" => {
                    let names: [&[u8]; 2] = [b"oldname", b"newname"];
                    let mut vals: [Option<String>; 2] = [None, None];
                    if let Err(err) = get_attributes(e.attributes(), &names, &mut vals) {
                        return Some(Err(err));
                    }
                    let [oldname, newname] = vals;
                    match (oldname, newname) {
                        (Some(oldname), Some(newname)) => {
                            Some(Ok(Event::Typedef { oldname, newname }))
                        }
                        _ => Some(Err(Error::Parse(
                            "typedef without newname and/or oldname".into(),
                        ))),
                    }
                }
                b"xidtype" => Some(expect_attribute(e.attributes(), b"name").map(Event::XidType)),
                b"error" => Some({
                    let start = e.to_owned();
                    self.parse_op_struct(start, b"")
                        .map(|ns| Event::Error(ns.number, ns.stru))
                }),
                b"errorcopy" => Some({
                    let start = e.to_owned();
                    let nsres = self.parse_op_struct(start, b"");
                    if let Ok(ns) = nsres {
                        if ns.ref_.is_none() {
                            return Some(Err(Error::Parse("".into())));
                        }
                        let number = ns.number;
                        let ref_ = ns.ref_.unwrap();

                        Ok(Event::ErrorCopy {
                            name: ns.stru.name,
                            number,
                            ref_,
                        })
                    } else {
                        Err(Error::Parse("<errorcopy> without ref attribute".into()))
                    }
                }),
                b"eventcopy" => Some({
                    let start = e.to_owned();
                    let nsres = self.parse_op_struct(start, b"");
                    if let Ok(ns) = nsres {
                        if ns.ref_.is_none() {
                            return Some(Err(Error::Parse("".into())));
                        }
                        let number = ns.number;
                        let ref_ = ns.ref_.unwrap();

                        Ok(Event::EventCopy {
                            name: ns.stru.name,
                            number,
                            ref_,
                        })
                    } else {
                        Err(Error::Parse("<errorcopy> without ref attribute".into()))
                    }
                }),
                b"request" => {
                    let start = e.to_owned();
                    Some(self.parse_request(start, true).map(Event::Request))
                }
                _ => Some(Ok(Event::Ignore)),
            },
            Ok(XmlEv::Start(ref e)) => match e.name() {
                b"xcb" => {
                    let names: [&[u8]; 5] = [
                        b"header",
                        b"extension-xname",
                        b"extension-name",
                        b"major-version",
                        b"minor-version",
                    ];
                    let mut vals: [Option<String>; 5] = [None, None, None, None, None];
                    if let Err(err) = get_attributes(e.attributes(), &names, &mut vals) {
                        return Some(Err(err));
                    }
                    let [mod_name, xname, name, major_version, minor_version] = vals;

                    if mod_name.is_none() {
                        return Some(Err(Error::Parse("<xcb> without header attr".into())));
                    }
                    let mod_name = mod_name.unwrap();

                    let ext_info = match (xname, name, major_version, minor_version) {
                        (Some(xname), Some(name), Some(major_version), Some(minor_version)) => {
                            let major_version = major_version
                                .parse::<u32>()
                                .expect("could not parse major_version");
                            let minor_version = minor_version
                                .parse::<u32>()
                                .expect("could not parse major_version");
                            Some(ExtInfo {
                                xname,
                                name,
                                major_version,
                                minor_version,
                            })
                        }
                        (None, None, None, None) => None,
                        _ => panic!("incomplete extension info for {}", &mod_name),
                    };

                    Some(Ok(Event::Info(mod_name, ext_info)))
                }
                b"import" => Some(self.parse_import().map(Event::Import)),
                b"xidunion" => Some(expect_attribute(e.attributes(), b"name").and_then(|name| {
                    let unionres = self.parse_xidunion(name);
                    unionres.map(Event::XidUnion)
                })),
                b"enum" => Some(expect_attribute(e.attributes(), b"name").and_then(|name| {
                    let enumres = self.parse_enum(name);
                    enumres.map(Event::Enum)
                })),
                b"struct" => Some(expect_attribute(e.attributes(), b"name").and_then(|name| {
                    let structres = self.parse_struct(name, b"struct");
                    structres.map(Event::Struct)
                })),
                b"union" => Some(expect_attribute(e.attributes(), b"name").and_then(|name| {
                    let unionres = self.parse_struct(name, b"union");
                    unionres.map(Event::Union)
                })),
                b"error" => Some({
                    let start = e.to_owned();
                    self.parse_op_struct(start, b"error")
                        .map(|ns| Event::Error(ns.number, ns.stru))
                }),
                b"event" => Some({
                    let start = e.to_owned();
                    self.parse_op_struct(start, b"event").map(
                        |OpStruct {
                             number,
                             stru,
                             no_seq_number,
                             xge,
                             ..
                         }| Event::Event {
                            number,
                            stru,
                            no_seq_number,
                            xge,
                        },
                    )
                }),
                b"request" => {
                    let start = e.to_owned();
                    Some(self.parse_request(start, false).map(Event::Request))
                }
                _ => Some(Ok(Event::Ignore)),
            },

            Ok(XmlEv::Eof) => None,

            _ => Some(Ok(Event::Ignore)),
        }
    }
}

fn attr_value(attr: &Attribute) -> Result<String> {
    let val = attr.unescaped_value()?;
    Ok(str::from_utf8(&val)?.into())
}

fn expect_attribute(attrs: Attributes, name: &[u8]) -> Result<String> {
    for attr in attrs {
        match attr {
            Ok(attr) => {
                if attr.key == name {
                    return attr_value(&attr);
                }
            }
            Err(err) => return Err(err.into()),
        }
    }
    Err(Error::Parse(format!(
        "could not find expected attribute: {}",
        str::from_utf8(name).unwrap()
    )))
}

fn get_attributes(attrs: Attributes, names: &[&[u8]], output: &mut [Option<String>]) -> Result<()> {
    assert_eq!(names.len(), output.len());
    for attr in attrs {
        match attr {
            Ok(attr) => {
                for (i, nam) in names.iter().enumerate() {
                    if attr.key == *nam {
                        output[i] = Some(attr_value(&attr)?);
                    }
                }
            }
            Err(err) => return Err(err.into()),
        }
    }
    Ok(())
}
