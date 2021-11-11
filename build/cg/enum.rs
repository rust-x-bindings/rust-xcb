use super::{util, CodeGen};
use crate::cg::doc::emit_doc_text;
use crate::cg::{self, Doc, TypeInfo};
use crate::ir::{self, EnumItem};

use std::io::{self, Write};

impl CodeGen {
    pub(super) fn resolve_enum(&mut self, typ: &str, items: &[EnumItem], doc: &Option<ir::Doc>) {
        let doc = self.resolve_doc(doc.clone());
        let is_mask = items
            .iter()
            .any(|item| matches!(item, ir::EnumItem::Bit { .. }));
        let mut rs_typ = cg::rust_type_name(typ);

        // solving name conflicts that may happen with XID of the same name
        if rs_typ == "Event" || self.rs_typs[&rs_typ] > 1 {
            *self.rs_typs.get_mut(&rs_typ).unwrap() -= 1;
            rs_typ.push_str(if is_mask { "Flags" } else { "Enum" });
            self.rs_typs.insert(rs_typ.clone(), 1);
        }

        for ex in &self.mask_exceptions {
            if ex.module == self.xcb_mod && ex.rs_typ == rs_typ {
                rs_typ = ex.new_rs_typ.to_string();
                break;
            }
        }

        let items = self.map_enum_items(items, is_mask, doc.as_ref());
        let info = if is_mask {
            TypeInfo::Mask {
                module: None,
                rs_typ,
                items,
                doc,
            }
        } else {
            TypeInfo::Enum {
                module: None,
                rs_typ,
                items,
                doc,
            }
        };
        self.register_typ(typ.to_string(), info);
    }

    fn map_enum_items(
        &self,
        items: &[EnumItem],
        is_mask: bool,
        doc: Option<&Doc>,
    ) -> Vec<(String, u32, Option<String>)> {
        let mut items = items.iter();
        let mut vec = vec![self.map_enum_item(items.next().unwrap(), is_mask, doc)];
        let mut last_val = vec[0].1;
        for ei in items {
            let ei = self.map_enum_item(ei, is_mask, doc);
            if ei.1 != last_val {
                last_val = ei.1;
                vec.push(ei);
            }
        }
        vec
    }

    fn map_enum_item(
        &self,
        ei: &ir::EnumItem,
        is_mask: bool,
        doc: Option<&Doc>,
    ) -> (String, u32, Option<String>) {
        match ei {
            ir::EnumItem::Bit(name, value) => {
                let doc = self.doc_lookup_field(doc, &cg::rust_field_name(name));
                (map_mask_item_name(name), 1 << value, doc.map(|d| d.text))
            }
            ir::EnumItem::Value(name, value) => {
                let doc = self.doc_lookup_field(doc, &cg::rust_field_name(name));
                (
                    if is_mask {
                        map_mask_item_name(name)
                    } else {
                        map_enum_item_name(name)
                    },
                    *value,
                    doc.map(|d| d.text),
                )
            }
        }
    }

    pub(super) fn emit_enum<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        items: &[(String, u32, Option<String>)],
        doc: Option<&Doc>,
    ) -> io::Result<()> {
        writeln!(out)?;
        if self.xcb_mod == "xproto" && rs_typ == "AtomEnum" {
            for item in items {
                if let Some(text) = &item.2 {
                    emit_doc_text(out, 0, text)?;
                }
                writeln!(
                    out,
                    "pub const ATOM_{}: Atom = Atom{{ res_id: {} }};",
                    util::tit_split(&item.0).to_ascii_uppercase(),
                    item.1
                )?;
            }

            return Ok(());
        }

        if let Some(doc) = doc {
            doc.emit(out, 0)?;
        }

        if self.xcb_mod == "xproto" && rs_typ == "SendEventDest" {
            writeln!(out, "#[derive(Copy, Clone, Debug)]")?;
            writeln!(out, "pub enum SendEventDest {{")?;
            for item in items {
                if let Some(text) = &item.2 {
                    emit_doc_text(out, 1, text)?;
                }
                writeln!(out, "    {},", item.0)?;
            }
            writeln!(out, "    Window(Window),")?;
            writeln!(out, "}}")?;

            writeln!(out)?;
            writeln!(out, "impl SendEventDest {{")?;
            writeln!(out, "    pub fn resource_id(&self) -> u32 {{")?;
            writeln!(out, "        match self {{")?;
            for item in items {
                writeln!(
                    out,
                    "{}SendEventDest::{} => {},",
                    cg::ind(3),
                    item.0,
                    item.1
                )?;
            }
            writeln!(
                out,
                "{}SendEventDest::Window(w) => w.resource_id(),",
                cg::ind(3)
            )?;
            writeln!(out, "        }}")?;
            writeln!(out, "    }}")?;
            writeln!(out)?;
            writeln!(
                out,
                "    pub fn serialize(&self, wire_buf: &mut [u8]) -> usize {{"
            )?;
            writeln!(out, "        debug_assert!(wire_buf.len() >= 4);")?;
            writeln!(out, "        unsafe {{")?;
            writeln!(
                out,
                "            *(wire_buf.as_mut_ptr() as *mut u32) = self.resource_id();"
            )?;
            writeln!(out, "        }}")?;
            writeln!(out, "        4")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;

            return Ok(());
        }

        writeln!(out, "#[derive(Copy, Clone, Debug, PartialEq)]")?;
        writeln!(out, "#[repr(u32)]")?;
        writeln!(out, "pub enum {} {{", rs_typ)?;
        for item in items {
            if let Some(text) = &item.2 {
                emit_doc_text(out, 1, text)?;
            }
            writeln!(out, "    {} = {},", item.0, item.1)?;
        }
        writeln!(out, "}}")?;

        Ok(())
    }

    pub(super) fn emit_mask<O: Write>(
        &self,
        out: &mut O,
        rs_typ: &str,
        items: &[(String, u32, Option<String>)],
        doc: Option<&Doc>,
    ) -> io::Result<()> {
        writeln!(out)?;
        writeln!(out, "bitflags! {{")?;
        if let Some(doc) = doc {
            doc.emit(out, 1)?;
        }
        writeln!(out, "    pub struct {}: u32 {{", rs_typ)?;
        for item in items {
            if let Some(text) = &item.2 {
                emit_doc_text(out, 2, text)?;
            }
            writeln!(out, "{}const {} = {:#010x};", cg::ind(2), item.0, item.1)?;
        }
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}
pub(super) fn map_enum_item_name(name: &str) -> String {
    let mut name = util::tit_cap(name);
    if name.chars().next().unwrap().is_digit(10) {
        name.insert(0, 'N');
    }
    name
}

pub(super) fn map_mask_item_name(name: &str) -> String {
    let mut name = util::tit_split(name).to_ascii_uppercase();
    if name.chars().next().unwrap().is_digit(10) {
        name.insert(0, 'N');
    }
    name
}
