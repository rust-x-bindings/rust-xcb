use std::io::{self, Write};

use super::{
    capitalize, emit_doc_field, emit_doc_text, enum_suffix_exception, expr_fixed_length,
    extract_module, field_name, has_fd, make_field, tit_dig_split, CodeGen,
};
use crate::ast::{Doc, Expr, Reply, Struct, StructField, SwitchCase};

impl CodeGen {
    fn has_ffi_type(&self, typ: &str) -> bool {
        self.ffi_typ_reg.contains(typ)
    }

    /// FFI type name
    pub fn ffi_decl_type_name(&self, typ: &str) -> String {
        let typ = tit_dig_split(typ).to_ascii_lowercase();
        format!("xcb_{}{}_t", &self.xcb_mod_prefix, typ)
    }

    /// same as ffi_decl_type_name but can also have a namespace before (with a single colon)
    pub fn ffi_use_type_name(&self, typ: &str) -> String {
        match typ {
            "CARD8" => "u8".into(),
            "CARD16" => "u16".into(),
            "CARD32" => "u32".into(),
            "CARD64" => "u64".into(),
            "INT8" => "i8".into(),
            "INT16" => "i16".into(),
            "INT32" => "i32".into(),
            "BYTE" => "u8".into(),
            "BOOL" => "u8".into(),
            "char" => "c_char".into(),
            "float" => "f32".into(),
            "double" => "f64".into(),
            "void" => "c_void".into(),
            typ => {
                let (module, typ) = extract_module(typ);

                if let Some(module) = module {
                    let typ = tit_dig_split(typ).to_ascii_lowercase();
                    let mod_prefix = if module == "xproto" {
                        String::new()
                    } else {
                        format!("{}_", module)
                    };
                    format!("xcb_{}{}_t", &mod_prefix, typ)
                } else {
                    let mod_prefix = if self.has_type(typ) {
                        &self.xcb_mod_prefix
                    } else {
                        let mut pref = "";

                        for di in self.dep_info.iter() {
                            if di.has_type(typ) {
                                pref = &di.xcb_mod_prefix;
                                break;
                            }
                        }

                        pref
                    };
                    let typ = tit_dig_split(typ).to_ascii_lowercase();
                    format!("xcb_{}{}_t", mod_prefix, typ)
                }
            }
        }
    }

    pub fn ffi_iterator_name(&self, typ: &str) -> String {
        let mod_prefix = if self.has_type(typ) {
            &self.xcb_mod_prefix
        } else {
            let mut pref = "";

            for di in self.dep_info.iter() {
                if di.has_type(typ) {
                    pref = &di.xcb_mod_prefix;
                    break;
                }
            }

            pref
        };
        format!(
            "xcb_{}{}_iterator_t",
            &mod_prefix,
            tit_dig_split(typ).to_ascii_lowercase()
        )
    }

    pub fn ffi_type_sizeof(&self, typ: &str) -> Option<usize> {
        match typ {
            "CARD8" => Some(1),
            "CARD16" => Some(2),
            "CARD32" => Some(4),
            "CARD64" => Some(8),
            "INT8" => Some(1),
            "INT16" => Some(2),
            "INT32" => Some(4),
            "BYTE" => Some(1),
            "BOOL" => Some(1),
            "char" => Some(1),
            "float" => Some(4),
            "double" => Some(8),
            "void" => Some(0),
            typ => {
                // FIXME: handle module
                let (_, typ) = extract_module(typ);

                if let Some(sz) = self.ffi_type_sizes.get(typ) {
                    *sz
                } else {
                    // checking in the imported dependencies
                    for di in self.dep_info.iter() {
                        if let Some(sz) = di.ffi_type_sizes.get(typ) {
                            return *sz;
                        }
                    }
                    None
                }
            }
        }
    }

    pub fn ffi_enum_type_name(&mut self, typ: &str) -> String {
        let base = tit_dig_split(typ).to_ascii_lowercase();
        let try1 = format!("xcb_{}{}_t", self.xcb_mod_prefix, base);
        if self.has_ffi_type(&try1) || enum_suffix_exception(&self.xcb_mod, typ) {
            format!("xcb_{}{}_enum_t", self.xcb_mod_prefix, base)
        } else {
            try1
        }
    }

    pub fn compute_ffi_struct_field_sizeof(&self, field: &StructField) -> Option<usize> {
        match field {
            StructField::Field { typ, .. } => self.ffi_type_sizeof(typ),
            StructField::Pad(_, pad_sz) => Some(*pad_sz),
            StructField::List { typ, len_expr, .. } => {
                match (self.ffi_type_sizeof(typ), expr_fixed_length(len_expr)) {
                    (Some(sz), Some(len)) => Some(sz * len),
                    _ => None,
                }
            }
            StructField::ListNoLen { .. } => None,
            f => unimplemented!("{:?}", f),
        }
    }

    pub fn compute_ffi_struct_size(&self, stru: &Struct) -> Option<usize> {
        let mut stru_sz = Some(0usize);

        for f in stru.fields.iter() {
            match f {
                StructField::AlignPad(_, alignment) => match stru_sz.as_mut() {
                    Some(sz) => {
                        let curr = *sz % alignment;
                        if curr != 0 {
                            *sz += alignment - curr;
                        }
                    }
                    _ => {
                        return None;
                    }
                },
                field => {
                    match (
                        stru_sz.as_mut(),
                        self.compute_ffi_struct_field_sizeof(field),
                    ) {
                        (Some(stru_sz), Some(f_sz)) => {
                            *stru_sz += f_sz;
                        }
                        _ => {
                            return None;
                        }
                    }
                }
            }
        }

        stru_sz
    }

    pub fn compute_ffi_union_size(&self, stru: &Struct) -> usize {
        let mut biggest = 1;
        let mut alignment = 1;

        for f in stru.fields.iter() {
            let mut fs = self.ptr_width;
            let mut fa = self.ptr_width;
            match f {
                StructField::AlignPad(_, _) => panic!("Unexpected align pad in union"),
                StructField::Pad(_, _) => panic!("Unexpected pad in union"),
                StructField::Field { typ, .. } => {
                    if let Some(sz) = self.ffi_type_sizeof(typ) {
                        fs = sz;
                        fa = sz;
                    }
                }
                StructField::List { typ, len_expr, .. } => {
                    if let Some(sz) = self.ffi_type_sizeof(typ) {
                        fs = sz;
                        fa = sz;
                    }
                    if let Some(len) = expr_fixed_length(len_expr) {
                        fs = len * fa;
                    }
                }
                StructField::ListNoLen { .. } => panic!("Unexpected list without length in union"),

                f => unimplemented!("{:?}", f),
            }

            biggest = biggest.max(fs);
            alignment = alignment.max(fa);
        }

        let mut num_aligned = biggest / alignment;
        if biggest % alignment > 0 {
            num_aligned += 1;
        }
        num_aligned * alignment
    }

    pub fn emit_ffi_iterator(
        &mut self,
        name: &str,
        typ: &str,
        has_lifetime: bool,
    ) -> io::Result<String> {
        let it_typ = self.ffi_iterator_name(name);
        let it_next = iterator_next_fn_name(&self.xcb_mod_prefix, name);
        let it_end = iterator_end_fn_name(&self.xcb_mod_prefix, name);

        let out = &mut self.ffi;

        let lifetime = if has_lifetime { "<'a>" } else { "" };

        writeln!(out)?;
        writeln!(out, "#[repr(C)]")?;
        writeln!(out, "#[derive(Debug)]")?;
        writeln!(out, "pub struct {}{} {{", &it_typ, lifetime)?;
        writeln!(out, "    pub data:  *mut {},", &typ)?;
        writeln!(out, "    pub rem:   c_int,")?;
        writeln!(out, "    pub index: c_int,")?;
        if has_lifetime {
            writeln!(out, "    _phantom: std::marker::PhantomData<&'a {}>,", &typ)?;
        }
        writeln!(out, "}}")?;

        let out = &mut self.ffi_buf;
        writeln!(out).unwrap();
        writeln!(out, "pub fn {}(i: *mut {});", &it_next, &it_typ).unwrap();
        writeln!(out).unwrap();
        writeln!(
            out,
            "pub fn {}(i: *mut {}) -> xcb_generic_iterator_t;",
            &it_end, &it_typ
        )
        .unwrap();
        Ok(it_typ)
    }

    fn emit_ffi_field_list_accessor(
        &mut self,
        ffi_typ: &str,
        xcb_name: &str,
        fname: &str,
        ftyp: &str,
        toplevel_typ: Option<&str>,
        fixed_size: bool,
    ) -> io::Result<()> {
        let is_simple = self.typ_is_simple(ftyp);

        let accessor_needed = fixed_size;
        let length_needed = true;
        let end_needed = is_simple;
        let iterator_needed = !is_simple;

        let has_lifetime = self.type_has_lifetime(ftyp);

        let args = if let Some(toplevel_typ) = toplevel_typ {
            format!(
                "R: *const {}, S: *const {}",
                self.ffi_use_type_name(toplevel_typ),
                ffi_typ
            )
        } else {
            format!("R: *const {}", ffi_typ)
        };

        if accessor_needed {
            let ftyp = self.ffi_use_type_name(ftyp);
            let acc_fn = field_list_iterator_acc_fn_name(&self.xcb_mod_prefix, xcb_name, fname);
            // the following only for diff equality with Python code
            let param = if toplevel_typ.is_some() { "S" } else { "R" };
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub fn {}({}: *const {}) -> *mut {};",
                &acc_fn, param, &ffi_typ, &ftyp
            )?;
        }

        if length_needed {
            let len_fn = field_list_iterator_len_fn_name(&self.xcb_mod_prefix, xcb_name, fname);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(out, "pub fn {}({}) -> c_int;", &len_fn, &args)?;
        }

        if end_needed {
            let end_fn = field_list_iterator_end_fn_name(&self.xcb_mod_prefix, xcb_name, fname);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub fn {}({}) -> xcb_generic_iterator_t;",
                &end_fn, &args
            )?;
        }

        if iterator_needed {
            let lifetime = if has_lifetime { "<'a>" } else { "" };
            let it_fn = field_list_iterator_it_fn_name(&self.xcb_mod_prefix, xcb_name, fname);
            let it_typ = self.ffi_iterator_name(ftyp);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub fn {}{}({}) -> {}{};",
                &it_fn, &lifetime, &args, &it_typ, &lifetime
            )?;
        }
        Ok(())
    }

    pub fn emit_ffi_field_list_accessors(
        &mut self,
        ffi_typ: &str,
        xcb_name: &str,
        fields: &[StructField],
        toplevel_typ: Option<&str>,
        is_switch: bool,
    ) -> io::Result<()> {
        for f in fields {
            match f {
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    let fixed_size = self.ffi_type_sizeof(typ).is_some();
                    let fixed_len = expr_fixed_length(len_expr).is_some();

                    if fixed_size && fixed_len {
                        continue;
                    }

                    self.emit_ffi_field_list_accessor(
                        ffi_typ,
                        xcb_name,
                        name,
                        typ,
                        toplevel_typ,
                        fixed_size,
                    )?;
                }
                StructField::ValueParam { list_name, .. } => {
                    self.emit_ffi_field_list_accessor(
                        ffi_typ,
                        xcb_name,
                        list_name,
                        "CARD32",
                        toplevel_typ,
                        true,
                    )?;
                }
                StructField::Field { name, typ, .. } => {
                    if is_switch && !self.typ_is_pod(typ) {
                        let fn_name = switch_accessor_fn(&self.xcb_mod_prefix, xcb_name, name);
                        let ret = self.ffi_use_type_name(typ);
                        let out = &mut self.ffi_buf;
                        writeln!(
                            out,
                            "pub fn {}(R: *const {}) -> *mut {};",
                            &fn_name, ffi_typ, ret,
                        )?;
                    }
                }
                StructField::Switch(name, ..) => {
                    let fn_name = switch_accessor_fn(&self.xcb_mod_prefix, xcb_name, name);
                    let ret = if is_switch {
                        let typ = xcb_name.to_string() + &capitalize(name);
                        self.notify_typ(typ.to_string());
                        self.ffi_use_type_name(&typ)
                    } else {
                        "c_void".to_string()
                    };

                    let out = &mut self.ffi_buf;

                    writeln!(
                        out,
                        "pub fn {}(R: *const {}) -> *mut {};",
                        &fn_name, ffi_typ, ret,
                    )?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn emit_ffi_struct(
        &mut self,
        stru: &Struct,
        case_req_name: Option<&str>,
        must_pack: bool,
        no_copy: bool,
        is_switch: bool,
    ) -> io::Result<String> {
        let Struct { name, fields, doc } = &stru;

        let ffi_typ = if name.starts_with('_') {
            name.clone()
        } else {
            self.ffi_decl_type_name(name)
        };

        let impl_copy_clone = self.eligible_to_copy(stru) && !no_copy;

        let copyclone = if impl_copy_clone { "Copy, Clone, " } else { "" };

        {
            let must_pack = if must_pack { ", packed" } else { "" };

            let out = &mut self.ffi;
            writeln!(out)?;
            emit_doc_text(out, doc)?;
            writeln!(out, "#[derive({}Debug)]", copyclone)?;
            writeln!(out, "#[repr(C{})]", must_pack)?;
            writeln!(out, "pub struct {} {{", &ffi_typ)?;
        }

        // cases of ValueParam were the mask is declared as a field in the struct
        // before the actual ValueParam field. We keep track of all fields written
        // to avoid doubles
        let mut written_fields = Vec::new();

        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } | StructField::Expr { name, typ, .. } => {
                    let ptr = if self.ffi_type_sizeof(typ).is_some() {
                        ""
                    } else {
                        "*mut "
                    };
                    //let ptr = if self.typ_is_pod(&typ) { "" } else {"*mut "};
                    let typ = self.ffi_use_type_name(typ);
                    let out = &mut self.ffi;
                    emit_doc_field(out, doc, name)?;
                    writeln!(out, "    pub {}: {}{},", field_name(name), ptr, typ,)?;
                    written_fields.push(name.as_str());
                }
                StructField::Pad(name, sz) => {
                    let out = &mut self.ffi;
                    let padtyp = match sz {
                        1 => "u8".into(),
                        x => format!("[u8; {}]", x),
                    };
                    writeln!(out, "    pub {}: {},", name, padtyp)?;
                }
                StructField::AlignPad(name, _) => {
                    let out = &mut self.ffi;
                    if is_switch {
                        writeln!(out, "    pub {}: *mut u8,", name)?;
                    }
                }
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    if let Some(sz) = expr_fixed_length(len_expr) {
                        let typ = self.ffi_use_type_name(typ);
                        let out = &mut self.ffi;

                        emit_doc_field(out, doc, name)?;
                        writeln!(out, "    pub {}: [{}; {}],", field_name(name), &typ, sz)?;
                    } else if is_switch {
                        let typ = self.ffi_use_type_name(typ);
                        let out = &mut self.ffi;

                        emit_doc_field(out, doc, name)?;
                        writeln!(out, "    pub {}: *mut {},", field_name(name), &typ)?;
                    }
                }
                StructField::ValueParam {
                    mask_name,
                    mask_typ,
                    ..
                } => {
                    if written_fields.contains(&mask_name.as_str()) {
                        continue;
                    }
                    let mask_typ = self.ffi_use_type_name(mask_typ);
                    let out = &mut self.ffi;
                    emit_doc_field(out, doc, mask_name)?;
                    writeln!(out, "    pub {}: {},", field_name(mask_name), mask_typ,)?;
                }
                StructField::Switch(name, _, _) => {
                    if let Some(case_req_name) = case_req_name {
                        let typ = switch_struct_name(&self.xcb_mod_prefix, case_req_name, name);
                        let out = &mut self.ffi;
                        writeln!(out, "    pub {}: {},", field_name(name), typ)?;
                    }
                }
                StructField::NamedCase(name, typ) => {
                    let out = &mut self.ffi;
                    writeln!(out, "    pub {}: {},", field_name(name), typ)?;
                }
                _ => {}
            }
        }

        let out = &mut self.ffi;
        writeln!(out, "}}")?;

        Ok(ffi_typ)
    }

    pub fn emit_ffi_switch_struct(
        &mut self,
        typ_name: &str,
        switch_name: &str,
        _expr: &Expr<usize>,
        cases: &[SwitchCase],
        toplevel_typ: &str,
        parent_switch: Option<&str>,
    ) -> io::Result<String> {
        let fields = {
            let mut fields = Vec::new();
            for c in cases.iter() {
                if let Some(name) = &c.name {
                    let typ = switch_named_case(&self.xcb_mod_prefix, typ_name, switch_name, name);
                    fields.push(StructField::NamedCase(name.clone(), typ));
                } else {
                    fields.append(&mut c.fields.clone());
                }
            }
            fields
        };

        let stru_name = typ_name.to_string() + &capitalize(switch_name);

        let stru = Struct {
            name: stru_name.clone(),
            fields,
            doc: None,
        };
        let ffi_typ = self.emit_ffi_struct(&stru, None, false, true, true)?;

        let ffi_typ = parent_switch.unwrap_or(&ffi_typ);

        self.emit_ffi_field_list_accessors(
            ffi_typ,
            &stru_name,
            &stru.fields,
            Some(toplevel_typ),
            true,
        )?;

        for c in cases.iter() {
            if let Some(name) = &c.name {
                let typ = switch_named_case(&self.xcb_mod_prefix, typ_name, switch_name, name);
                let stru = Struct {
                    name: typ.clone(),
                    fields: c.fields.clone(),
                    doc: None,
                };
                let case_req_name = stru_name.clone() + &capitalize(name);
                self.emit_ffi_struct(&stru, Some(&case_req_name), false, true, true)?;

                self.emit_ffi_field_list_accessors(
                    ffi_typ,
                    &case_req_name,
                    &stru.fields,
                    Some(toplevel_typ),
                    true,
                )?;
            }
        }
        for c in cases.iter() {
            for f in c.fields.iter() {
                if let StructField::Switch(cname, cexpr, ccases) = f {
                    let stru_name = if let Some(name) = &c.name {
                        stru_name.clone() + &capitalize(name)
                    } else {
                        stru_name.clone()
                    };
                    self.emit_ffi_switch_struct(
                        &stru_name,
                        cname,
                        cexpr,
                        ccases,
                        toplevel_typ,
                        Some(ffi_typ),
                    )?;
                }
            }
        }
        Ok(ffi_typ.to_string())
    }

    pub fn emit_ffi_req_fn(
        &mut self,
        req_name: &str,
        fn_name: &str,
        cookie_name: &str,
        fields: &[StructField],
        doc: &Option<Doc>,
    ) -> io::Result<()> {
        let cookie_typ = self.ffi_use_type_name(cookie_name);
        {
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            emit_doc_text(out, doc)?;
            writeln!(out, "    pub fn {} (", &fn_name)?;
            writeln!(out, "        c: *mut xcb_connection_t,")?;
        }
        let mut written_fields = Vec::new();
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    written_fields.push(name);
                    let name = field_name(name);
                    let typ = self.ffi_use_type_name(typ);
                    writeln!(&mut self.ffi_buf, "        {}: {},", &name, &typ)?;
                }
                StructField::Fd(name) => {
                    let name = field_name(name);
                    writeln!(&mut self.ffi_buf, "        {}: i32,", &name)?;
                }
                StructField::ValueParam {
                    mask_typ,
                    mask_name,
                    list_name,
                } => {
                    let mask_typ = self.ffi_use_type_name(mask_typ);
                    let list_name = field_name(list_name);

                    let out = &mut self.ffi_buf;
                    if !written_fields.contains(&mask_name) {
                        let mask_name = field_name(mask_name);
                        writeln!(out, "        {}: {},", &mask_name, &mask_typ)?;
                    }
                    writeln!(out, "        {}: *const u32,", &list_name)?;
                }
                StructField::List { name, typ, .. } => {
                    let name = field_name(name);
                    let typ = self.ffi_use_type_name(typ);
                    let out = &mut self.ffi_buf;
                    writeln!(out, "        {}: *const {},", &name, &typ)?;
                }
                StructField::ListNoLen { name, typ } => {
                    let len_name = name.clone() + "_len";
                    let name = field_name(name);
                    let typ = self.ffi_use_type_name(typ);
                    let out = &mut self.ffi_buf;
                    writeln!(out, "        {}: u32,", &len_name)?;
                    writeln!(out, "        {}: *const {},", &name, &typ)?;
                }
                StructField::Switch(name, ..) => {
                    let name = field_name(name);
                    let typ = switch_struct_name(&self.xcb_mod_prefix, req_name, &name);
                    writeln!(&mut self.ffi_buf, "        {}: *const {},", &name, &typ)?;
                }
                _ => {}
            }
        }

        let out = &mut self.ffi_buf;
        writeln!(out, "    ) -> {};", &cookie_typ)?;

        Ok(())
    }

    pub fn emit_ffi_reply(
        &mut self,
        req_name: &str,
        mut reply: Reply,
    ) -> io::Result<(String, String, String)> {
        // writting cookie struct
        let cookie_name = req_name.to_string() + "Cookie";
        let cookie_ffi_typ = self.ffi_decl_type_name(&cookie_name);
        {
            let out = &mut self.ffi;
            writeln!(out)?;
            writeln!(out, "#[derive(Copy, Clone, Debug)]")?;
            writeln!(out, "#[repr(C)]")?;
            writeln!(out, "pub struct {} {{", &cookie_ffi_typ)?;
            writeln!(out, "    pub(crate) sequence: c_uint,")?;
            writeln!(out, "}}")?;
        }

        for f in &reply.fields {
            if let StructField::Switch(name, expr, cases) = f {
                let toplevel = req_name.to_string() + "Reply";
                self.notify_typ(toplevel.clone());
                self.emit_ffi_switch_struct(req_name, name, expr, cases, &toplevel, None)?;
            }
        }

        let reply_fields = {
            let mut fields = vec![
                make_field("response_type".into(), "CARD8".into()),
                if reply.fields.is_empty() {
                    StructField::Pad("pad0".into(), 1usize)
                } else {
                    reply.fields.remove(0)
                },
                make_field("sequence".into(), "CARD16".into()),
                make_field("length".into(), "CARD32".into()),
            ];
            fields.append(&mut reply.fields);
            fields
        };
        let reply = Struct {
            name: req_name.to_string() + "Reply",
            fields: reply_fields,
            doc: reply.doc,
        };

        let ffi_reply_typ = self.emit_ffi_struct(&reply, None, false, false, false)?;

        self.emit_ffi_field_list_accessors(&ffi_reply_typ, req_name, &reply.fields, None, false)?;

        let ffi_reply_fn = reply_fn_name(&self.xcb_mod_prefix, req_name);
        {
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(
                out,
                "    /// the returned value must be freed by the caller using libc::free()."
            )?;
            writeln!(out, "    pub fn {} (", &ffi_reply_fn)?;
            writeln!(out, "        c: *mut xcb_connection_t,")?;
            writeln!(out, "        cookie: {},", &cookie_ffi_typ)?;
            writeln!(out, "        error: *mut *mut xcb_generic_error_t,")?;
            writeln!(out, "    ) -> *mut {};", &ffi_reply_typ)?;
        }

        if has_fd(&reply.fields) {
            let fds_fn = reply_fds_fn_name(&self.xcb_mod_prefix, req_name);
            let out = &mut self.ffi_buf;
            writeln!(out)?;
            writeln!(out, "    pub fn {}(", &fds_fn)?;
            writeln!(out, "        c: *mut xcb_connection_t,")?;
            writeln!(out, "        reply: *mut {},", &ffi_reply_typ)?;
            writeln!(out, "    ) -> *mut c_int;")?;
        }

        Ok((cookie_ffi_typ, ffi_reply_fn, ffi_reply_typ))
    }
}

pub fn enum_item_name(xcb_mod_prefix: &str, name: &str, item: &str) -> String {
    format!(
        "XCB_{}{}_{}",
        xcb_mod_prefix,
        tit_dig_split(name),
        tit_dig_split(item)
    )
    .to_ascii_uppercase()
}

pub fn iterator_next_fn_name(xcb_mod_prefix: &str, typ: &str) -> String {
    format!(
        "xcb_{}{}_next",
        xcb_mod_prefix,
        tit_dig_split(typ).to_ascii_lowercase()
    )
}

pub fn iterator_end_fn_name(xcb_mod_prefix: &str, typ: &str) -> String {
    format!(
        "xcb_{}{}_end",
        xcb_mod_prefix,
        tit_dig_split(typ).to_ascii_lowercase()
    )
}

pub fn field_list_iterator_acc_fn_name(
    xcb_mod_prefix: &str,
    typ_name: &str,
    field: &str,
) -> String {
    format!(
        "xcb_{}{}_{}",
        &xcb_mod_prefix,
        tit_dig_split(typ_name).to_ascii_lowercase(),
        tit_dig_split(field).to_ascii_lowercase()
    )
}

pub fn field_list_iterator_len_fn_name(
    xcb_mod_prefix: &str,
    typ_name: &str,
    field: &str,
) -> String {
    format!(
        "xcb_{}{}_{}_length",
        &xcb_mod_prefix,
        tit_dig_split(typ_name).to_ascii_lowercase(),
        tit_dig_split(field).to_ascii_lowercase()
    )
}

pub fn field_list_iterator_end_fn_name(
    xcb_mod_prefix: &str,
    typ_name: &str,
    field: &str,
) -> String {
    format!(
        "xcb_{}{}_{}_end",
        &xcb_mod_prefix,
        tit_dig_split(typ_name).to_ascii_lowercase(),
        tit_dig_split(field).to_ascii_lowercase()
    )
}

pub fn field_list_iterator_it_fn_name(xcb_mod_prefix: &str, typ_name: &str, field: &str) -> String {
    format!(
        "xcb_{}{}_{}_iterator",
        &xcb_mod_prefix,
        tit_dig_split(typ_name).to_ascii_lowercase(),
        tit_dig_split(field).to_ascii_lowercase()
    )
}

pub fn switch_struct_name(xcb_mod_prefix: &str, req_name: &str, switch_name: &str) -> String {
    format!(
        "xcb_{}{}_{}_t",
        &xcb_mod_prefix,
        tit_dig_split(req_name).to_ascii_lowercase(),
        tit_dig_split(switch_name).to_ascii_lowercase()
    )
}

pub fn switch_accessor_fn(xcb_mod_prefix: &str, req_name: &str, switch_name: &str) -> String {
    format!(
        "xcb_{}{}_{}",
        &xcb_mod_prefix,
        tit_dig_split(req_name).to_ascii_lowercase(),
        tit_dig_split(switch_name).to_ascii_lowercase()
    )
}

pub fn switch_named_case(
    xcb_mod_prefix: &str,
    req_name: &str,
    switch_name: &str,
    case_name: &str,
) -> String {
    format!(
        "_xcb_{}{}_{}__{}",
        &xcb_mod_prefix,
        tit_dig_split(req_name).to_ascii_lowercase(),
        tit_dig_split(switch_name),
        tit_dig_split(case_name)
    )
}

pub fn request_fn_name(xcb_mod_prefix: &str, req_name: &str) -> String {
    format!(
        "xcb_{}{}",
        &xcb_mod_prefix,
        tit_dig_split(req_name).to_ascii_lowercase(),
    )
}

pub fn reply_fn_name(xcb_mod_prefix: &str, req_name: &str) -> String {
    format!(
        "xcb_{}{}_reply",
        &xcb_mod_prefix,
        tit_dig_split(req_name).to_ascii_lowercase(),
    )
}

pub fn reply_fds_fn_name(xcb_mod_prefix: &str, req_name: &str) -> String {
    format!(
        "xcb_{}{}_reply_fds",
        &xcb_mod_prefix,
        tit_dig_split(req_name).to_ascii_lowercase(),
    )
}

pub fn opcode_name(xcb_mod_prefix: &str, name: &str) -> String {
    format!(
        "XCB_{}{}",
        xcb_mod_prefix.to_ascii_uppercase(),
        tit_dig_split(name).to_ascii_uppercase()
    )
}

pub fn emit_opcode<Out: Write>(
    out: &mut Out,
    xcb_mod_prefix: &str,
    name: &str,
    num: i32,
) -> io::Result<()> {
    let op_name = opcode_name(xcb_mod_prefix, name);
    let num_typ = if num < 0 { "i8" } else { "u8" };
    writeln!(out)?;
    writeln!(out, "pub const {}: {} = {};", &op_name, &num_typ, num)?;
    Ok(())
}
