use std::io::{self, Write};

use super::ffi;

use super::{
    emit_doc_field, emit_doc_field_indent, emit_doc_text, enum_suffix_exception, expr_fixed_length,
    extract_module, field_name, qualified_name, request_has_template, tit_cap, tit_dig_split,
    CodeGen, ListField, KEYWORDS,
};
use crate::ast::{Doc, OpCopy, Reply, Struct, StructField, SwitchCase};

impl CodeGen {
    fn has_rs_type(&self, typ: &str) -> bool {
        self.rs_typ_reg.contains(typ)
    }

    pub fn rs_enum_type_name(&mut self, typ: &str) -> String {
        let try1 = type_name(&typ);
        if self.has_rs_type(&try1) || enum_suffix_exception(&self.xcb_mod, typ) {
            format!("{}Enum", &try1)
        } else {
            try1
        }
    }

    fn rs_use_type_name(&self, typ: &str) -> String {
        let (module, typ) = extract_module(&typ);

        let module = module.or_else(|| {
            if self.has_type(&typ) {
                None
            } else {
                let mut module = None;
                for di in self.dep_info.iter() {
                    if di.has_type(typ) {
                        module = Some(di.xcb_mod.as_str());
                        break;
                    }
                }
                module
            }
        });

        let typ = type_name(&typ);

        qualified_name(&self.xcb_mod, &module, &typ)
    }

    fn emit_rs_struct_field_accessors(
        &mut self,
        typ_name: &str,
        stru: &Struct,
        accessor: &str,
        skip_fields: &[&str],
        has_lifetime: bool,
        is_reply: bool,
    ) -> io::Result<()> {
        let has_reply_fd_nfd = is_reply && has_fd_nfd(&stru.fields);
        for f in stru.fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if skip_fields.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
                    if has_reply_fd_nfd && name == "nfd" {
                        continue;
                    }
                    let is_simple = self.typ_is_simple(&typ);
                    let is_pod = self.typ_is_pod(&typ);

                    let rs_typ = self.rs_use_type_name(&typ);

                    let out = &mut self.rs_buf;
                    emit_doc_field(out, &stru.doc, &name)?;

                    let f_name = field_name(name);

                    if !self.typ_unions.contains(typ) {
                        writeln!(out, "    pub fn {}(&self) -> {} {{", &f_name, rs_typ,)?;
                        if typ == "BOOL" {
                            writeln!(out, "        unsafe {{ {}.{} != 0 }}", &accessor, &f_name)?;
                        } else if is_pod && !is_simple {
                            writeln!(
                                out,
                                "        unsafe {{ std::mem::transmute({}.{}) }}",
                                &accessor, &f_name
                            )?;
                        } else if !is_pod && !is_simple {
                            writeln!(
                                out,
                                "        unsafe {{ std::mem::transmute(&{}.{}) }}",
                                &accessor, &f_name
                            )?;
                        } else {
                            writeln!(out, "        unsafe {{ {}.{} }}", &accessor, &f_name)?;
                        }
                        writeln!(out, "    }}")?;
                    } else {
                        // unions are returned by ref and need lifetime (TODO: check if really needed)
                        // adding lifetime declaration if not already declared in the impl opening
                        let lifetime_decl = if has_lifetime { "" } else { "<'a>" };
                        writeln!(
                            out,
                            "    pub fn {}{}(&'a self) -> &'a {} {{",
                            &name, &lifetime_decl, &rs_typ
                        )?;
                        writeln!(out, "        unsafe {{ &{}.{} }}", &accessor, &name)?;
                        writeln!(out, "    }}")?;
                    }
                }
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    if skip_fields.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
                    let f_name = field_name(name);
                    let is_simple = self.typ_is_simple(&typ);
                    let is_string = typ == "char";
                    let fixed_len = expr_fixed_length(len_expr);

                    let rs_typ = self.rs_use_type_name(&typ);

                    if fixed_len.is_some() {
                        let out = &mut self.rs_buf;
                        writeln!(out, "    pub fn {}(&self) -> &[{}] {{", &f_name, &rs_typ)?;
                        writeln!(out, "        unsafe {{ &(*self.ptr).{} }}", &f_name)?;
                        writeln!(out, "    }}")?;
                    } else if is_simple {
                        let len_fn = ffi::field_list_iterator_len_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let acc_fn = ffi::field_list_iterator_acc_fn_name(
                            &self.xcb_mod_prefix,
                            &stru.name,
                            &name,
                        );
                        let is_template = typ == "void";
                        let template = if is_template { "<T>" } else { "" };
                        let ret = match typ.as_str() {
                            "char" => "&str".to_string(),
                            "void" => "&[T]".to_string(),
                            "BOOL" => "Vec<bool>".to_string(),
                            _ => format!("&[{}]", &rs_typ),
                        };
                        let out = &mut self.rs_buf;
                        writeln!(
                            out,
                            "    pub fn {}{}(&self) -> {} {{",
                            &f_name, &template, &ret
                        )?;
                        writeln!(out, "        unsafe {{")?;
                        writeln!(out, "            let field = self.ptr;")?;
                        writeln!(out, "            let len = {}(field) as usize;", &len_fn)?;
                        writeln!(out, "            let data = {}(field);", &acc_fn)?;
                        if typ == "BOOL" {
                            writeln!(
                                out,
                                "           let slice = std::slice::from_raw_parts(data, len);"
                            )?;
                            writeln!(out, "           slice.iter().map(|el| if *el == 0 {{ false }} else {{ true }}).collect()")?;
                        } else if is_string {
                            writeln!(out, "            let slice = std::slice::from_raw_parts(data as *const u8, len);")?;
                            writeln!(out, "            // should we check what comes from X?")?;
                            writeln!(out, "            std::str::from_utf8_unchecked(&slice)")?;
                        } else if is_template {
                            writeln!(
                                out,
                                "            debug_assert_eq!(len % std::mem::size_of::<T>(), 0);"
                            )?;
                            writeln!(out, "            std::slice::from_raw_parts(data as *const T, len / std::mem::size_of::<T>())")?;
                        } else {
                            writeln!(out, "            std::slice::from_raw_parts(data, len)")?;
                        }
                        writeln!(out, "        }}")?;
                        writeln!(out, "    }}")?;
                    } else {
                        let has_lifetime = has_lifetime && self.typ_with_lifetime.contains(typ);
                        let lifetime = if has_lifetime { "<'a>" } else { "" };
                        let it_typ = self.rs_use_type_name(&typ) + "Iterator";
                        let ffi_it_fn_name = ffi::field_list_iterator_it_fn_name(
                            &self.xcb_mod_prefix,
                            &typ_name,
                            &name,
                        );
                        let out = &mut self.rs_buf;
                        writeln!(
                            out,
                            "    pub fn {}(&self) -> {}{} {{",
                            &f_name, &it_typ, &lifetime
                        )?;
                        writeln!(out, "        unsafe {{ {}(self.ptr) }}", &ffi_it_fn_name)?;
                        writeln!(out, "    }}")?;
                    }
                }
                StructField::ListNoLen { name, typ } => {
                    let has_lifetime = has_lifetime && self.typ_with_lifetime.contains(typ);
                    let lifetime = if has_lifetime { "<'a>" } else { "" };
                    let len_name = name.clone() + "_len";
                    let rs_name = field_name(&name);
                    let it_typ = self.rs_use_type_name(&typ) + "Iterator";
                    let ffi_it_fn_name =
                        ffi::field_list_iterator_it_fn_name(&self.xcb_mod_prefix, &typ_name, &name);
                    let out = &mut self.rs_buf;
                    writeln!(out, "    pub fn {}(&self) -> u32 {{", &len_name)?;
                    writeln!(out, "        unsafe {{ {}.{} }}", &accessor, &len_name)?;
                    writeln!(out, "    }}")?;
                    writeln!(
                        out,
                        "    pub fn {}(&self) -> {}{} {{",
                        &rs_name, &it_typ, &lifetime
                    )?;
                    writeln!(out, "        unsafe {{ {}(self.ptr) }}", &ffi_it_fn_name)?;
                    writeln!(out, "    }}")?;
                }
                StructField::ValueParam {
                    mask_typ,
                    mask_name,
                    list_name,
                } => {
                    let rs_mask_typ = type_name(&mask_typ);
                    let f_mask_name = field_name(&mask_name);
                    let rs_list_name = field_name(&list_name);
                    let len_fn = ffi::field_list_iterator_len_fn_name(
                        &self.xcb_mod_prefix,
                        &stru.name,
                        &list_name,
                    );
                    let acc_fn = ffi::field_list_iterator_acc_fn_name(
                        &self.xcb_mod_prefix,
                        &stru.name,
                        &list_name,
                    );
                    let out = &mut self.rs_buf;
                    writeln!(
                        out,
                        "    pub fn {}(&self) -> {} {{",
                        &f_mask_name, &rs_mask_typ
                    )?;
                    writeln!(out, "        unsafe {{ (*self.ptr).{} }}", &f_mask_name)?;
                    writeln!(out, "    }}")?;
                    writeln!(out, "    pub fn {}(&self) -> &[u32] {{", &rs_list_name)?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(out, "            let field = self.ptr;")?;
                    writeln!(out, "            let len = {}(field) as usize;", len_fn)?;
                    writeln!(out, "            let data = {}(field);", acc_fn)?;
                    writeln!(out, "            std::slice::from_raw_parts(data, len)")?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                _ => {}
            }
        }

        if has_reply_fd_nfd {
            for f in stru.fields.iter() {
                match f {
                    StructField::Fd(name) => {
                        let getter = ffi::reply_fds_fn_name(&self.xcb_mod_prefix, &stru.name);
                        let out = &mut self.rs_buf;
                        writeln!(
                            out,
                            "    pub fn {}s(&self, c: &base::Connection) -> &[i32] {{",
                            name
                        )?;
                        writeln!(out, "        unsafe {{")?;
                        writeln!(out, "            let nfd = {}.nfd as usize;", &accessor)?;
                        writeln!(
                            out,
                            "            let ptr = {}(c.get_raw_conn(), self.ptr);",
                            &getter
                        )?;
                        writeln!(out, "            std::slice::from_raw_parts(ptr, nfd)")?;
                        writeln!(out, "        }}")?;
                        writeln!(out, "    }}")?;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn emit_rs_struct_impl(
        &mut self,
        rs_typ: &str,
        ffi_typ: &str,
        stru: &Struct,
        has_lifetime: bool,
        is_pod: bool,
    ) -> io::Result<()> {
        let (accessor, lifetime) = if has_lifetime {
            ("(*self.ptr)", "<'a>")
        } else {
            ("self.base", "")
        };
        // emitting struct impl
        {
            let out = &mut self.rs_buf;

            writeln!(out)?;
            writeln!(out, "impl{} {}{} {{", &lifetime, &rs_typ, &lifetime)?;
        }

        // emitting ctor
        if is_pod {
            {
                let out = &mut self.rs_buf;
                writeln!(out, "    #[allow(unused_unsafe)]")?;
                write!(out, "    pub fn new(")?;
            }
            for f in stru.fields.iter() {
                match f {
                    StructField::Field { name, typ, .. } => {
                        let rs_typ = self.rs_use_type_name(&typ);
                        let name = field_name(name);
                        write!(&mut self.rs_buf, "{}: {},", name, rs_typ)?;
                    }
                    _ => {}
                }
            }
            {
                let out = &mut self.rs_buf;
                writeln!(out, ") -> {} {{", &rs_typ)?;
                writeln!(out, "        unsafe {{")?;
                writeln!(out, "            {} {{", &rs_typ)?;
                writeln!(out, "                base: {} {{", &ffi_typ)?;
            }
            for f in stru.fields.iter() {
                match f {
                    StructField::Field { name, typ, .. } => {
                        let f_name = field_name(name);
                        let name = field_name(name);
                        let is_pod = self.typ_is_pod(&typ);
                        let is_simple = self.typ_is_simple(&typ);
                        let out = &mut self.rs_buf;
                        if typ == "BOOL" {
                            writeln!(
                                out,
                                "                    {}: if {} {{ 1 }} else {{ 0 }},",
                                &f_name, &name
                            )?;
                        } else if is_pod && !is_simple {
                            writeln!(
                                out,
                                "                    {}: std::mem::transmute({}),",
                                &f_name, &name
                            )?;
                        } else {
                            writeln!(out, "                    {}: {},", &f_name, &name)?;
                        }
                    }
                    StructField::Pad(name, sz) => {
                        let val = if *sz == 1 {
                            "0".into()
                        } else {
                            format!("[0; {}]", sz)
                        };
                        let out = &mut self.rs_buf;
                        writeln!(out, "                {}: {},", name, val)?;
                    }
                    _ => {}
                }
            }
            let out = &mut self.rs_buf;
            writeln!(out, "                }}")?;
            writeln!(out, "            }}")?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
        }

        // emitting accessors
        self.emit_rs_struct_field_accessors(&stru.name, &stru, accessor, &[], has_lifetime, false)?;

        writeln!(&mut self.rs_buf, "}}")?;

        Ok(())
    }

    pub fn emit_rs_iterator(
        &mut self,
        name: &str,
        typ: &str,
        ffi_it_typ: &str,
        has_lifetime: bool,
        is_union: bool,
    ) -> io::Result<()> {
        let it_typ = format!("{}Iterator", &typ);
        let ffi_it_next = ffi::iterator_next_fn_name(&self.xcb_mod_prefix, &name);

        let lifetime = if has_lifetime { "<'a>" } else { "" };

        let return_expr = match (has_lifetime, is_union) {
            (true, true) => unimplemented!(),
            (true, false) => "std::mem::transmute(data)",
            (false, true) => "*data",
            (false, false) => "std::mem::transmute(*data)",
        };

        let out = &mut self.rs_buf;

        writeln!(out)?;
        writeln!(
            out,
            "pub type {}{} = {}{};",
            &it_typ, &lifetime, &ffi_it_typ, &lifetime
        )?;
        writeln!(out)?;
        writeln!(
            out,
            "impl{} Iterator for {}{} {{",
            &lifetime, &it_typ, &lifetime
        )?;
        writeln!(out, "    type Item = {}{};", &typ, &lifetime)?;
        writeln!(
            out,
            "    fn next(&mut self) -> std::option::Option<{}{}> {{",
            &typ, &lifetime,
        )?;
        writeln!(out, "        if self.rem == 0 {{")?;
        writeln!(out, "            None")?;
        writeln!(out, "        }} else {{")?;
        writeln!(out, "            unsafe {{")?;
        writeln!(
            out,
            "                let iter = self as *mut {};",
            &ffi_it_typ
        )?;
        writeln!(out, "                let data = (*iter).data;")?;
        writeln!(out, "                {}(iter);", &ffi_it_next)?;
        writeln!(out, "                Some({})", &return_expr)?;
        writeln!(out, "            }}")?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    pub fn emit_rs_struct(
        &mut self,
        ffi_typ: &str,
        stru: &Struct,
        has_lifetime: bool,
    ) -> io::Result<String> {
        let Struct { name, doc, .. } = &stru;

        let typ = type_name(&name);

        let out = &mut self.rs_buf;
        // emitting struct
        writeln!(out)?;
        emit_doc_text(out, &doc)?;
        if has_lifetime {
            writeln!(
                out,
                "pub type {}<'a> = base::StructPtr<'a, {}>;",
                &typ, &ffi_typ
            )?;
        } else {
            writeln!(out, "#[derive(Copy, Clone)]")?;
            writeln!(out, "pub struct {} {{", &typ)?;
            writeln!(out, "    pub base: {},", &ffi_typ)?;
            writeln!(out, "}}")?;
        }

        self.emit_rs_struct_impl(&typ, &ffi_typ, &stru, has_lifetime, !has_lifetime)?;

        Ok(typ)
    }

    pub fn emit_rs_union_impl(
        &mut self,
        rs_typ: &str,
        ffi_sz: usize,
        stru: &Struct,
    ) -> io::Result<()> {
        let Struct { fields, doc, .. } = &stru;

        {
            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "impl {} {{", &rs_typ)?;
        }

        // emitting accessors
        for f in fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    let rs_name = field_name(name);
                    let f_rs_typ = self.rs_use_type_name(&typ);
                    let has_lifetime = self.type_has_lifetime(&typ);

                    let out = &mut self.rs_buf;
                    emit_doc_field(out, &doc, &name)?;

                    if has_lifetime {
                        writeln!(
                            out,
                            "    pub fn {}<'a> (&'a self) -> {}<'a> {{",
                            &rs_name, &f_rs_typ
                        )?;
                        writeln!(out, "        unsafe {{ std::mem::transmute(self) }}")?;
                        writeln!(out, "    }}")?;
                    } else {
                        writeln!(out)?;
                        writeln!(out, "    pub fn {}(&self) -> {} {{", &rs_name, &f_rs_typ)?;
                        writeln!(out, "        unsafe {{")?;
                        writeln!(
                            out,
                            "            let _ptr = self.data.as_ptr() as *const {};",
                            &f_rs_typ
                        )?;
                        writeln!(out, "            *_ptr")?;
                        writeln!(out, "        }}")?;
                        writeln!(out, "    }}")?;
                        writeln!(
                            out,
                            "    pub fn from_{}({}: {}) -> {} {{",
                            &rs_name, &rs_name, &f_rs_typ, &rs_typ
                        )?;
                        writeln!(out, "        unsafe {{")?;
                        writeln!(
                            out,
                            "            let mut res = {} {{ data: [0; {}] }};",
                            &rs_typ, ffi_sz
                        )?;
                        writeln!(
                            out,
                            "            let res_ptr = res.data.as_mut_ptr() as *mut {};",
                            &f_rs_typ
                        )?;
                        writeln!(out, "            *res_ptr = {};", &rs_name)?;
                        writeln!(out, "            res")?;
                        writeln!(out, "        }}")?;
                        writeln!(out, "    }}")?;
                    }
                }
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    let rs_name = field_name(name);
                    let f_rs_typ = self.rs_use_type_name(&typ);
                    let len =
                        expr_fixed_length(len_expr).expect("union list field with variable length");

                    let out = &mut self.rs_buf;
                    emit_doc_field(out, &doc, &name)?;
                    // accessor
                    writeln!(out, "    pub fn {}(&self) -> &[{}] {{", &rs_name, &f_rs_typ)?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(
                        out,
                        "            let ptr = self.data.as_ptr() as *const {};",
                        &f_rs_typ
                    )?;
                    writeln!(out, "            std::slice::from_raw_parts(ptr, {})", len)?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                    // constructor
                    writeln!(
                        out,
                        "    pub fn from_{}({}: [{}; {}]) -> {} {{",
                        &rs_name, &rs_name, &f_rs_typ, len, &rs_typ
                    )?;
                    writeln!(out, "        unsafe {{")?;
                    writeln!(out, "            {} {{", &rs_typ)?;
                    writeln!(
                        out,
                        "                data: std::mem::transmute({})",
                        &rs_name
                    )?;
                    writeln!(out, "            }}")?;
                    writeln!(out, "        }}")?;
                    writeln!(out, "    }}")?;
                }
                _ => {
                    unimplemented!();
                }
            }
        }
        let out = &mut self.rs_buf;
        writeln!(out, "}}")?;

        Ok(())
    }

    pub fn emit_rs_event(
        &mut self,
        name: &str,
        number: i32,
        stru: &Struct,
        ffi_typ: &str,
        opcopies: &[OpCopy],
        xge: bool,
    ) -> io::Result<String> {
        let rs_typ = type_name(&stru.name);

        let opn = opname(&name);
        let num_typ = if number < 0 { "i8" } else { "u8" };

        {
            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "pub const {}: {} = {};", &opn, &num_typ, number)?;

            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            writeln!(out, "pub type {} = base::Event<{}>;", &rs_typ, &ffi_typ)?;

            writeln!(out)?;
            writeln!(out, "impl {} {{", &rs_typ)?;
        }

        let field_skip = {
            let mut fs = if xge {
                vec![
                    "sequence",
                    "extension",
                    "length",
                    "event_type",
                    "full_sequence",
                ]
            } else {
                vec!["sequence"]
            };
            if opcopies.is_empty() {
                fs.push("response_type");
            }
            fs
        };

        {
            let mut fs = field_skip.clone();
            fs.push("response_type");
            self.emit_rs_struct_field_accessors(&name, &stru, "(*self.ptr)", &fs, false, false)?;
        }

        // emitting ctor

        {
            let out = &mut self.rs_buf;

            writeln!(out, "    /// Constructs a new {}", &rs_typ)?;
            if opcopies.is_empty() {
                writeln!(
                    out,
                    "    /// `response_type` will be set automatically to {}",
                    &opn
                )?;
            } else {
                writeln!(out, "    /// `response_type` must be set to one of:")?;
                writeln!(out, "    ///     - `{}`", &opn)?;
                for op in opcopies.iter() {
                    let opn = opname(&op.name);
                    writeln!(out, "    ///     - `{}`", &opn)?;
                }
            }

            writeln!(out, "    pub fn new (")?;
        }

        for f in stru.fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if field_skip.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
                    let rs_typ = self.rs_use_type_name(&typ);
                    writeln!(
                        &mut self.rs_buf,
                        "        {}: {},",
                        field_name(name),
                        &rs_typ
                    )?;
                }
                StructField::Pad(_, _) => {}
                StructField::List {
                    name,
                    typ,
                    len_expr,
                } => {
                    if field_skip.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
                    let typ = match typ.as_str() {
                        "char" => "&str".into(),
                        typ => {
                            let typ = self.rs_use_type_name(&typ);
                            if let Some(len) = expr_fixed_length(&len_expr) {
                                format!("[{}; {}]", &typ, len)
                            } else {
                                format!("&[{}]", &typ)
                            }
                        }
                    };
                    writeln!(&mut self.rs_buf, "        {}: {},", field_name(name), &typ)?;
                }
                StructField::ListNoLen { name, typ } => {
                    let len_name = name.clone() + "_len";
                    let typ = self.rs_use_type_name(&typ);
                    writeln!(&mut self.rs_buf, "        {}: u32,", len_name)?;
                    writeln!(&mut self.rs_buf, "        {}: {},", field_name(&name), typ)?;
                }
                _ => unimplemented!("{}::{}::{:?}", self.xcb_mod, &rs_typ, f),
            }
        }

        {
            let out = &mut self.rs_buf;

            writeln!(out, "    ) -> {} {{", &rs_typ)?;
            writeln!(out, "        unsafe {{")?;
            writeln!(
                out,
                "            let raw = libc::malloc(32 as usize) as *mut {};",
                &ffi_typ
            )?;
            if !opcopies.is_empty() {
                let copies = opcopies.iter().map(|opc| opname(&opc.name));
                let opcodes = Some(opn.clone()).into_iter().chain(copies);
                let assert_expr = opcodes
                    .map(|opc| format!("response_type == {}", &opc))
                    .collect::<Vec<String>>()
                    .join(" || ");
                writeln!(
                    out,
                    "            assert!({}, \"wrong response_type supplied to {}::new\");",
                    &assert_expr, &rs_typ
                )?;
            } else {
                writeln!(out, "            (*raw).response_type = {};", &opn)?;
            }
        }

        for f in stru.fields.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if field_skip.iter().find(|skip| skip == &name).is_some() {
                        continue;
                    }
                    let f_name = field_name(name);

                    let expr = if !self.typ_is_simple(&typ) && self.typ_is_pod(&typ) {
                        format!("{}.base", &f_name)
                    } else if typ == "BOOL" {
                        format!("if {} {{ 1 }} else {{ 0 }}", &f_name)
                    } else {
                        f_name.clone()
                    };

                    writeln!(
                        &mut self.rs_buf,
                        "            (*raw).{} = {};",
                        f_name, expr
                    )?;
                }
                StructField::List { name, .. } => {
                    let f_name = field_name(name);
                    writeln!(
                        &mut self.rs_buf,
                        "            (*raw).{} = {};",
                        f_name, f_name
                    )?;
                }
                StructField::ListNoLen { name, .. } => {
                    let len_name = name.clone() + "_len";
                    let f_name = field_name(&name);
                    writeln!(
                        &mut self.rs_buf,
                        "            (*raw).{} = {};",
                        len_name, len_name
                    )?;
                    writeln!(
                        &mut self.rs_buf,
                        "            (*raw).{} = {};",
                        f_name, f_name
                    )?;
                }
                _ => {}
            }
        }

        let out = &mut self.rs_buf;

        writeln!(out, "            {} {{ ptr: raw }}", &rs_typ)?;
        writeln!(out, "        }}")?; // closing unsafe
        writeln!(out, "    }}")?; // closing new
        writeln!(out, "}}")?; // closing impl

        for opc in opcopies.iter() {
            let opn = opname(&opc.name);
            let name = opc.name.clone() + "Event";
            let num_typ = if number < 0 { "i8" } else { "u8" };
            let rs_typ = type_name(&name);
            let ffi_typ = self.ffi_decl_type_name(&name);

            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "pub const {}: {} = {};", &opn, &num_typ, &opc.number)?;

            writeln!(out)?;
            emit_doc_text(out, &stru.doc)?;
            writeln!(out, "pub type {} = base::Event<{}>;", &rs_typ, &ffi_typ)?;
        }

        Ok(rs_typ)
    }

    pub fn emit_rs_switch_typedef(
        &mut self,
        req_name: &str,
        switch_name: &str,
        cases: &[SwitchCase],
        toplevel_typ: &str,
        _parent_switch: Option<&str>,
    ) -> io::Result<()> {
        let ffi_typ = ffi::switch_struct_name(&self.xcb_mod_prefix, &req_name, &switch_name);
        let rs_typ = switch_type_name(&req_name, &switch_name);

        let out = &mut self.rs_buf;

        writeln!(
            out,
            "pub type {}<'a> = base::StructPtr<'a, {}>;",
            &rs_typ, &ffi_typ
        )?;

        for c in cases.iter() {
            for f in c.fields.iter() {
                if let StructField::Switch(cname, _, ccases) = f {
                    let rs_typ = if let Some(name) = &c.name {
                        rs_typ.clone() + &tit_cap(&name)
                    } else {
                        rs_typ.clone()
                    };
                    self.emit_rs_switch_typedef(
                        &rs_typ,
                        cname,
                        ccases,
                        &toplevel_typ,
                        Some(&ffi_typ),
                    )?;
                }
            }
        }

        Ok(())
    }

    pub fn emit_rs_req_fn(
        &mut self,
        req_name: &str,
        fn_name: &str,
        ffi_fn_name: &str,
        cookie_name: &str,
        params: &[StructField],
        doc: &Option<Doc>,
        checked: bool,
    ) -> io::Result<()> {
        // special case for the send_event request
        let send_event = match (req_name, self.xcb_mod.as_str()) {
            ("SendEvent", "xproto") => true,
            ("Send", "xevie") => true,
            (_, _) => false,
        };
        let event_is_list = if send_event {
            let mut is_list = false;
            for f in params.iter() {
                if let StructField::List { name, .. } = f {
                    if name == "event" {
                        is_list = true;
                        break;
                    }
                }
            }
            is_list
        } else {
            false
        };

        let has_template = request_has_template(&params) || event_is_list;
        let list_fields = ListField::fetch_from(&params);
        let skip_fields = {
            let mut sf = Vec::new();

            for f in params.iter() {
                if let StructField::ValueParam { mask_name, .. } = f {
                    sf.push(mask_name.clone());
                }
            }

            sf
        };

        let template = if has_template { ", T" } else { "" };

        {
            let out = &mut self.rs_buf;

            writeln!(out)?;
            emit_doc_text(out, &doc)?;
            if let Some(_) = &doc {
                writeln!(out, "///")?;
                writeln!(out, "/// parameters:")?;
                writeln!(out, "///")?;
                writeln!(out, "///   - __c__:")?;
                writeln!(out, "///       The connection object to the server")?;
                for f in params.iter() {
                    if let Some(name) = field_doc_name(&f) {
                        writeln!(out, "///")?;
                        writeln!(out, "///   - __{}__:", &name)?;
                        emit_doc_field_indent(out, &doc, &name, "       ")?;
                    }
                }
            }
            writeln!(out, "pub fn {}<'a{}>(", &fn_name, &template)?;

            writeln!(out, "    c: &'a base::Connection,")?;
        }

        // request parameters
        for f in params.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if skip_fields.contains(&name) {
                        continue;
                    }
                    if list_fields.iter().any(|lf| &lf.lenfield == name) {
                        continue;
                    }
                    let name = field_name(&name);
                    let typ = self.rs_use_type_name(&typ);
                    let out = &mut self.rs_buf;
                    writeln!(out, "    {}: {},", name, typ)?;
                }
                StructField::Fd(name) => {
                    let name = field_name(&name);
                    writeln!(&mut self.rs_buf, "    {}: i32,", name)?;
                }
                StructField::List { name, typ, .. } | StructField::ListNoLen { name, typ } => {
                    let name = field_name(&name);
                    let typ = match (send_event, name.as_str(), typ.as_str()) {
                        (true, "event", _) => "&base::Event<T>".to_string(),
                        (_, _, "char") => "&str".to_string(),
                        (_, _, "void") => "&[T]".to_string(),
                        (_, _, typ) => {
                            format!("&[{}]", self.rs_use_type_name(&typ))
                        }
                    };
                    let out = &mut self.rs_buf;
                    writeln!(out, "    {}: {},", name, typ)?;
                }
                StructField::ValueParam {
                    list_name,
                    mask_typ,
                    ..
                } => {
                    let name = field_name(&list_name);
                    let typ = self.rs_use_type_name(&mask_typ);
                    let out = &mut self.rs_buf;
                    writeln!(out, "    {}: &[({}, u32)],", &name, &typ)?;
                }
                StructField::Switch(name, ..) => {
                    let rs_typ = switch_type_name(req_name, name);
                    let name = field_name(&name);
                    let out = &mut self.rs_buf;
                    writeln!(out, "    {}: std::option::Option<{}>,", &name, &rs_typ)?;
                }
                _ => {}
            }
        }

        let out = &mut self.rs_buf;
        writeln!(out, ") -> {}<'a> {{", cookie_name)?;
        writeln!(out, "    unsafe {{")?;

        let mut issued_len_vars = Vec::new();
        let mut issued_list_vars = Vec::new();

        // local variables
        if event_is_list {
            writeln!(
                out,
                "        let event_ptr = std::mem::transmute(event.ptr);"
            )?;
        }
        for ListField {
            name,
            typ,
            lenfield,
        } in list_fields.iter()
        {
            issued_list_vars.push(name);
            let name = field_name(&name);
            if typ == "char" {
                writeln!(out, "        let {} = {}.as_bytes();", &name, &name)?;
            }
            if !issued_len_vars.contains(&lenfield) {
                writeln!(out, "        let {}_len = {}.len();", &name, &name)?;
                issued_len_vars.push(&lenfield);
            }
            writeln!(out, "        let {}_ptr = {}.as_ptr();", &name, &name)?;
        }
        for f in params.iter() {
            match f {
                StructField::List { name, .. } => {
                    if issued_list_vars.contains(&name) {
                        continue;
                    }
                    if send_event && name == "event" {
                        continue;
                    }
                    let name = field_name(&name);
                    writeln!(out, "        let {}_ptr = {}.as_ptr();", &name, &name)?;
                }
                StructField::ValueParam { list_name, .. } => {
                    let list_sym = field_name(&list_name);
                    writeln!(
                        out,
                        "        let mut {}_copy = {}.to_vec();",
                        &list_name, &list_sym
                    )?;
                    writeln!(
                        out,
                        "        let ({}_mask, {}_vec) = base::pack_bitfield(&mut {}_copy);",
                        &list_name, &list_name, &list_name
                    )?;
                    writeln!(
                        out,
                        "        let {}_ptr = {}_vec.as_ptr();",
                        &list_name, &list_name
                    )?;
                }
                StructField::Switch(name, ..) => {
                    let ptr_name = format!("{}_ptr", &name);
                    let ffi_typ = ffi::switch_struct_name(&self.xcb_mod_prefix, &req_name, &name);
                    let name = field_name(&name);
                    writeln!(out, "        let {} = match {} {{", &ptr_name, name)?;
                    writeln!(out, "            Some(p) => p.ptr as *const {},", &ffi_typ)?;
                    writeln!(out, "            None => std::ptr::null(),")?;
                    writeln!(out, "        }};")?;
                }
                _ => {}
            }
        }
        writeln!(out, "        let cookie = {}(", &ffi_fn_name)?;
        writeln!(out, "            c.get_raw_conn(),")?;

        // FFI request arguments
        for f in params.iter() {
            match f {
                StructField::Field { name, typ, .. } => {
                    if skip_fields.contains(&name) {
                        continue;
                    }
                    let is_pod = self.typ_is_pod(&typ);
                    let is_simple = self.typ_is_simple(&typ);
                    if send_event && name == "event" || is_pod && !is_simple {
                        let rs_name = field_name(&name);
                        let out = &mut self.rs_buf;
                        writeln!(out, "            {}.base,", &rs_name)?;
                    } else {
                        let ffi_typ = self.ffi_use_type_name(&typ);
                        let mut rs_name = field_name(&name);

                        if let Some(lf) = list_fields.iter().find(|lf| &lf.lenfield == name) {
                            rs_name = field_name(&lf.name) + "_len";
                        }
                        let out = &mut self.rs_buf;
                        writeln!(out, "            {} as {},", &rs_name, &ffi_typ)?;
                    }
                }
                StructField::Fd(name) => {
                    let name = field_name(name);
                    writeln!(&mut self.rs_buf, "            {} as i32,", &name)?;
                }
                StructField::List { name, typ, .. } => {
                    if send_event && name == "event" {
                        let out = &mut self.rs_buf;
                        writeln!(out, "            event_ptr")?;
                    } else {
                        let rs_name = field_name(&name);
                        let ffi_typ = self.ffi_use_type_name(&typ);
                        let out = &mut self.rs_buf;
                        writeln!(out, "            {}_ptr as *const {},", &rs_name, &ffi_typ)?;
                    }
                }
                StructField::ListNoLen { name, typ } => {
                    let rs_name = field_name(&name);
                    let ffi_typ = self.ffi_use_type_name(&typ);
                    let out = &mut self.rs_buf;
                    writeln!(out, "            {}_len as u32,", &rs_name)?;
                    writeln!(out, "            {}_ptr as *const {},", &rs_name, &ffi_typ)?;
                }
                StructField::ValueParam {
                    list_name,
                    mask_typ,
                    ..
                } => {
                    let rs_list_name = field_name(&list_name);
                    let typ = self.rs_use_type_name(&mask_typ);
                    let out = &mut self.rs_buf;
                    writeln!(out, "            {}_mask as {},", &rs_list_name, &typ)?;
                    writeln!(out, "            {}_ptr as *const u32,", &rs_list_name)?;
                }
                StructField::Switch(name, ..) => {
                    let rs_name = field_name(&name);
                    let out = &mut self.rs_buf;
                    writeln!(out, "            {}_ptr,", &rs_name)?;
                }
                _ => {}
            }
        }

        let out = &mut self.rs_buf;
        writeln!(out, "        );")?;
        writeln!(out, "        {} {{", cookie_name)?;
        writeln!(out, "            cookie: cookie,")?;
        writeln!(out, "            conn: c,")?;
        writeln!(
            out,
            "            checked: {},",
            if checked { "true" } else { "false" }
        )?;
        writeln!(out, "        }}")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }

    pub fn emit_rs_reply(
        &mut self,
        req_name: &str,
        ffi_cookie: &str,
        ffi_reply_fn: &str,
        ffi_reply_typ: &str,
        rs_cookie: &str,
        reply: Reply,
    ) -> io::Result<()> {
        let rs_reply = type_name(&req_name) + "Reply";

        {
            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(out, "impl base::CookieSeq for {} {{", &ffi_cookie)?;
            writeln!(out, "    fn sequence(&self) -> c_uint {{")?;
            writeln!(out, "        self.sequence")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(
                out,
                "pub type {}<'a> = base::Cookie<'a, {}>;",
                &rs_cookie, &ffi_cookie
            )?;
            writeln!(out)?;

            writeln!(out, "impl<'a> {}<'a> {{", &rs_cookie)?;
            writeln!(
                out,
                "    pub fn get_reply(self) -> Result<{}, base::ReplyError> {{",
                &rs_reply
            )?;
            writeln!(
                out,
                "        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();"
            )?;
            writeln!(
            out,
            "        let err_ptr = if self.checked {{ &mut err }} else {{ std::ptr::null_mut() }};"
        )?;
            writeln!(out, "        let reply = unsafe {{")?;
            writeln!(out, "            {} {{", &rs_reply)?;
            writeln!(out, "                ptr: {}(", &ffi_reply_fn)?;
            writeln!(out, "                    self.conn.get_raw_conn(),")?;
            writeln!(out, "                    self.cookie,")?;
            writeln!(out, "                    err_ptr,")?;
            writeln!(out, "                ),")?;
            writeln!(out, "            }}")?;
            writeln!(out, "        }};")?;
            writeln!(out, "        let checked = self.checked;")?;
            writeln!(out, "        std::mem::forget(self);")?;
            writeln!(out)?;
            writeln!(
                out,
                "        match (reply.ptr.is_null(), err.is_null(), checked) {{"
            )?;
            writeln!(out, "            (false, _, false) => Ok(reply),")?;
            writeln!(out, "            (false, true, true) => Ok(reply),")?;
            writeln!(out, "            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {{ ptr: err }})),")?;
            writeln!(
                out,
                "            (true, true, _) => Err(base::ReplyError::NullResponse),"
            )?;
            writeln!(
                out,
                "            (r, e, c) => unreachable!(\"{{:?}}\", (r, e, c)),"
            )?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        for f in &reply.fields {
            if let StructField::Switch(name, _, cases) = f {
                let toplevel = req_name.to_string() + "Reply";
                self.emit_rs_switch_typedef(&req_name, name, &cases, &toplevel, None)?;
            }
        }

        {
            let out = &mut self.rs_buf;
            writeln!(out)?;
            writeln!(
                out,
                "pub type {} = base::Reply<{}>;",
                &rs_reply, &ffi_reply_typ
            )?;
            writeln!(out)?;
            writeln!(out, "impl {} {{", &rs_reply)?;
        }

        let stru = Struct {
            name: req_name.into(),
            fields: reply.fields,
            doc: reply.doc,
        };

        self.emit_rs_struct_field_accessors(&stru.name, &stru, "(*self.ptr)", &[], false, true)?;

        {
            let out = &mut self.rs_buf;
            writeln!(out, "}}")?;
        }

        Ok(())
    }
}

fn has_fd_nfd(fields: &[StructField]) -> bool {
    let mut has_fd = false;
    let mut has_nfd = false;

    for f in fields.iter() {
        match f {
            StructField::Field { name, .. } if name == "nfd" => {
                has_nfd = true;
            }
            StructField::Fd(_) => {
                has_fd = true;
            }
            _ => {}
        }
    }

    has_fd && has_nfd
}

pub fn type_name(typ: &str) -> String {
    match typ {
        "CARD8" => "u8".into(),
        "CARD16" => "u16".into(),
        "CARD32" => "u32".into(),
        "CARD64" => "u64".into(),
        "INT8" => "i8".into(),
        "INT16" => "i16".into(),
        "INT32" => "i32".into(),
        "BYTE" => "u8".into(),
        "BOOL" => "bool".into(),
        "char" => "c_char".into(),
        "float" => "f32".into(),
        "double" => "f64".into(),
        typ => tit_cap(typ),
    }
}

fn field_doc_name(f: &StructField) -> Option<String> {
    match f {
        StructField::Field { name, .. } => Some(field_name(name)),
        StructField::List { name, .. } => Some(field_name(name)),
        StructField::ListNoLen { name, .. } => Some(field_name(name)),
        StructField::Fd(name) => Some(field_name(name)),
        StructField::ValueParam { list_name, .. } => Some(field_name(list_name)),
        _ => None,
    }
}

pub fn enum_item_name(name: &str, item: &str) -> String {
    format!("{}_{}", tit_dig_split(name), tit_dig_split(item)).to_ascii_uppercase()
}

pub fn opname(name: &str) -> String {
    tit_dig_split(&name).to_ascii_uppercase()
}

pub fn switch_type_name(req_name: &str, switch_name: &str) -> String {
    format!("{}{}", tit_cap(&req_name), tit_cap(&switch_name))
}

pub fn request_fn_name(name: &str) -> String {
    let mut res = tit_dig_split(&name).to_ascii_lowercase();
    if KEYWORDS.contains(&res.as_str()) {
        res.push('_');
    }
    res
}

pub fn emit_opcode<Out: Write>(out: &mut Out, name: &str, opcode: i32) -> io::Result<()> {
    let opname = opname(&name);
    let num_typ = if opcode < 0 { "i8" } else { "u8" };

    writeln!(out)?;
    writeln!(out, "pub const {}: {} = {};", &opname, &num_typ, opcode)?;

    Ok(())
}

pub fn emit_error<Out: Write>(out: &mut Out, ffi_typ: &str, rs_typ: &str) -> io::Result<()> {
    writeln!(out)?;
    writeln!(out, "pub struct {} {{", &rs_typ)?;
    writeln!(out, "    pub base: base::Error<{}>,", &ffi_typ)?;
    writeln!(out, "}}")?;

    Ok(())
}
