use crate::cg;
use crate::cg::util;
use crate::ir;

use super::{
    doc::DocField, CodeGen, Doc, Expr, Field, HasWireLayout, QualifiedRsTyp, RsTyp, StructStyle,
    TypeInfo, WireSz,
};

use std::io::{self, Write};

/// struct of external parameters passed to functions such as compute_len
/// This is very often `()` for which `None` is used, but sometimes an field external to struct is needed
/// to compute the len of a list (i.e. <paramref> element types)
#[derive(Clone, Debug)]
pub(super) struct ParamsStruct {
    pub rs_typ: String,
    pub params: Vec<String>,
}

impl ParamsStruct {
    pub fn emit<O: Write>(&self, out: &mut O) -> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "#[derive(Copy, Clone, Debug)]")?;
        writeln!(out, "pub struct {} {{", self.rs_typ)?;
        for p in &self.params {
            writeln!(out, "    pub {}: usize,", p)?;
        }
        writeln!(out, "}}")?;
        Ok(())
    }
}

impl RsTyp for Option<&ParamsStruct> {
    fn rs_typ(&self) -> &str {
        match self {
            Some(params_struct) => &params_struct.rs_typ,
            _ => "()",
        }
    }
}

#[derive(Debug)]
pub(super) struct ResolvedFields {
    pub fields: Vec<Field>,
    pub wire_sz: Expr,
    pub has_wire_layout: bool,
    pub unresolved_fieldrefs: Vec<String>,
    pub params_struct: Option<ParamsStruct>,
}

impl CodeGen {
    pub(super) fn resolve_struct(
        &mut self,
        typ: &str,
        fields: &[ir::Field],
        doc: &Option<ir::Doc>,
    ) {
        let rs_typ = cg::rust_type_name(typ);
        let doc = self.resolve_doc(doc.clone());
        let ResolvedFields {
            fields,
            wire_sz,
            has_wire_layout,
            params_struct,
            // unresolved_fieldrefs,
            ..
        } = self.resolv_struct_fields(&rs_typ, "", fields, doc.as_ref());

        // assert!(unresolved_fieldrefs.is_empty());

        let typ_info = TypeInfo::Struct {
            module: None,
            rs_typ,
            fields,
            wire_sz,
            has_wire_layout,
            params_struct,
            doc,
        };
        self.register_typ(typ.to_string(), typ_info);
    }

    pub(super) fn resolv_struct_fields(
        &mut self,
        struct_rs_typ: &str,
        parent_switch: &str,
        fields: &[ir::Field],
        doc: Option<&Doc>,
    ) -> ResolvedFields {
        let mut vec = Vec::new();
        let mut wire_off = Expr::Value(0usize);
        let mut has_wire_layout = true;
        let mut need_compute_offset = false;
        // fields that are referenced in list length of this struct
        let mut fieldrefs = Vec::new();
        // external fields that are refereced in list length of this struct
        let mut paramrefs = Vec::new();
        // fields of this struct that are used as list length of other struct
        let mut paramfields = Vec::new();
        let mut unresolved_fieldrefs = Vec::new();

        let has_prop_field = fields.iter().any(
            |f| matches!(f, ir::Field::Field{name, typ, ..} if name == "format" && typ == "CARD8"),
        ) && fields
            .iter()
            .any(|f| matches!(f, ir::Field::List{typ, ..} if typ == "void"));

        for f in fields {
            let f_sz = match f {
                ir::Field::Field {
                    name,
                    typ,
                    r#enum,
                    mask,
                    ..
                } => {
                    let FieldInfo {
                        name,
                        module,
                        rs_typ,
                        wire_sz,
                        has_wire_layout: hfl,
                        struct_style,
                        is_union,
                        is_xid,
                        is_mask,
                        params_struct,
                        doc,
                        ..
                    } = self.get_field_info(name, typ, r#enum.is_some(), doc);

                    if !hfl || (rs_typ != "u32" && (r#enum.is_some() || (mask.is_some()))) {
                        has_wire_layout = false;
                    }

                    let r#enum = r#enum.as_ref().map(|typ| self.get_mod_rs_typ(typ));
                    let mask = mask.as_ref().map(|typ| self.get_mod_rs_typ(typ));

                    let wire_sz = wire_sz.reduce();

                    let is_prop_format = has_prop_field && name == "format";

                    vec.push(Field::Field {
                        name,
                        module,
                        rs_typ,
                        wire_off: wire_off.clone(),
                        wire_sz: wire_sz.clone(),
                        struct_style,
                        params_struct,
                        doc,
                        is_mask: is_mask || mask.is_some(),
                        r#enum,
                        mask,
                        is_fieldref: false,
                        is_paramref: false,
                        is_copy: hfl,
                        is_union,
                        is_xid,
                        need_compute_offset,
                        is_prop_format,
                    });
                    wire_sz
                }
                ir::Field::Pad(wire_sz) => {
                    has_wire_layout = false;
                    let wire_off = wire_off.clone().reduce();
                    vec.push(Field::Pad {
                        wire_off,
                        wire_sz: Expr::Value(*wire_sz),
                    });
                    Expr::Value(*wire_sz)
                }
                ir::Field::AlignPad(sz) => {
                    has_wire_layout = false;
                    let wire_off = wire_off.clone().reduce();
                    let wire_sz = Expr::AlignPad(*sz, Box::new(wire_off.clone())).reduce();
                    vec.push(Field::AlignPad {
                        wire_off,
                        wire_sz: wire_sz.clone(),
                    });
                    wire_sz
                }
                ir::Field::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    let FieldInfo {
                        name,
                        module,
                        mut rs_typ,
                        typ,
                        wire_sz,
                        has_wire_layout: hfl,
                        struct_style,
                        params_struct,
                        doc,
                        ..
                    } = self.get_field_info(name, typ, false, doc);

                    let len_expr = self.resolve_expr(len_expr);

                    wire_sz.fetch_paramrefs_owned(&mut paramfields);

                    let wire_sz = Expr::Op(
                        "*".to_string(),
                        Box::new(len_expr.clone()),
                        Box::new(wire_sz),
                    );
                    let wire_sz = wire_sz.reduce();

                    let fixed_str = matches!((rs_typ.as_str(), &wire_sz), ("char", Expr::Value(_)));
                    if fixed_str {
                        rs_typ = "u8".into();
                    } else if !hfl || !matches!(wire_sz, Expr::Value(_)) {
                        has_wire_layout = false;
                    }

                    len_expr.fetch_fieldrefs_owned(&mut fieldrefs);
                    len_expr.fetch_paramrefs_owned(&mut paramrefs);

                    // assert!(!need_compute_offset, "{}::{}", self.xcb_mod, name);
                    let is_prop = has_prop_field && typ == "void";

                    vec.push(Field::List {
                        name,
                        module,
                        rs_typ,
                        wire_off: wire_off.clone(),
                        wire_sz: wire_sz.clone(),
                        struct_style,
                        is_fieldref: false,
                        len_expr,
                        need_compute_offset,
                        params_struct,
                        is_prop,
                        doc,
                    });

                    if let Some(StructStyle::DynBuf) = struct_style {
                        need_compute_offset = true;
                    }

                    wire_sz
                }
                ir::Field::ListNoLen { typ, name } => {
                    let FieldInfo {
                        name,
                        module,
                        rs_typ,
                        typ,
                        struct_style,
                        params_struct,
                        doc,
                        ..
                    } = self.get_field_info(name, typ, false, doc);

                    has_wire_layout = false;
                    let is_prop = has_prop_field && typ == "void";

                    vec.push(Field::List {
                        name,
                        module,
                        rs_typ,
                        wire_off: wire_off.clone(),
                        wire_sz: Expr::UntilEnd,
                        struct_style,
                        params_struct,
                        is_fieldref: false,
                        len_expr: Expr::UntilEnd,
                        need_compute_offset,
                        is_prop,
                        doc,
                    });

                    Expr::Unknown("list no len".to_string())
                }
                ir::Field::ValueParam { .. } => {
                    unreachable!("<valueparam> not anymore used with xcb-1.14")
                }
                ir::Field::Switch(name, expr, cases) => {
                    let expr = self.resolve_expr(expr);
                    let field = self.resolve_switch(
                        struct_rs_typ,
                        parent_switch,
                        name,
                        &expr,
                        &wire_off,
                        cases,
                        &mut vec,
                        need_compute_offset,
                    );
                    expr.fetch_fieldrefs_owned(&mut fieldrefs);
                    if let Field::Switch { params_struct, .. } = &field {
                        let mut params = params_struct.params.clone();
                        fieldrefs.append(&mut params);
                    } else {
                        unreachable!();
                    };
                    vec.push(field);
                    need_compute_offset = true;
                    has_wire_layout = false;
                    Expr::Unknown("switch len".to_string())
                }
                ir::Field::Expr { name, typ, expr } => {
                    let FieldInfo {
                        name, typ, wire_sz, ..
                    } = self.get_field_info(name, typ, false, doc);

                    has_wire_layout = false;

                    let wire_sz = wire_sz.reduce();
                    let expr = self.resolve_expr(expr);

                    vec.push(Field::Expr {
                        name,
                        typ,
                        wire_off: wire_off.clone(),
                        wire_sz: wire_sz.clone(),
                        expr,
                    });
                    wire_sz
                }
                ir::Field::Fd(name) => {
                    eprintln!("partial treatment of Fd field: {}::{}", struct_rs_typ, name);
                    let doc = self.doc_lookup_field(doc, name);

                    vec.push(Field::Field {
                        name: name.clone(),
                        module: None,
                        rs_typ: "RawFd".to_string(),
                        wire_off: wire_off.clone(),
                        wire_sz: Expr::Value(4),
                        struct_style: None,
                        params_struct: None,
                        doc,
                        is_fieldref: false,
                        is_paramref: false,
                        is_copy: true,
                        is_union: false,
                        is_xid: false,
                        is_mask: false,
                        r#enum: None,
                        mask: None,
                        need_compute_offset: false,
                        is_prop_format: false,
                    });
                    has_wire_layout = false;
                    Expr::Value(4)
                } // f => unreachable!("{:#?}", f),
            };

            let new_off = Expr::Op("+".to_string(), Box::new(wire_off), Box::new(f_sz));
            wire_off = new_off.reduce();
        }

        for fr in fieldrefs {
            let mut resolved = false;
            for f in &mut vec {
                match f {
                    Field::Field {
                        name, is_fieldref, ..
                    } if name == &fr => {
                        *is_fieldref = true;
                        resolved = true;
                        break;
                    }
                    Field::List {
                        name, is_fieldref, ..
                    } if name == &fr => {
                        *is_fieldref = true;
                        resolved = true;
                        break;
                    }
                    _ => {}
                }
            }
            if !resolved {
                unresolved_fieldrefs.push(fr);
            }
        }
        for pf in &paramfields {
            for f in &mut vec {
                match f {
                    Field::Field {
                        name, is_paramref, ..
                    } if name == pf => {
                        *is_paramref = true;
                        break;
                    }
                    _ => {}
                }
            }
        }
        let params_struct = if paramrefs.is_empty() {
            None
        } else {
            Some(ParamsStruct {
                rs_typ: struct_rs_typ.to_string() + "Params",
                params: paramrefs,
            })
        };

        ResolvedFields {
            fields: vec,
            wire_sz: wire_off,
            has_wire_layout,
            unresolved_fieldrefs,
            params_struct,
        }
    }

    fn get_mod_rs_typ(&self, typ: &str) -> (Option<String>, String) {
        let (module, typ) = util::extract_module(typ);
        let typinfo = self.find_typinfo(module, typ);
        let module = typinfo.module();
        let rs_typ = typinfo.rs_typ();
        (module.map(str::to_owned), rs_typ.to_string())
    }

    fn get_field_info(
        &self,
        name: &str,
        typ: &str,
        alt_repr: bool,
        doc: Option<&Doc>,
    ) -> FieldInfo {
        let name = cg::rust_field_name(name);
        let (module, typ) = util::extract_module(typ);
        let typinfo = self.find_typinfo(module, typ);
        let module = typinfo.module();
        let rs_typ = typinfo.rs_typ();
        let wire_sz = typinfo.wire_sz();
        let is_union = matches!(typinfo, TypeInfo::Union { .. } | TypeInfo::XidUnion { .. });
        let is_xid = matches!(typinfo, TypeInfo::Xid { .. } | TypeInfo::XidUnion { .. });
        let is_mask = matches!(typinfo, TypeInfo::Mask { .. });
        let params_struct = match typinfo {
            TypeInfo::Struct { params_struct, .. } => params_struct.clone(),
            _ => None,
        };
        let doc = self.doc_lookup_field(doc, &name);

        FieldInfo {
            name,
            module: module.map(str::to_owned),
            typ: typ.to_string(),
            rs_typ: rs_typ.to_string(),
            wire_sz,
            has_wire_layout: !alt_repr && typinfo.has_wire_layout(),
            struct_style: typinfo.struct_style(),
            params_struct,
            doc,
            is_union,
            is_xid,
            is_mask,
        }
    }

    fn build_params_expr(
        &self,
        params_struct: Option<&ParamsStruct>,
        module: Option<&str>,
        acc_pref: &str,
        acc_post: &str,
    ) -> String {
        if params_struct.is_none() {
            return "()".to_string();
        }
        let params_struct = params_struct.unwrap();
        let q_rs_typ = (module, params_struct.rs_typ.as_str()).qualified_rs_typ();
        let mut expr = q_rs_typ + " {";
        for (i, p) in params_struct.params.iter().enumerate() {
            expr.push_str(&format!("{}: {}{}{} as usize", p, acc_pref, p, acc_post));
            if i < params_struct.params.len() - 1 {
                expr.push_str(", ");
            }
        }
        expr.push('}');
        expr
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn emit_struct<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        fields: &[Field],
        wire_sz: &Expr,
        has_wire_layout: bool,
        params_struct: Option<&ParamsStruct>,
        doc: Option<&Doc>,
    ) -> io::Result<()> {
        if !self.rs_typ_is_needed(rs_typ) {
            return Ok(());
        }

        match (has_wire_layout, wire_sz) {
            (true, Expr::Value(wire_sz)) => {
                self.emit_wire_layout_struct(out, rs_typ, fields, *wire_sz, doc)?
            }
            (false, Expr::Value(wire_sz)) => {
                self.emit_fix_buf_struct(out, rs_typ, fields, *wire_sz, doc)?
            }
            (false, _) => self.emit_dyn_buf_struct(out, rs_typ, fields, params_struct, doc)?,
            _ => unreachable!("{}::{}", self.xcb_mod, rs_typ),
        }

        Ok(())
    }

    fn emit_wire_layout_struct<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        fields: &[Field],
        wire_sz: usize,
        doc: Option<&Doc>,
    ) -> io::Result<()> {
        writeln!(out)?;
        if let Some(doc) = doc {
            doc.emit(out, 0)?;
        }
        writeln!(out, "#[derive(Copy, Clone, Debug)]")?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "pub struct {} {{", rs_typ)?;
        for f in fields {
            match f {
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    r#enum,
                    mask,
                    doc,
                    ..
                } => {
                    let q_rs_typ = enum_mask_qualified_rs_typ(module, rs_typ, r#enum, mask);
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub {}: {},", name, q_rs_typ)?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    len_expr: Expr::Value(len),
                    doc,
                    ..
                } => {
                    let mod_rs_typ = (module, rs_typ);
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "    pub {}: [{}; {}],",
                        name,
                        mod_rs_typ.qualified_rs_typ(),
                        len
                    )?;
                }
                _ => unreachable!("emitting struct field {:?}", f),
            }
        }
        writeln!(out, "}}")?;

        self.emit_sizeof_test(out, rs_typ, wire_sz)?;

        writeln!(out)?;
        writeln!(out, "impl base::Wired for {} {{", rs_typ)?;
        writeln!(out, "    type Params = ();")?;
        writeln!(
            out,
            "    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {{ {} }}",
            wire_sz
        )?;
        writeln!(out)?;
        writeln!(out, "    fn wire_len(&self) -> usize {{ {} }}", wire_sz)?;
        writeln!(out)?;
        writeln!(
            out,
            "    fn serialize(&self, wire_buf: &mut [u8]) -> usize {{"
        )?;
        writeln!(out, "        let me = unsafe {{")?;
        writeln!(
            out,
            "            std::slice::from_raw_parts(self as *const {} as _, {})",
            rs_typ, wire_sz
        )?;
        writeln!(out, "        }};")?;
        writeln!(out, "        wire_buf.copy_from_slice(me);")?;
        writeln!(out, "        {}", wire_sz)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_fix_buf_struct<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        fields: &[Field],
        wire_sz: usize,
        doc: Option<&Doc>,
    ) -> io::Result<()> {
        writeln!(out)?;
        if let Some(doc) = doc {
            doc.emit(out, 0)?;
        }
        writeln!(out, "#[derive(Copy, Clone)]")?;
        writeln!(out, "pub struct {} {{", rs_typ)?;
        writeln!(out, "    data: [u8; {}],", wire_sz)?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "#[allow(unused_parens)]")?;
        writeln!(out, "impl {} {{", rs_typ)?;
        writeln!(
            out,
            "    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &{} {{",
            rs_typ
        )?;
        writeln!(
            out,
            "        debug_assert_eq!(data.as_ref().len(), {});",
            wire_sz
        )?;
        writeln!(
            out,
            "        &*(data.as_ref() as *const [u8] as *const {})",
            rs_typ
        )?;
        writeln!(out, "    }}")?;

        writeln!(out)?;
        writeln!(
            out,
            "    fn wire_ptr(&self) -> *const u8 {{ self.data.as_ptr() }}"
        )?;

        writeln!(out)?;
        writeln!(out, "    fn wire_len(&self) -> usize {{ self.data.len() }}")?;

        self.emit_struct_accessors(out, rs_typ, fields)?;

        writeln!(out, "}}")?;

        self.emit_sizeof_test(out, rs_typ, wire_sz)?;

        writeln!(out)?;
        writeln!(out, "impl base::Wired for {} {{", rs_typ)?;
        writeln!(out, "    type Params = ();")?;
        writeln!(
            out,
            "    unsafe fn compute_wire_len(_ptr: *const u8, _params: ()) -> usize {{ {} }}",
            wire_sz
        )?;
        writeln!(out)?;
        writeln!(out, "    fn wire_len(&self) -> usize {{ {} }}", wire_sz)?;
        writeln!(out)?;
        writeln!(
            out,
            "    fn serialize(&self, wire_buf: &mut [u8]) -> usize {{"
        )?;
        writeln!(out, "        wire_buf.copy_from_slice(&self.data);")?;
        writeln!(out, "        self.data.len()")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        self.emit_debug_impl(out, rs_typ, fields)?;

        Ok(())
    }

    fn emit_dyn_buf_struct<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        fields: &[Field],
        params_struct: Option<&ParamsStruct>,
        doc: Option<&Doc>,
    ) -> io::Result<()> {
        if let Some(params_struct) = params_struct {
            params_struct.emit(out)?;
        }
        writeln!(out)?;
        if let Some(doc) = doc {
            doc.emit(out, 0)?;
        }
        writeln!(out, "pub struct {} {{", rs_typ)?;
        writeln!(out, "    data: [u8],")?;
        writeln!(out, "}}")?;
        writeln!(out)?;
        writeln!(out, "#[allow(unused_parens)]")?;
        writeln!(out, "impl {} {{", rs_typ)?;
        writeln!(
            out,
            "    pub(crate) unsafe fn from_data<D: AsRef<[u8]> + ?Sized>(data: &D) -> &{} {{",
            rs_typ
        )?;
        if params_struct.is_none() {
            writeln!(
                out,
                "        debug_assert_eq!(data.as_ref().len(), {}::compute_wire_len(data.as_ref().as_ptr(), ()));",
                rs_typ
            )?;
        }
        writeln!(
            out,
            "        &*(data.as_ref() as *const [u8] as *const {})",
            rs_typ
        )?;
        writeln!(out, "    }}")?;

        writeln!(out)?;
        writeln!(
            out,
            "    fn wire_ptr(&self) -> *const u8 {{ self.data.as_ptr() }}"
        )?;

        let compute_wire_len_stmts =
            self.emit_compute_offset_and_get_stmts(out, rs_typ, fields, params_struct)?;
        self.emit_struct_accessors(out, rs_typ, fields)?;

        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl base::Wired for {} {{", rs_typ)?;
        writeln!(out, "    type Params = {};", params_struct.rs_typ())?;
        self.emit_compute_func(
            out,
            "compute_wire_len",
            params_struct,
            &compute_wire_len_stmts,
        )?;
        writeln!(out)?;
        writeln!(out, "    fn wire_len(&self) -> usize {{ self.data.len() }}")?;
        writeln!(out)?;
        writeln!(
            out,
            "    fn serialize(&self, wire_buf: &mut [u8]) -> usize {{"
        )?;
        writeln!(out, "        wire_buf.copy_from_slice(&self.data);")?;
        writeln!(out, "        self.data.len()")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "#[derive(Clone)]")?;
        writeln!(out, "pub struct {}Buf {{", rs_typ)?;
        writeln!(out, "    data: Vec<u8>,")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl {}Buf {{", rs_typ)?;
        writeln!(
            out,
            "    pub(crate) unsafe fn from_data(data: Vec<u8>) -> {}Buf {{",
            rs_typ
        )?;
        if params_struct.is_none() {
            writeln!(
                out,
                "        debug_assert_eq!({}::compute_wire_len(data.as_ptr(), ()), data.len());",
                rs_typ
            )?;
        }
        writeln!(out, "        {}Buf {{ data }}", rs_typ)?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl std::ops::Deref for {}Buf {{", rs_typ)?;
        writeln!(out, "    type Target = {};", rs_typ)?;
        writeln!(out, "    fn deref(&self) -> &Self::Target {{")?;
        writeln!(
            out,
            "        unsafe {{ {}::from_data(&self.data) }}",
            rs_typ
        )?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(
            out,
            "impl std::borrow::Borrow<{}> for {}Buf {{",
            rs_typ, rs_typ
        )?;
        writeln!(out, "    fn borrow(&self) -> &{} {{", rs_typ)?;
        writeln!(
            out,
            "        unsafe {{ {}::from_data(&self.data) }}",
            rs_typ
        )?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl std::borrow::ToOwned for {} {{", rs_typ)?;
        writeln!(out, "    type Owned = {}Buf;", rs_typ)?;
        writeln!(out, "    fn to_owned(&self) -> Self::Owned {{")?;
        writeln!(out, "        {}Buf {{", rs_typ)?;
        writeln!(out, "            data: self.data.to_vec()")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        self.emit_debug_impl(out, rs_typ, fields)?;
        self.emit_debug_impl(out, &(rs_typ.to_string() + "Buf"), fields)?;

        self.emit_dyn_buf_struct_iterator(out, rs_typ, params_struct)?;

        Ok(())
    }

    pub(super) fn field_compute_len_stmts(
        &self,
        field: &Field,
        stmts: &mut Vec<String>,
        struct_offset: &Expr,
    ) {
        let struct_offset = if let Expr::Value(0) = struct_offset {
            String::new()
        } else {
            self.build_rs_expr(struct_offset, "", "", &[]) + " + "
        };
        match field {
            Field::Field {
                name,
                struct_style: Some(StructStyle::DynBuf),
                params_struct,
                module,
                rs_typ,
                ..
            } => {
                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                let params_expr = self.build_params_expr(
                    params_struct.as_ref(),
                    module.as_ref().map(|s| s.as_str()),
                    "",
                    "",
                );
                stmts.push(format!("// {}", name));
                stmts.push(format!(
                    "sz += {}::compute_wire_len(ptr.add({}sz), {});",
                    q_rs_typ, struct_offset, params_expr
                ));
            }
            Field::Field {
                wire_sz,
                name,
                is_fieldref,
                is_paramref,
                rs_typ,
                ..
            } => {
                stmts.push(format!("// {}", name));

                if *is_fieldref || *is_paramref {
                    stmts.push(format!(
                        "let {} = *(ptr.add({}sz) as *const {});",
                        name, struct_offset, rs_typ
                    ));
                }

                stmts.push(format!(
                    "sz += {};",
                    self.build_rs_expr(wire_sz, "", "", &[])
                ));
            }
            Field::List {
                name,
                len_expr,
                module,
                rs_typ,
                struct_style: Some(StructStyle::DynBuf),
                params_struct,
                is_fieldref,
                ..
            } => {
                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                stmts.push(format!("// {}", name));
                if *is_fieldref {
                    stmts.push(format!("let mut {} = Vec::new();", name));
                }
                stmts.push(format!(
                    "for _ in 0 .. {} {{",
                    self.build_rs_expr(len_expr, "", "", &[])
                ));
                let params_expr = self.build_params_expr(
                    params_struct.as_ref(),
                    module.as_ref().map(|s| s.as_str()),
                    "",
                    "",
                );
                if *is_fieldref {
                    stmts.push(format!(
                        "    let len = {}::compute_wire_len(ptr.add({}sz), {});",
                        q_rs_typ, struct_offset, params_expr
                    ));
                    stmts.push(format!(
                        "    let data = std::slice::from_raw_parts(ptr.add({}sz), len);",
                        struct_offset
                    ));
                    stmts.push(format!("    {}.push({}::from_data(data));", name, q_rs_typ));
                    stmts.push("    sz += len;".to_string());
                } else {
                    stmts.push(format!(
                        "    sz += {}::compute_wire_len(ptr.add({}sz), {});",
                        q_rs_typ, struct_offset, params_expr
                    ));
                }
                stmts.push("}".to_string());
            }
            Field::List {
                name,
                wire_sz,
                is_fieldref,
                len_expr,
                module,
                rs_typ,
                ..
            } => {
                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                stmts.push(format!("// {}", name));
                if *is_fieldref {
                    stmts.push(format!("let {} = {{", name));
                    stmts.push(format!(
                        "    let len = {};",
                        self.build_rs_expr(len_expr, "", "", &[])
                    ));
                    stmts.push(format!(
                        "    let data = ptr.add({}sz) as *const {};",
                        struct_offset, q_rs_typ
                    ));
                    stmts.push(format!(
                        "    sz += len * std::mem::size_of::<{}>();",
                        q_rs_typ
                    ));
                    stmts.push("    std::slice::from_raw_parts(data, len)".to_string());
                    stmts.push("};".to_string());
                } else {
                    stmts.push(format!(
                        "sz += {};",
                        self.build_rs_expr(wire_sz, "", "", &[])
                    ));
                }
            }
            Field::Switch {
                name,
                module,
                rs_typ,
                params_struct,
                is_mask,
                ..
            } => {
                let params_expr = self.build_params_expr(
                    Some(params_struct),
                    module.as_ref().map(|m| m.as_str()),
                    "",
                    "",
                );
                let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                let impl_type = if *is_mask {
                    format!("<&[{}]>", q_rs_typ)
                } else {
                    q_rs_typ
                };
                stmts.push(format!("// {}", name));
                stmts.push(format!(
                    "sz += {}::compute_wire_len(ptr.add({}sz), {});",
                    impl_type, struct_offset, params_expr
                ));
            }
            Field::Pad { wire_sz, .. } => {
                stmts.push("// pad".to_string());
                stmts.push(format!(
                    "sz += {};",
                    self.build_rs_expr(wire_sz, "", "", &[])
                ));
            }
            Field::AlignPad {
                wire_sz: Expr::AlignPad(align, _),
                ..
            } => {
                stmts.push("// align pad".to_string());
                stmts.push(format!("sz += base::align_pad(sz, {});", align));
            }
            f => unreachable!("missed handling of field in compute_wire_len: {:?}", f),
        }
    }

    pub(super) fn emit_compute_func<O: Write>(
        &self,
        out: &mut O,
        fname: &str,
        params_struct: Option<&ParamsStruct>,
        stmts: &[String],
    ) -> io::Result<()> {
        writeln!(out)?;
        writeln!(
            out,
            "    unsafe fn {}(ptr: *const u8, {}params: {}) -> usize {{",
            fname,
            if params_struct.is_none() { "_" } else { "" },
            params_struct.rs_typ(),
        )?;
        if let Some(params_struct) = params_struct {
            writeln!(out, "        let {} {{", params_struct.rs_typ)?;
            for p in &params_struct.params {
                writeln!(out, "{}{},", cg::ind(3), p)?;
            }
            writeln!(out, "        }} = params;")?;
        }
        for s in stmts {
            writeln!(out, "        {}", s)?;
        }
        writeln!(out, "        sz")?;
        writeln!(out, "    }}")?;
        Ok(())
    }

    /// Get the statements for each field size calculation, and emit offset function
    /// each time we hit a field that requires an offset function.
    /// The resulting statements can be used to emit the compute_wire_len function
    pub(super) fn emit_compute_offset_and_get_stmts<O: Write>(
        &self,
        out: &mut O,
        _struct_rs_typ: &str,
        fields: &[Field],
        params_struct: Option<&ParamsStruct>,
    ) -> io::Result<Vec<String>> {
        let mut stmts = vec!["let mut sz = 0;".to_string()];

        for f in fields {
            if let Field::Field {
                name,
                need_compute_offset: true,
                ..
            }
            | Field::List {
                name,
                need_compute_offset: true,
                ..
            }
            | Field::Switch {
                name,
                need_compute_offset: true,
                ..
            } = f
            {
                let fname = format!("compute_{}_offset", name);
                self.emit_compute_func(out, &fname, params_struct, &stmts)?;
            }
            self.field_compute_len_stmts(f, &mut stmts, &Expr::Value(0));
        }

        Ok(stmts)
    }

    pub(super) fn emit_struct_accessors<O: Write>(
        &self,
        out: &mut O,
        struct_rs_typ: &str,
        fields: &[Field],
    ) -> io::Result<()> {
        for f in fields {
            writeln!(out)?;
            if self.handle_client_message_data(out, struct_rs_typ, f)? {
                continue;
            }
            if self.handle_randr_notify_data(out, struct_rs_typ, f)? {
                continue;
            }
            match f {
                Field::Field {
                    name,
                    rs_typ,
                    wire_off,
                    doc,
                    ..
                } if rs_typ == "bool" => {
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub fn {}(&self) -> bool {{", name)?;
                    writeln!(
                        out,
                        "        let val = unsafe {{ *(self.wire_ptr().add({})) }};",
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(out, "        val != 0")?;
                    writeln!(out, "    }}")?;
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    r#enum,
                    mask,
                    doc,
                    ..
                } if r#enum.is_some() || mask.is_some() => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    let ret_q_rs_typ = r#enum.as_ref().or_else(|| mask.as_ref()).unwrap();
                    let ret_q_rs_typ = {
                        let (module, rs_typ) = ret_q_rs_typ;
                        (module, rs_typ).qualified_rs_typ()
                    };
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub fn {}(&self) -> {} {{", name, ret_q_rs_typ)?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "            let offset = {};",
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "            let ptr = self.wire_ptr().add(offset) as *const {};",
                        q_rs_typ
                    )?;
                    writeln!(out, "            let val = *ptr as u32;")?;
                    writeln!(
                        out,
                        "            std::mem::transmute::<u32, {}>(val)",
                        ret_q_rs_typ,
                    )?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    is_xid: true,
                    is_union: true,
                    doc,
                    ..
                } => {
                    // XID unions constructed out of context require a `Unknown` variant
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    fn {}(&self) -> {} {{", name, q_rs_typ)?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "{}let offset = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let res_id = *(self.wire_ptr().add(offset) as *const u32);",
                        cg::ind(3)
                    )?;
                    writeln!(out, "{}{}::Unknown(res_id)", cg::ind(3), q_rs_typ)?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    is_fieldref,
                    is_copy: true,
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    // lenfield visibility is not necessary, lists are exposed as sliced
                    let visibility = if *is_fieldref { "" } else { "pub " };
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "{}{}fn {}(&self) -> {} {{",
                        cg::ind(1),
                        visibility,
                        name,
                        q_rs_typ
                    )?;
                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}let offset = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let ptr = self.wire_ptr().add(offset) as *const {};",
                        cg::ind(3),
                        q_rs_typ
                    )?;
                    writeln!(out, "            *ptr")?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    wire_sz,
                    is_copy: false,
                    is_union: false,
                    struct_style,
                    params_struct,
                    doc,
                    need_compute_offset,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();

                    let params_expr = self.build_params_expr(
                        params_struct.as_ref(),
                        module.as_ref().map(|s| s.as_str()),
                        "self.",
                        "()",
                    );
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    pub fn {}(&self) -> &{} {{", name, q_rs_typ)?;
                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    if *need_compute_offset {
                        writeln!(
                            out,
                            "{}let offset = Self::compute_{}_offset(self.wire_ptr(), {});",
                            cg::ind(3),
                            name,
                            params_expr
                        )?;
                    } else {
                        writeln!(
                            out,
                            "{}let offset = {};",
                            cg::ind(3),
                            self.build_rs_expr(wire_off, "self.", "()", fields)
                        )?;
                    }
                    if let Some(StructStyle::DynBuf) = struct_style {
                        writeln!(
                            out,
                            "{}let len = {}::compute_wire_len(self.wire_ptr().add(offset), {});",
                            cg::ind(3),
                            q_rs_typ,
                            params_expr
                        )?;
                    } else {
                        writeln!(
                            out,
                            "            let len = {};",
                            self.build_rs_expr(wire_sz, "self.", "()", fields)
                        )?;
                    }
                    writeln!(out, "            let data = std::slice::from_raw_parts(self.wire_ptr().add(offset), len);")?;
                    writeln!(out, "            {}::from_data(data)", q_rs_typ,)?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                Field::Field {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    wire_sz,
                    is_copy: false,
                    is_union: true,
                    need_compute_offset,
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(out, "    fn {}(&self) -> {} {{", name, q_rs_typ)?;
                    writeln!(out, "        unsafe {{")?;
                    if *need_compute_offset {
                        writeln!(
                            out,
                            "            let offset = Self::compute_{}_offset(self.wire_ptr());",
                            name
                        )?;
                    } else {
                        writeln!(
                            out,
                            "            let offset = {};",
                            self.build_rs_expr(wire_off, "self.", "()", fields)
                        )?;
                    }
                    writeln!(
                        out,
                        "{}let len = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_sz, "self.", "()", fields)
                    )?;
                    writeln!(out, "{}let data = std::slice::from_raw_parts(self.wire_ptr().add(offset), len);", cg::ind(3))?;
                    writeln!(out, "{}{}::from_data(data)", cg::ind(3), q_rs_typ)?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                Field::List {
                    name,
                    wire_off,
                    len_expr,
                    is_prop: true,
                    doc,
                    ..
                } => {
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "{}pub fn {}<P: PropEl>(&self) -> &[P] {{",
                        cg::ind(1),
                        name
                    )?;
                    writeln!(
                        out,
                        "{}assert_eq!(self.format(), P::FORMAT, \"mismatched format of {}::{}::{}\");",
                        cg::ind(2),
                        self.xcb_mod,
                        struct_rs_typ,
                        name
                    )?;
                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}let offset = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let len = {};",
                        cg::ind(3),
                        self.build_rs_expr(len_expr, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let len = len / std::mem::size_of::<P>();",
                        cg::ind(3)
                    )?;
                    writeln!(
                        out,
                        "{}let ptr = self.wire_ptr().add(offset) as *const P;",
                        cg::ind(3),
                    )?;
                    writeln!(out, "{}std::slice::from_raw_parts(ptr, len)", cg::ind(3))?;
                    writeln!(out, "{}}}", cg::ind(2))?;
                    writeln!(out, "{}}}", cg::ind(1))?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    len_expr,
                    struct_style: None,
                    doc,
                    ..
                } if rs_typ != "char" && !matches!(len_expr, Expr::Value(_)) => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    let params = len_expr.params_str();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "    pub fn {}(&self{}) -> &[{}] {{",
                        name, params, q_rs_typ
                    )?;
                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}let offset = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let len = {};",
                        cg::ind(3),
                        self.build_rs_expr(len_expr, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let ptr = self.wire_ptr().add(offset) as *const {};",
                        cg::ind(3),
                        q_rs_typ
                    )?;
                    writeln!(out, "{}std::slice::from_raw_parts(ptr, len)", cg::ind(3))?;
                    writeln!(out, "{}}}", cg::ind(2))?;
                    writeln!(out, "{}}}", cg::ind(1))?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    len_expr,
                    struct_style: None,
                    doc,
                    ..
                } if rs_typ != "char" => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    let len = len_expr.fixed_length().unwrap();
                    let params = len_expr.params_str();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "    pub fn {}(&self{}) -> &[{}; {}] {{",
                        name, params, q_rs_typ, len
                    )?;

                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "            let offset = {};",
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "            let ptr = self.wire_ptr().add(offset) as *const [{}; {}];",
                        q_rs_typ, len
                    )?;
                    writeln!(out, "            &*ptr")?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                Field::List {
                    name,
                    rs_typ,
                    wire_off,
                    len_expr,
                    doc,
                    ..
                } if rs_typ == "char" => {
                    let params = len_expr.params_str();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    // String returned from X may not be valid utf-8.
                    // To overcome this, we provide two methods:
                    //   - one that returns Result<&str, Utf8Error>
                    //   - one that returns &[u8]
                    // see rust-xcb#96
                    writeln!(
                        out,
                        "{}pub fn {}(&self{}) -> Result<&str, std::str::Utf8Error> {{",
                        cg::ind(1),
                        name,
                        params
                    )?;

                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}let offset = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let len = {} as _;",
                        cg::ind(3),
                        self.build_rs_expr(len_expr, "self.", "()", fields)
                    )?;
                    writeln!(out, "{}let ptr = self.wire_ptr().add(offset);", cg::ind(3))?;
                    writeln!(
                        out,
                        "{}let raw = std::slice::from_raw_parts(ptr, len);",
                        cg::ind(3)
                    )?;
                    writeln!(out, "{}std::str::from_utf8(raw)", cg::ind(3))?;
                    writeln!(out, "{}}}", cg::ind(2))?;
                    writeln!(out, "{}}}", cg::ind(1))?;

                    writeln!(out)?;
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "{}pub fn {}_raw(&self{}) -> &[u8] {{",
                        cg::ind(1),
                        name,
                        params
                    )?;

                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}let offset = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "{}let len = {} as _;",
                        cg::ind(3),
                        self.build_rs_expr(len_expr, "self.", "()", fields)
                    )?;
                    writeln!(out, "{}let ptr = self.wire_ptr().add(offset);", cg::ind(3))?;
                    writeln!(out, "{}std::slice::from_raw_parts(ptr, len)", cg::ind(3))?;
                    writeln!(out, "{}}}", cg::ind(2))?;
                    writeln!(out, "{}}}", cg::ind(1))?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    len_expr: Expr::UntilEnd,
                    struct_style: None | Some(StructStyle::WireLayout | StructStyle::FixBuf),
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();

                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "{}pub fn {}(&self) -> &[{}] {{",
                        cg::ind(1),
                        name,
                        q_rs_typ
                    )?;
                    writeln!(out, "{}unsafe {{", cg::ind(2))?;
                    writeln!(
                        out,
                        "{}let offset = {};",
                        cg::ind(3),
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(out, "{}assert_eq!((self.length() as usize - offset) % std::mem::size_of::<{}>(), 0);", cg::ind(3), q_rs_typ)?;
                    writeln!(out, "{}let len = (self.length() as usize - offset) / std::mem::size_of::<{}>();", cg::ind(3), q_rs_typ)?;
                    writeln!(out, "{}std::slice::from_raw_parts(self.wire_ptr().add(offset) as *const {}, len)", cg::ind(3), q_rs_typ)?;
                    writeln!(out, "{}}}", cg::ind(2))?;
                    writeln!(out, "{}}}", cg::ind(1))?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    len_expr,
                    struct_style: Some(StructStyle::WireLayout | StructStyle::FixBuf),
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    let params = len_expr.params_str();
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "    pub fn {}(&self{}) -> &[{}] {{",
                        name, params, q_rs_typ
                    )?;

                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "            let offset = {};",
                        self.build_rs_expr(wire_off, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "            let len = {} as _;",
                        self.build_rs_expr(len_expr, "self.", "()", fields)
                    )?;
                    writeln!(
                        out,
                        "            let ptr = self.wire_ptr().add(offset) as *const {};",
                        q_rs_typ
                    )?;
                    writeln!(out, "            std::slice::from_raw_parts(ptr, len)",)?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                Field::List {
                    name,
                    module,
                    rs_typ,
                    wire_off,
                    len_expr,
                    struct_style: Some(StructStyle::DynBuf),
                    params_struct,
                    need_compute_offset,
                    doc,
                    ..
                } => {
                    let q_rs_typ = (module, rs_typ).qualified_rs_typ();
                    let params = len_expr.params_str();
                    let params_expr = self.build_params_expr(
                        params_struct.as_ref(),
                        module.as_ref().map(|s| s.as_str()),
                        "self.",
                        "()",
                    );
                    if let Some(doc) = doc {
                        doc.emit(out, 1)?;
                    }
                    writeln!(
                        out,
                        "    pub fn {}(&self{}) -> {}Iterator {{",
                        name, params, q_rs_typ
                    )?;
                    writeln!(out, "        unsafe {{")?;
                    if *need_compute_offset {
                        writeln!(
                            out,
                            "{}let offset = Self::compute_{}_offset(self.wire_ptr(), {});",
                            cg::ind(3),
                            name,
                            params_expr
                        )?;
                    } else {
                        writeln!(
                            out,
                            "{}let offset = {};",
                            cg::ind(3),
                            self.build_rs_expr(wire_off, "self.", "()", fields)
                        )?;
                    }
                    writeln!(out, "{}{}Iterator {{", cg::ind(3), q_rs_typ)?;
                    writeln!(out, "{}    params: {},", cg::ind(3), params_expr)?;
                    writeln!(
                        out,
                        "{}    rem: {},",
                        cg::ind(3),
                        self.build_rs_expr(len_expr, "self.", "()", fields)
                    )?;
                    writeln!(out, "{}    ptr: self.wire_ptr().add(offset),", cg::ind(3))?;
                    writeln!(out, "{}    phantom: std::marker::PhantomData,", cg::ind(3))?;
                    writeln!(out, "{}}}", cg::ind(3))?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    // specific handling of ClientMessageEvent::data()
    // This is nearly static function definition. We could possibly directly define that in the
    // xproto module in lib.rs
    fn handle_client_message_data<O: Write>(
        &self,
        out: &mut O,
        struct_rs_typ: &str,
        field: &Field,
    ) -> io::Result<bool> {
        if self.xcb_mod == "xproto"
            && struct_rs_typ == "ClientMessageEvent"
            && matches!(field, Field::Field{name, ..} if name == "data")
        {
            writeln!(out)?;
            writeln!(
                out,
                "{}pub fn data(&self) -> ClientMessageData {{",
                cg::ind(1)
            )?;
            writeln!(out, "{}unsafe {{", cg::ind(2))?;
            writeln!(out, "{}match self.format() {{", cg::ind(3))?;
            for sz in [8, 16, 32] {
                writeln!(
                    out,
                    "{}{} => ClientMessageData::Data{} (",
                    cg::ind(4),
                    sz,
                    sz
                )?;
                writeln!(out, "{}std::slice::from_raw_parts(", cg::ind(5))?;
                writeln!(
                    out,
                    "{}self.wire_ptr().add(12) as *const u{}, {}",
                    cg::ind(6),
                    sz,
                    (20 * 8) / sz
                )?;
                writeln!(out, "{}).try_into().unwrap()", cg::ind(5))?;
                writeln!(out, "{}),", cg::ind(4))?;
            }
            writeln!(
                out,
                "{}format => unreachable!(\"invalid ClientMessageEvent format: {{}}\", format),",
                cg::ind(4)
            )?;
            writeln!(out, "{}}}", cg::ind(3))?;
            writeln!(out, "{}}}", cg::ind(2))?;
            writeln!(out, "{}}}", cg::ind(1))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn handle_randr_notify_data<O: Write>(
        &self,
        out: &mut O,
        struct_rs_typ: &str,
        field: &Field,
    ) -> io::Result<bool> {
        if self.xcb_mod == "randr"
            && struct_rs_typ == "NotifyEvent"
            && matches!(field, Field::Field{name, ..} if name == "u")
        {
            writeln!(out)?;
            writeln!(out, "{}pub fn u(&self) -> NotifyData {{", cg::ind(1))?;
            writeln!(out, "{}unsafe {{", cg::ind(2))?;
            writeln!(out, "{}match self.sub_code() {{", cg::ind(3))?;

            for code in RANDR_SUBCODES {
                writeln!(
                    out,
                    "{}Notify::{} => NotifyData::{}(",
                    cg::ind(4),
                    code.0,
                    code.2
                )?;
                writeln!(out, "{}*{}::from_data(", cg::ind(5), code.1)?;
                writeln!(
                    out,
                    "{}std::slice::from_raw_parts(self.wire_ptr().add(4), 28)",
                    cg::ind(6)
                )?;
                writeln!(out, "{})", cg::ind(5))?;
                writeln!(out, "{}),", cg::ind(4))?;
            }

            writeln!(out, "{}}}", cg::ind(3))?;
            writeln!(out, "{}}}", cg::ind(2))?;
            writeln!(out, "{}}}", cg::ind(1))?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(super) fn emit_debug_impl<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        fields: &[Field],
    ) -> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "impl std::fmt::Debug for {} {{", rs_typ)?;
        writeln!(
            out,
            "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
        )?;
        writeln!(out, "        f.debug_struct(\"{}\")", rs_typ)?;
        for f in fields {
            match f {
                Field::Field { name, .. } => {
                    writeln!(out, "{}.field(\"{}\", &self.{}())", cg::ind(3), name, name)?;
                }
                Field::List {
                    name,
                    is_prop: true,
                    ..
                } => {
                    writeln!(
                        out,
                        "{}.field(\"{}\", &self.{}::<u8>())",
                        cg::ind(3),
                        name,
                        name
                    )?;
                }
                Field::List { name, wire_sz, .. } if wire_sz.params().is_empty() => {
                    writeln!(out, "{}.field(\"{}\", &self.{}())", cg::ind(3), name, name)?;
                }
                Field::List { .. } => {
                    // TODO
                }
                Field::Switch { .. } => {
                    // TODO
                }
                Field::Pad {
                    wire_sz: Expr::Value(sz),
                    ..
                } => {
                    writeln!(out, "{}.field(\"pad\", &{})", cg::ind(3), sz)?;
                }
                Field::AlignPad {
                    wire_sz: Expr::AlignPad(sz, _),
                    ..
                } => {
                    writeln!(out, "{}.field(\"align_pad\", &{})", cg::ind(3), sz)?;
                }
                _ => {}
            }
        }
        writeln!(out, "            .finish()")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    fn emit_dyn_buf_struct_iterator<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        params_struct: Option<&ParamsStruct>,
    ) -> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "#[derive(Clone)]")?;
        writeln!(out, "pub struct {}Iterator<'a> {{", rs_typ)?;
        writeln!(out, "    pub(crate) params: {},", params_struct.rs_typ())?;
        writeln!(out, "    pub(crate) rem: usize,")?;
        writeln!(out, "    pub(crate) ptr: *const u8,")?;
        writeln!(
            out,
            "    pub(crate) phantom: std::marker::PhantomData<&'a {}>,",
            rs_typ
        )?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(out, "impl<'a> Iterator for {}Iterator<'a> {{", rs_typ)?;
        writeln!(out, "    type Item = &'a {};", rs_typ)?;
        writeln!(out)?;
        writeln!(
            out,
            "    fn next(&mut self) -> std::option::Option<Self::Item> {{"
        )?;
        writeln!(out, "        if self.rem == 0 {{")?;
        writeln!(out, "            None")?;
        writeln!(out, "        }} else {{ unsafe {{")?;
        writeln!(out, "            self.rem -= 1;")?;
        writeln!(
            out,
            "            let len = {}::compute_wire_len(self.ptr, self.params);",
            rs_typ
        )?;
        writeln!(
            out,
            "            let res = {}::from_data(std::slice::from_raw_parts(self.ptr, len));",
            rs_typ
        )?;
        writeln!(out, "            self.ptr = self.ptr.add(len);")?;
        writeln!(out, "            Some(res)")?;
        writeln!(out, "        }}}}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        writeln!(out)?;
        writeln!(
            out,
            "impl<'a> std::fmt::Debug for {}Iterator<'a> {{",
            rs_typ
        )?;
        writeln!(
            out,
            "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"
        )?;
        writeln!(out, "        f.debug_list().entries(self.clone()).finish()")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        Ok(())
    }
}

struct FieldInfo {
    name: String,
    module: Option<String>,
    typ: String,
    rs_typ: String,
    wire_sz: Expr,
    has_wire_layout: bool,
    struct_style: Option<StructStyle>,
    params_struct: Option<ParamsStruct>,
    doc: Option<DocField>,
    is_union: bool,
    is_xid: bool,
    is_mask: bool,
}

pub(super) const RANDR_SUBCODES: &[(&str, &str, &str)] = &[
    ("CrtcChange", "CrtcChange", "Cc"),
    ("OutputChange", "OutputChange", "Oc"),
    ("OutputProperty", "OutputProperty", "Op"),
    ("ProviderChange", "ProviderChange", "Pc"),
    ("ProviderProperty", "ProviderProperty", "Pp"),
    ("ResourceChange", "ResourceChange", "Rc"),
    ("Lease", "LeaseNotify", "Lc"),
];

pub(super) fn enum_mask_qualified_rs_typ(
    module: &Option<String>,
    rs_typ: &str,
    r#enum: &Option<(Option<String>, String)>,
    mask: &Option<(Option<String>, String)>,
) -> String {
    let mod_rs_typ = r#enum
        .as_ref()
        .or_else(|| mask.as_ref())
        .map(|(m, t)| (m.as_ref().map(|m| m.as_str()), t.as_str()))
        .or_else(|| Some((module.as_ref().map(|m| m.as_str()), rs_typ)))
        .unwrap();
    mod_rs_typ.qualified_rs_typ()
}

pub(super) fn make_field(name: String, typ: String) -> ir::Field {
    ir::Field::Field {
        name,
        typ,
        r#enum: None,
        mask: None,
        altenum: None,
        altmask: None,
    }
}
