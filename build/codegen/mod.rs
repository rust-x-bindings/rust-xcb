use std::collections::{HashMap, HashSet};
use std::env;
use std::io::{self, Cursor, Write};

use crate::ast::{Doc, EnumItem, Event, Expr, ExtInfo, OpCopyMap, Request, Struct, StructField};
use crate::output::Output;

mod ffi;
mod rs;

#[derive(Clone, Debug)]
pub struct DepInfo {
    pub xcb_mod: String,
    pub xcb_mod_prefix: String,
    pub imports: Vec<String>,
    pub typ_with_lifetime: HashSet<String>,
    pub typ_unions: HashSet<String>,
    pub typ_simple: HashSet<String>,
    pub typ_pod: HashSet<String>,
    pub ffi_type_sizes: HashMap<String, Option<usize>>,
    pub ffi_typ_reg: HashSet<String>,
    pub rs_typ_reg: HashSet<String>,
}

impl DepInfo {
    fn has_type(&self, typ: &str) -> bool {
        self.ffi_type_sizes.contains_key(typ)
    }
}

#[derive(Debug)]
pub struct CodeGen {
    xcb_mod: String,
    xcb_mod_prefix: String,

    // output files
    ffi: Output,
    rs: Output,

    // additional output buffers that make a second group of declaration/functions
    // they are appended to the output at the end
    ffi_buf: Cursor<Vec<u8>>,
    rs_buf: Cursor<Vec<u8>>,

    typ_with_lifetime: HashSet<String>,
    typ_unions: HashSet<String>,
    typ_simple: HashSet<String>, // integer, chars, xids, enums
    typ_pod: HashSet<String>,    // simple and plain old data

    imports: Vec<String>,
    // registered types sizes (is None if size is not fixed - eg. lists with dynamic length)
    ffi_type_sizes: HashMap<String, Option<usize>>,
    // types registered in the FFI module
    ffi_typ_reg: HashSet<String>,
    // types registered in the Rust module
    rs_typ_reg: HashSet<String>,

    // keep track of struct declarations to handle typealiases refering to a struct
    // we could simply issue a `type new = old;`, but for now we want to be diff compatible
    // with python generated bindings which rewrites a complete new struct.
    structs: Vec<Struct>,

    dep_info: Vec<DepInfo>,

    evcopies: OpCopyMap,

    ptr_width: usize,
}

#[cfg(target_pointer_width = "64")]
const PTR_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
const PTR_WIDTH: usize = 32;

impl CodeGen {
    pub fn new(
        xcb_mod: &str,
        ffi: Output,
        rs: Output,
        dep_info: Vec<DepInfo>,
        evcopies: OpCopyMap,
    ) -> CodeGen {
        let mp = if xcb_mod == "xproto" {
            String::new()
        } else {
            format!("{}_", &xcb_mod)
        };

        let ptr_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH")
            .map(|s| {
                str::parse::<usize>(&s).expect(&format!(
                    "can't parse CARGO_CFG_TARGET_POINTER_WIDTH {} as usize",
                    s
                ))
            })
            .unwrap_or(PTR_WIDTH);

        CodeGen {
            xcb_mod: xcb_mod.to_owned(),
            xcb_mod_prefix: mp,
            ffi,
            rs,
            ffi_buf: Cursor::new(Vec::new()),
            rs_buf: Cursor::new(Vec::new()),
            imports: Vec::new(),
            typ_with_lifetime: HashSet::new(),
            typ_unions: HashSet::new(),
            typ_simple: HashSet::new(),
            typ_pod: HashSet::new(),
            ffi_type_sizes: HashMap::new(),
            ffi_typ_reg: HashSet::new(),
            rs_typ_reg: HashSet::new(),
            structs: Vec::new(),
            dep_info,
            evcopies,
            ptr_width,
        }
    }

    pub fn into_depinfo(self) -> DepInfo {
        DepInfo {
            xcb_mod: self.xcb_mod,
            xcb_mod_prefix: self.xcb_mod_prefix,
            imports: self.imports,
            typ_with_lifetime: self.typ_with_lifetime,
            typ_unions: self.typ_unions,
            typ_simple: self.typ_simple,
            typ_pod: self.typ_pod,
            ffi_type_sizes: self.ffi_type_sizes,
            ffi_typ_reg: self.ffi_typ_reg,
            rs_typ_reg: self.rs_typ_reg,
        }
    }

    pub fn prologue(&mut self, imports: Vec<String>, ext_info: &Option<ExtInfo>) -> io::Result<()> {
        self.imports = imports;
        let mut imports = HashSet::<String>::new();
        for imp in self.imports.iter() {
            imports.insert(imp.clone());
        }
        for di in &self.dep_info {
            for imp in &di.imports {
                imports.insert(imp.clone());
            }
        }

        let out = &mut self.ffi;
        // Adding a comment only to fit the python generated code and pass initial tests
        writeln!(
            out,
            "// Generated automatically from {}.xml by rs_client.py version 0.9.0.",
            &self.xcb_mod
        )?;
        writeln!(out, "// Do not edit!")?;
        writeln!(out)?;
        writeln!(out, "use ffi::base::*;")?;
        for imp in imports.iter() {
            writeln!(out, "use ffi::{}::*;", imp)?;
        }
        writeln!(out, "use libc::{{c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;

        if let Some(ext_info) = ext_info {
            let maj_name = ffi::opcode_name(&self.xcb_mod_prefix, "MajorVersion");
            let min_name = ffi::opcode_name(&self.xcb_mod_prefix, "MinorVersion");

            writeln!(out)?;
            writeln!(
                out,
                "pub const {}: u32 = {};",
                &maj_name, ext_info.major_version
            )?;
            writeln!(
                out,
                "pub const {}: u32 = {};",
                &min_name, ext_info.minor_version
            )?;

            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub static mut xcb_{}_id: xcb_extension_t;",
                &self.xcb_mod
            )?;
        }

        let out = &mut self.rs;
        writeln!(
            out,
            "// Generated automatically from {}.xml by rs_client.py version 0.9.0.",
            &self.xcb_mod
        )?;
        writeln!(out, "// Do not edit!")?;
        writeln!(out, "")?;
        writeln!(out, "use base;")?;
        for imp in imports.iter() {
            writeln!(out, "use {};", imp)?;
        }
        writeln!(out, "use ffi::base::*;")?;
        writeln!(out, "use ffi::{}::*;", self.xcb_mod)?;
        for imp in imports.iter() {
            writeln!(out, "use ffi::{}::*;", imp)?;
        }
        writeln!(out, "use libc::{{self, c_char, c_int, c_uint, c_void}};")?;
        writeln!(out, "use std;")?;
        writeln!(out, "use std::iter::Iterator;")?;
        writeln!(out, "")?;

        if let Some(ext_info) = ext_info {
            let out = &mut self.rs;
            writeln!(out)?;
            writeln!(out, "pub fn id() -> &'static mut base::Extension {{")?;
            writeln!(out, "    unsafe {{")?;
            writeln!(out, "        &mut xcb_{}_id", &self.xcb_mod)?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
            writeln!(out)?;
            writeln!(
                out,
                "pub const MAJOR_VERSION: u32 = {};",
                ext_info.major_version
            )?;
            writeln!(
                out,
                "pub const MINOR_VERSION: u32 = {};",
                ext_info.minor_version
            )?;
        }

        Ok(())
    }

    pub fn epilogue(&mut self) -> io::Result<()> {
        let linklib = match self.xcb_mod.as_str() {
            "xproto" | "big_requests" | "xc_misc" => "xcb".to_owned(),
            "genericevent" => "xcb-ge".to_owned(),
            "x_print" => "xcb-xprint".to_owned(),
            "test" => "xcb-xtest".to_owned(),
            "selinux" => "xcb-xselinux".to_owned(),
            m => {
                let mut l = "xcb-".to_owned();
                l.push_str(m);
                l
            }
        };

        let out = &mut self.ffi;
        // write out all the external functions
        writeln!(out)?;
        writeln!(out, "#[link(name = \"{}\")]", linklib)?;
        writeln!(out, "extern {{")?;

        out.write_all(self.ffi_buf.get_ref())?;

        writeln!(out)?;
        writeln!(out, "}} // extern")?;

        let out = &mut self.rs;
        out.write_all(self.rs_buf.get_ref())?;
        Ok(())
    }

    pub fn event(&mut self, ev: Event) -> io::Result<()> {
        match ev {
            Event::Typedef { oldname, newname } => {
                let stru = self.structs.iter().find(|stru| stru.name == oldname);
                if let Some(stru) = stru {
                    // regenerating struct
                    let newstruct = Struct {
                        name: newname,
                        fields: stru.fields.clone(),
                        doc: stru.doc.clone(),
                    };
                    self.emit_struct(newstruct)?;
                } else {
                    // classic typedef with `type new = old;`
                    self.notify_typ(newname.clone());

                    let ffi_old_typ = self.ffi_use_type_name(&oldname);
                    let ffi_new_typ = self.ffi_decl_type_name(&newname);

                    emit_type_alias(&mut self.ffi, &ffi_new_typ, &ffi_old_typ)?;
                    self.emit_ffi_iterator(&newname, &ffi_new_typ, false)?;

                    let rs_new_typ = rs::type_name(&newname);
                    emit_type_alias(&mut self.rs, &rs_new_typ, &ffi_new_typ)?;

                    let is_simple = self.typ_is_simple(&oldname);
                    let is_pod = self.typ_is_pod(&oldname);

                    self.reg_type(
                        newname,
                        ffi_new_typ,
                        rs_new_typ,
                        self.ffi_type_sizeof(&oldname),
                        false,
                        is_simple,
                        is_pod,
                    )
                }
            }
            Event::XidType(name) => self.emit_xid(name)?,
            Event::XidUnion(xidun) => self.emit_xid(xidun.name)?,
            Event::Enum(en) => {
                // make owned string to pass into the closure
                // otherwise borrow checker complains
                let xcb_mod_prefix = self.xcb_mod_prefix.to_string();

                let ffi_typ = self.ffi_enum_type_name(&en.name);
                emit_enum(
                    &mut self.ffi,
                    &ffi_typ,
                    en.items.iter().map(|it| EnumItem {
                        id: it.id.clone(),
                        name: ffi::enum_item_name(&xcb_mod_prefix, &en.name, &it.name),
                        value: it.value,
                    }),
                    &en.doc,
                )?;

                let rs_typ = self.rs_enum_type_name(&en.name);
                emit_enum(
                    &mut self.rs,
                    &rs_typ,
                    en.items.iter().map(|it| EnumItem {
                        id: it.id.clone(),
                        name: rs::enum_item_name(&en.name, &it.name),
                        value: it.value,
                    }),
                    &en.doc,
                )?;
                self.reg_type(en.name, ffi_typ, rs_typ, Some(4), false, true, true);
            }
            Event::Struct(stru) => self.emit_struct(stru)?,
            Event::Union(stru) => self.emit_union(stru)?,
            Event::Error(number, stru) => self.emit_error(number, stru)?,
            Event::ErrorCopy { name, number, ref_ } => {
                self.emit_error_copy(&name, number, &ref_)?
            }
            Event::Event {
                number,
                stru,
                no_seq_number,
                xge,
                ..
            } => self.emit_event(number, stru, no_seq_number, xge)?,
            Event::Request(req) => self.emit_request(req)?,
            _ => {}
        }
        Ok(())
    }

    // pub fn xcb_mod(&self) -> &str {
    //     &self.xcb_mod
    // }

    fn reg_type(
        &mut self,
        typ: String,
        ffi_typ: String,
        rs_typ: String,
        ffi_sz: Option<usize>,
        is_union: bool,
        is_simple: bool,
        is_pod: bool,
    ) {
        self.ffi_typ_reg.insert(ffi_typ);
        self.rs_typ_reg.insert(rs_typ);
        if is_union {
            self.typ_unions.insert(typ.clone());
        }
        if is_simple {
            self.typ_simple.insert(typ.clone());
        }
        if is_pod {
            self.typ_pod.insert(typ.clone());
        }
        self.ffi_type_sizes.insert(typ, ffi_sz);
    }

    /// notifies that a XCB type was declared in this module
    /// alternative to reg_type, with less info
    /// this is useful for replies, cookies, etc. and other derived types
    fn notify_typ(&mut self, typ: String) {
        self.ffi_type_sizes.insert(typ, None);
    }

    fn has_type(&self, typ: &str) -> bool {
        self.ffi_type_sizes.contains_key(typ)
    }

    fn typ_is_simple(&self, typ: &str) -> bool {
        match typ {
            "CARD8" => true,
            "CARD16" => true,
            "CARD32" => true,
            "CARD64" => true,
            "INT8" => true,
            "INT16" => true,
            "INT32" => true,
            "BYTE" => true,
            "BOOL" => true,
            "char" => true,
            "float" => true,
            "double" => true,
            "void" => true,
            typ => {
                let (_, typ) = extract_module(&typ);
                self.typ_simple.contains(typ)
                    || self.dep_info.iter().any(|di| di.typ_simple.contains(typ))
            }
        }
    }

    fn typ_is_pod(&self, typ: &str) -> bool {
        let (_, typ) = extract_module(&typ);
        self.typ_is_simple(&typ)
            || self.typ_pod.contains(typ)
            || self.dep_info.iter().any(|di| di.typ_pod.contains(typ))
    }

    fn fields_are_pod(&self, fields: &[StructField]) -> bool {
        for f in fields.iter() {
            match f {
                StructField::Field { typ, .. } => {
                    if !self.typ_is_pod(&typ) {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        true
    }

    fn fields_need_lifetime(&self, fields: &[StructField]) -> bool {
        for f in fields.iter() {
            match f {
                StructField::Field { typ, .. } => {
                    if self.typ_unions.contains(typ) {
                        return true;
                    }
                    if self.type_has_lifetime(typ) {
                        return true;
                    }
                }
                StructField::List { .. } => {
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    fn type_has_lifetime(&self, typ: &str) -> bool {
        if self.typ_with_lifetime.contains(typ) {
            true
        } else {
            for di in &self.dep_info {
                if di.typ_with_lifetime.contains(typ) {
                    return true;
                }
            }
            false
        }
    }

    fn eligible_to_copy(&self, stru: &Struct) -> bool {
        for f in stru.fields.iter() {
            match f {
                StructField::List { len_expr, .. } => {
                    if expr_fixed_length(len_expr).is_none() {
                        return false;
                    }
                }
                StructField::ValueParam { .. } => {
                    return false;
                }
                StructField::Switch(..) => {
                    return false;
                }
                StructField::ListNoLen { .. } => {
                    return false;
                }
                _ => {}
            }
        }
        true
    }

    fn emit_xid(&mut self, name: String) -> io::Result<()> {
        self.notify_typ(name.clone());
        let ffi_typ = self.ffi_decl_type_name(&name);
        emit_type_alias(&mut self.ffi, &ffi_typ, "u32")?;
        self.emit_ffi_iterator(&name, &ffi_typ, false)?;

        let rs_typ = rs::type_name(&name);
        emit_type_alias(&mut self.rs, &rs_typ, &ffi_typ)?;

        self.reg_type(name, ffi_typ, rs_typ, Some(4), false, true, true);
        Ok(())
    }

    fn emit_struct(&mut self, stru: Struct) -> io::Result<()> {
        self.notify_typ(stru.name.clone());

        let has_lifetime = self.fields_need_lifetime(&stru.fields);
        if has_lifetime {
            self.typ_with_lifetime.insert(stru.name.clone());
        }
        let ffi_typ = self.emit_ffi_struct(&stru, None, false, false, false)?;
        self.emit_ffi_field_list_accessors(&ffi_typ, &stru.name, &stru.fields, None, false)?;
        let ffi_it_typ = self.emit_ffi_iterator(&stru.name, &ffi_typ, has_lifetime)?;

        let rs_typ = self.emit_rs_struct(&ffi_typ, &stru, has_lifetime)?;
        self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ, has_lifetime, false)?;

        let stru_sz = self.compute_ffi_struct_size(&stru);
        self.reg_type(
            stru.name.clone(),
            ffi_typ,
            rs_typ,
            stru_sz,
            false,
            false,
            self.fields_are_pod(&stru.fields),
        );

        self.structs.push(stru);

        Ok(())
    }

    fn emit_union(&mut self, stru: Struct) -> io::Result<()> {
        self.notify_typ(stru.name.clone());

        let ffi_sz = self.compute_ffi_union_size(&stru);
        let ffi_typ = self.ffi_decl_type_name(&stru.name);

        {
            let out = &mut self.ffi;
            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            writeln!(out, "// union")?;
            writeln!(out, "#[repr(C)]")?;
            writeln!(out, "#[derive(Debug)]")?;
            writeln!(out, "pub struct {} {{", &ffi_typ)?;
            writeln!(out, "    pub data: [u8; {}],", ffi_sz)?;
            writeln!(out, "}}")?;
            emit_copy_clone(out, &ffi_typ)?;
        }

        self.emit_ffi_iterator(&stru.name, &ffi_typ, false)?;

        let rs_typ = rs::type_name(&stru.name);

        {
            let out = &mut self.rs_buf;

            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            emit_type_alias(out, &rs_typ, &ffi_typ)?;
        }

        self.emit_rs_union_impl(&rs_typ, ffi_sz, &stru)?;

        let ffi_it_typ = self.ffi_iterator_name(&stru.name);

        self.emit_rs_iterator(&stru.name, &rs_typ, &ffi_it_typ, false, true)?;

        self.reg_type(stru.name, ffi_typ, rs_typ, Some(ffi_sz), true, false, false);

        Ok(())
    }

    fn emit_error(&mut self, number: i32, stru: Struct) -> io::Result<()> {
        self.notify_typ(stru.name.clone() + "Error");

        ffi::emit_opcode(&mut self.ffi, &self.xcb_mod_prefix, &stru.name, number)?;

        rs::emit_opcode(&mut self.rs_buf, &stru.name, number)?;

        let fields = {
            let mut fields = vec![
                StructField::Field {
                    name: "response_type".into(),
                    typ: "CARD8".into(),
                    enu: None,
                },
                StructField::Field {
                    name: "error_code".into(),
                    typ: "CARD8".into(),
                    enu: None,
                },
                StructField::Field {
                    name: "sequence".into(),
                    typ: "CARD16".into(),
                    enu: None,
                },
            ];
            for f in stru.fields.into_iter() {
                fields.push(f);
            }
            fields
        };
        let stru = Struct {
            name: stru.name + "Error",
            fields,
            doc: stru.doc,
        };

        let ffi_typ = self.emit_ffi_struct(&stru, None, false, false, false)?;

        let rs_typ = rs::type_name(&stru.name);

        rs::emit_error(&mut self.rs, &ffi_typ, &rs_typ)?;

        self.reg_type(
            stru.name,
            ffi_typ,
            rs_typ,
            None,
            false,
            false,
            self.fields_are_pod(&stru.fields),
        );

        Ok(())
    }

    fn emit_error_copy(&mut self, name: &str, number: i32, error_ref: &str) -> io::Result<()> {
        ffi::emit_opcode(&mut self.ffi, &self.xcb_mod_prefix, &name, number)?;
        let new_name = name.to_owned() + "Error";
        let old_name = error_ref.to_owned() + "Error";

        let new_ffi_typ = self.ffi_decl_type_name(&new_name);
        let old_ffi_typ = self.ffi_use_type_name(&old_name);

        emit_type_alias(&mut self.ffi, &new_ffi_typ, &old_ffi_typ)?;

        let rs_typ = rs::type_name(&new_name);

        rs::emit_error(&mut self.rs, &new_ffi_typ, &rs_typ)?;
        rs::emit_opcode(&mut self.rs_buf, &name, number)?;

        self.notify_typ(new_name);

        Ok(())
    }

    fn emit_event(
        &mut self,
        number: i32,
        stru: Struct,
        no_seq_number: bool,
        xge: bool,
    ) -> io::Result<()> {
        let Struct {
            name: orig_name,
            fields: mut orig_fields,
            doc,
        } = stru;
        let event_typ = orig_name.clone() + "Event";
        self.notify_typ(event_typ.clone());

        ffi::emit_opcode(&mut self.ffi, &self.xcb_mod_prefix, &orig_name, number)?;

        let opcopies = self
            .evcopies
            .remove(&orig_name)
            .expect("missing event copies");

        let (fields, must_pack) = {
            let mut fields = vec![StructField::Field {
                name: "response_type".into(),
                typ: "CARD8".into(),
                enu: None,
            }];
            let mut must_pack = false;

            let mut sz = 1; // response_type size

            if xge {
                fields.push(StructField::Field {
                    name: "extension".into(),
                    typ: "CARD8".into(),
                    enu: None,
                });
                fields.push(StructField::Field {
                    name: "sequence".into(),
                    typ: "CARD16".into(),
                    enu: None,
                });
                fields.push(StructField::Field {
                    name: "length".into(),
                    typ: "CARD32".into(),
                    enu: None,
                });
                fields.push(StructField::Field {
                    name: "event_type".into(),
                    typ: "CARD16".into(),
                    enu: None,
                });
                sz += 9;
            } else if !no_seq_number {
                fields.push(orig_fields.remove(0));
                fields.push(StructField::Field {
                    name: "sequence".into(),
                    typ: "CARD16".into(),
                    enu: None,
                });
            }

            for f in orig_fields.into_iter() {
                if xge {
                    let fsz = self.compute_ffi_struct_field_sizeof(&f);
                    fields.push(f);
                    if sz < 32 {
                        sz += fsz.expect("can't compute ffi full_sequence position");
                        if sz == 32 {
                            fields.push(StructField::Field {
                                name: "full_sequence".into(),
                                typ: "CARD32".into(),
                                enu: None,
                            });
                        }
                    } else if let Some(fsz) = fsz {
                        if fsz == 8 {
                            must_pack = true;
                        }
                    }
                } else {
                    fields.push(f);
                }
            }
            (fields, must_pack)
        };
        let stru = Struct {
            name: event_typ,
            fields,
            doc,
        };

        let ffi_typ = self.emit_ffi_struct(&stru, None, must_pack, false, false)?;
        let ffi_sz = self.compute_ffi_struct_size(&stru);

        for c in opcopies.iter() {
            ffi::emit_opcode(&mut self.ffi, &self.xcb_mod_prefix, &c.name, c.number)?;
            let new_name = c.name.to_owned() + "Event";

            let new_ffi_typ = self.ffi_decl_type_name(&new_name);
            let old_ffi_typ = self.ffi_use_type_name(&stru.name);

            emit_type_alias(&mut self.ffi, &new_ffi_typ, &old_ffi_typ)?;
        }

        let rs_typ = self.emit_rs_event(&orig_name, number, &stru, &ffi_typ, &opcopies, xge)?;

        self.reg_type(
            stru.name,
            ffi_typ,
            rs_typ,
            ffi_sz,
            false,
            false,
            self.fields_are_pod(&stru.fields),
        );

        Ok(())
    }

    fn emit_request(&mut self, mut req: Request) -> io::Result<()> {
        let request_typ = req.name.clone() + "Request";
        self.notify_typ(request_typ.clone());

        let fields = {
            let mut params = req.params.clone();
            let mut fields = vec![
                StructField::Field {
                    name: "major_opcode".into(),
                    typ: "CARD8".into(),
                    enu: None,
                },
                if self.xcb_mod == "xproto" {
                    if params.is_empty() {
                        StructField::Pad("pad0".into(), 1)
                    } else {
                        let p = params.remove(0);
                        // assert!(self.compute_ffi_struct_field_sizeof(&p) == Some(1usize));
                        p
                    }
                } else {
                    StructField::Field {
                        name: "minor_opcode".into(),
                        typ: "CARD8".into(),
                        enu: None,
                    }
                },
                StructField::Field {
                    name: "length".into(),
                    typ: "CARD16".into(),
                    enu: None,
                },
            ];
            fields.append(&mut params);
            fields
        };

        for f in &fields {
            if let StructField::Switch(name, expr, cases) = f {
                let toplevel = req.name.to_string() + "Request";
                self.notify_typ(toplevel.clone());
                self.emit_ffi_switch_struct(&req.name, &name, &expr, &cases, &toplevel, None)?;
                self.emit_rs_switch_typedef(&req.name, &name, &cases, &toplevel, None)?;
            }
        }

        ffi::emit_opcode(&mut self.ffi, &self.xcb_mod_prefix, &req.name, req.opcode)?;

        let stru = Struct {
            name: request_typ,
            fields,
            doc: req.doc.clone(),
        };
        self.emit_ffi_struct(&stru, None, false, false, false)?;

        let void = req.reply.is_none();
        let (ffi_cookie, check_name, checked) = if void {
            ("VoidCookie".to_string(), req.name.clone() + "Checked", true)
        } else {
            (
                req.name.clone() + "Cookie",
                req.name.clone() + "Unchecked",
                false,
            )
        };

        let rs_cookie = if ffi_cookie == "VoidCookie" {
            String::from("base::VoidCookie")
        } else {
            rs::type_name(&ffi_cookie)
        };

        rs::emit_opcode(&mut self.rs_buf, &req.name, req.opcode)?;

        if let Some(reply) = req.reply.take() {
            let (ffi_cookie, ffi_reply_fn, ffi_reply_typ) =
                self.emit_ffi_reply(&req.name, reply.clone())?;
            self.emit_rs_reply(
                &req.name,
                &ffi_cookie,
                &ffi_reply_fn,
                &ffi_reply_typ,
                &rs_cookie,
                reply,
            )?;
            self.notify_typ(req.name.clone() + "Reply");
            self.notify_typ(req.name.clone() + "Cookie");
        }

        let ffi_fn_name = ffi::request_fn_name(&self.xcb_mod_prefix, &req.name);
        let ffi_check_fn_name = ffi::request_fn_name(&self.xcb_mod_prefix, &check_name);
        let rs_fn_name = rs::request_fn_name(&req.name);
        let rs_check_fn_name = rs::request_fn_name(&check_name);

        self.emit_ffi_req_fn(&req.name, &ffi_fn_name, &ffi_cookie, &req.params, &stru.doc)?;
        self.emit_ffi_req_fn(
            &req.name,
            &ffi_check_fn_name,
            &ffi_cookie,
            &req.params,
            &stru.doc,
        )?;
        self.emit_rs_req_fn(
            &req.name,
            &rs_fn_name,
            &ffi_fn_name,
            &rs_cookie,
            &req.params,
            &stru.doc,
            !checked,
        )?;
        self.emit_rs_req_fn(
            &req.name,
            &rs_check_fn_name,
            &ffi_check_fn_name,
            &rs_cookie,
            &req.params,
            &stru.doc,
            checked,
        )?;

        Ok(())
    }
}

fn has_fd(fields: &[StructField]) -> bool {
    for f in fields.iter() {
        if let StructField::Fd(_) = f {
            return true;
        }
    }
    false
}

fn make_field(name: String, typ: String) -> StructField {
    StructField::Field {
        name,
        typ,
        enu: None,
    }
}

fn expr_fixed_length(expr: &Expr<usize>) -> Option<usize> {
    match expr {
        Expr::EnumRef { .. } => None, // FIXME: get the value of the enum item
        Expr::Value(val) => Some(*val),
        Expr::Popcount(ex) => expr_fixed_length(&ex).map(|sz| sz.count_ones() as _),
        Expr::Op(op, lhs, rhs) => match (expr_fixed_length(lhs), expr_fixed_length(rhs)) {
            (Some(lhs), Some(rhs)) => match op.as_str() {
                "+" => Some(lhs + rhs),
                "-" => Some(lhs - rhs),
                "*" => Some(lhs * rhs),
                "/" => Some(lhs / rhs),
                _ => panic!("Unexpected binary operator in Expr: {}", op),
            },
            _ => None,
        },
        Expr::Unop(op, val) => expr_fixed_length(val).map(|val| match op.as_str() {
            "~" => !val,
            _ => panic!("Unexpected unary operator in Expr: {}", op),
        }),
        _ => None,
    }
}

fn capitalize(s: &str) -> String {
    let mut ch = s.chars();
    match ch.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(ch).collect(),
    }
}

// insert a underscore before each uppercase/digit preceded or follwed by lowercase
// do not apply to the first char
fn tit_split<H>(name: &str, is_high: H) -> String
where
    H: Fn(char) -> bool,
{
    if name.len() <= 1 {
        return name.into();
    }

    let is_low = |c: char| c.is_ascii_lowercase();

    let mut res = String::new();
    let mut ch = name.chars();
    let mut prev = ch.next().unwrap();
    res.push(prev);
    let mut c = ch.next().unwrap();

    for next in ch {
        if is_low(prev) && is_high(c) || is_high(c) && is_low(next) {
            if prev != '_' {
                res.push('_');
            }
        }
        res.push(c);
        prev = c;
        c = next;
    }
    if is_low(prev) && is_high(c) && prev != '_' {
        res.push('_');
    }
    res.push(c);

    res
}

// capitalize each substring beginning by uppercase
// said otherwise: every upper preceded by another upper and followed by a upper is turned to lower
fn tit_cap(name: &str) -> String {
    if name.len() <= 1 {
        return name.into();
    }

    let is_high = |c: char| c.is_ascii_uppercase() | c.is_ascii_digit();

    let mut res = String::new();
    let mut ch = name.chars();
    let mut prev = ch.next().unwrap();
    res.push(prev.to_ascii_uppercase());
    let mut c = ch.next().unwrap();

    for next in ch {
        if c != '_' {
            if is_high(c) && is_high(prev) && (is_high(next) || next == '_') {
                res.push(c.to_ascii_lowercase())
            } else if prev == '_' && c != '_' {
                res.push(c.to_ascii_uppercase())
            } else {
                res.push(c)
            }
        }
        prev = c;
        c = next;
    }
    if is_high(c) && is_high(prev) {
        res.push(c.to_ascii_lowercase());
    } else {
        res.push(c);
    }

    res
}

#[test]
fn test_tit_cap() {
    assert!(tit_cap("SomeString") == "SomeString");
    assert!(tit_cap("WINDOW") == "Window");
    assert!(tit_cap("CONTEXT_TAG") == "ContextTag");
    assert!(tit_cap("value_list") == "ValueList");
    assert!(tit_cap("GContext") == "GContext");
    assert!(tit_cap("IDChoice") == "IdChoice");
}

fn tit_dig_split(name: &str) -> String {
    tit_split(name, |c| c.is_ascii_uppercase() || c.is_ascii_digit())
}

#[test]
fn test_tit_dig_split() {
    assert_eq!(tit_dig_split("SomeString"), "Some_String");
    assert_eq!(tit_dig_split("WINDOW"), "WINDOW");
}

const KEYWORDS: &[&str] = &["type", "str", "match", "new", "await"];

fn field_name(name: &str) -> String {
    let mut res = tit_split(name, |c| c.is_ascii_uppercase()).to_ascii_lowercase();

    if KEYWORDS.contains(&res.as_str()) {
        res.push('_');
    }

    res
}

#[test]
fn test_field_name() {
    assert_eq!(field_name("groupMaps"), "group_maps");
    assert_eq!(field_name("num_FB_configs"), "num_fb_configs");
    assert_eq!(field_name("sizeID"), "size_id");
    assert_eq!(field_name("new"), "new_");
    assert_eq!(field_name("byte1"), "byte1");
}

fn extract_module(typ: &str) -> (Option<&str>, &str) {
    let len = typ.len();
    let colon = typ.as_bytes().iter().position(|b| *b == b':');
    if let Some(colon) = colon {
        (Some(&typ[0..colon]), &typ[colon + 1..len])
    } else {
        (None, typ)
    }
}

fn qualified_name(xcb_mod: &str, module: &Option<&str>, name: &str) -> String {
    if let Some(module) = module {
        if module != &xcb_mod {
            format!("{}::{}", module, &name)
        } else {
            name.into()
        }
    } else {
        name.into()
    }
}
fn emit_type_alias<Out: Write>(out: &mut Out, new_typ: &str, old_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub type {} = {};", new_typ, old_typ)?;
    Ok(())
}

fn emit_doc_str<Out: Write>(out: &mut Out, docstr: &str, indent: &str) -> io::Result<()> {
    let mut wrote = false;
    if !docstr.is_empty() {
        for s in docstr.split('\n') {
            let s = s.trim_end();
            if !s.is_empty() {
                writeln!(out, "///{}{}", indent, s.trim_end())?;
            } else {
                writeln!(out, "///")?;
            }
            wrote = true;
        }
    }
    if !wrote {
        writeln!(out, "///")
    } else {
        Ok(())
    }
}

fn emit_doc_text<Out: Write>(out: &mut Out, doc: &Option<Doc>) -> io::Result<()> {
    if let Some(doc) = doc {
        if !doc.brief.is_empty() {
            emit_doc_str(out, &doc.brief, " ")?;
        }
        if !doc.brief.is_empty() && !doc.text.is_empty() {
            writeln!(out, "///")?;
        }
        if !doc.text.is_empty() {
            emit_doc_str(out, &doc.text, " ")?;
        }
    }
    Ok(())
}

fn emit_doc_field<Out: Write>(out: &mut Out, doc: &Option<Doc>, field: &str) -> io::Result<()> {
    if let Some(doc) = doc {
        if let Some(f) = doc.fields.iter().find(|f| f.name == field) {
            emit_doc_str(out, &f.text, " ")
        } else {
            //writeln!(out, "///")
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn emit_doc_field_indent<Out: Write>(
    out: &mut Out,
    doc: &Option<Doc>,
    field: &str,
    indent: &str,
) -> io::Result<()> {
    if let Some(doc) = doc {
        if let Some(f) = doc.fields.iter().find(|f| f.name == field) {
            emit_doc_str(out, &f.text, indent)?;
        }
    }
    Ok(())
}

fn emit_copy_clone<Out: Write>(out: &mut Out, typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "impl Copy for {} {{}}", &typ)?;
    writeln!(out, "impl Clone for {} {{", &typ)?;
    writeln!(out, "    fn clone(&self) -> {} {{ *self }}", &typ)?;
    writeln!(out, "}}")?;

    Ok(())
}

fn emit_enum<Out, Items>(
    out: &mut Out,
    typ: &str,
    items: Items,
    doc: &Option<Doc>,
) -> io::Result<()>
where
    Out: Write,
    Items: Iterator<Item = EnumItem>,
{
    writeln!(out)?;
    emit_doc_text(out, doc)?;
    writeln!(out, "pub type {} = u32;", typ)?;
    for item in items {
        emit_doc_field(out, doc, &item.id)?;
        let val = format!("0x{:02x}", item.value);
        writeln!(out, "pub const {}: {} = {};", item.name, typ, val)?;
    }
    Ok(())
}

struct ListField {
    name: String,
    typ: String,
    lenfield: String,
    // lenfield_typ: String,
}

impl ListField {
    fn fetch_from(fields: &[StructField]) -> Vec<ListField> {
        let mut res = Vec::new();
        for f in fields {
            match f {
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    let lenfield = expr_lenfield(&len_expr);
                    if let Some(lenfield) = lenfield {
                        let name = name.clone();
                        let typ = typ.clone();
                        let lenfield = lenfield.to_string();
                        // let lenfield_typ = fields
                        //     .iter()
                        //     .find_map(|f| match f {
                        //         StructField::Field { name, typ, .. } => {
                        //             if name == &lenfield {
                        //                 Some(typ.clone())
                        //             } else {
                        //                 None
                        //             }
                        //         }
                        //         _ => None,
                        //     })
                        //     .expect("can't find lenfield type");
                        res.push(ListField {
                            name,
                            typ,
                            lenfield,
                            // lenfield_typ,
                        });
                    }
                }
                StructField::ListNoLen { name, typ } => {
                    let name = name.clone();
                    let typ = typ.clone();
                    let lenfield = name.clone() + "_len";
                    res.push(ListField {
                        name,
                        typ,
                        lenfield,
                        // lenfield_typ: "CARD32".to_string(),
                    });
                }
                _ => {}
            }
        }
        res
    }
}

fn expr_lenfield(expr: &Expr<usize>) -> Option<&str> {
    match expr {
        Expr::FieldRef(name) => Some(name),
        Expr::Op(_, lhs, rhs) => expr_lenfield(&lhs).or_else(|| expr_lenfield(&rhs)),
        Expr::Unop(_, rhs) => expr_lenfield(&rhs),
        Expr::Popcount(e) => expr_lenfield(&e),
        _ => None,
    }
}

fn request_has_template(params: &[StructField]) -> bool {
    for f in params.iter() {
        if let StructField::List { typ, .. } = f {
            if typ == "void" {
                return true;
            }
        }
    }
    false
}

fn enum_suffix_exception(xcb_mod: &str, enum_typ: &str) -> bool {
    match (xcb_mod, enum_typ) {
        ("render", "Picture") => true,
        ("present", "Event") => true,
        _ => false,
    }
}
