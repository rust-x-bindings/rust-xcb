/*
 * This file generated automatically from xprint.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::xprint::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub type String8 = xcb_x_print_string8_t;

pub type String8Iterator = xcb_x_print_string8_iterator_t;

pub type PrinterIterator = xcb_x_print_printer_iterator_t;

pub type PcontextIterator = xcb_x_print_pcontext_iterator_t;


pub type xcb_x_print_get_doc_t = c_uint;//{
    pub static XCB_X_PRINT_GET_DOC_FINISHED : xcb_x_print_get_doc_t = 0;
    pub static XCB_X_PRINT_GET_DOC_SECOND_CONSUMER : xcb_x_print_get_doc_t = 1;
//}

pub type xcb_x_print_ev_mask_t = c_uint;//{
    pub static XCB_X_PRINT_EV_MASK_NO_EVENT_MASK : xcb_x_print_ev_mask_t = 0;
    pub static XCB_X_PRINT_EV_MASK_PRINT_MASK : xcb_x_print_ev_mask_t = 1;
    pub static XCB_X_PRINT_EV_MASK_ATTRIBUTE_MASK : xcb_x_print_ev_mask_t = 2;
//}

pub type xcb_x_print_detail_t = c_uint;//{
    pub static XCB_X_PRINT_DETAIL_START_JOB_NOTIFY : xcb_x_print_detail_t = 1;
    pub static XCB_X_PRINT_DETAIL_END_JOB_NOTIFY : xcb_x_print_detail_t = 2;
    pub static XCB_X_PRINT_DETAIL_START_DOC_NOTIFY : xcb_x_print_detail_t = 3;
    pub static XCB_X_PRINT_DETAIL_END_DOC_NOTIFY : xcb_x_print_detail_t = 4;
    pub static XCB_X_PRINT_DETAIL_START_PAGE_NOTIFY : xcb_x_print_detail_t = 5;
    pub static XCB_X_PRINT_DETAIL_END_PAGE_NOTIFY : xcb_x_print_detail_t = 6;
//}

pub type xcb_x_print_attr_t = c_uint;//{
    pub static XCB_X_PRINT_ATTR_JOB_ATTR : xcb_x_print_attr_t = 1;
    pub static XCB_X_PRINT_ATTR_DOC_ATTR : xcb_x_print_attr_t = 2;
    pub static XCB_X_PRINT_ATTR_PAGE_ATTR : xcb_x_print_attr_t = 3;
    pub static XCB_X_PRINT_ATTR_PRINTER_ATTR : xcb_x_print_attr_t = 4;
    pub static XCB_X_PRINT_ATTR_SERVER_ATTR : xcb_x_print_attr_t = 5;
    pub static XCB_X_PRINT_ATTR_MEDIUM_ATTR : xcb_x_print_attr_t = 6;
    pub static XCB_X_PRINT_ATTR_SPOOLER_ATTR : xcb_x_print_attr_t = 7;
//}
pub struct  PrintQueryVersionCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_query_version_cookie_t> }

/** Opcode for xcb_x_print_print_query_version. */
pub static XCB_X_PRINT_PRINT_QUERY_VERSION : u8 = 0;
pub struct PrintQueryVersionReply { base:  base::Reply<xcb_x_print_print_query_version_reply_t> }
fn mk_reply_xcb_x_print_print_query_version_reply_t(reply:*mut xcb_x_print_print_query_version_reply_t) -> PrintQueryVersionReply { PrintQueryVersionReply { base : base::mk_reply(reply) } }
pub struct  PrintGetPrinterListCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_printer_list_cookie_t> }

/** Opcode for xcb_x_print_print_get_printer_list. */
pub static XCB_X_PRINT_PRINT_GET_PRINTER_LIST : u8 = 1;
pub struct PrintGetPrinterListReply { base:  base::Reply<xcb_x_print_print_get_printer_list_reply_t> }
fn mk_reply_xcb_x_print_print_get_printer_list_reply_t(reply:*mut xcb_x_print_print_get_printer_list_reply_t) -> PrintGetPrinterListReply { PrintGetPrinterListReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_x_print_print_rehash_printer_list. */
pub static XCB_X_PRINT_PRINT_REHASH_PRINTER_LIST : u8 = 20;
/** Opcode for xcb_x_print_create_context. */
pub static XCB_X_PRINT_CREATE_CONTEXT : u8 = 2;
/** Opcode for xcb_x_print_print_set_context. */
pub static XCB_X_PRINT_PRINT_SET_CONTEXT : u8 = 3;
pub struct  PrintGetContextCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_context_cookie_t> }

/** Opcode for xcb_x_print_print_get_context. */
pub static XCB_X_PRINT_PRINT_GET_CONTEXT : u8 = 4;
pub struct PrintGetContextReply { base:  base::Reply<xcb_x_print_print_get_context_reply_t> }
fn mk_reply_xcb_x_print_print_get_context_reply_t(reply:*mut xcb_x_print_print_get_context_reply_t) -> PrintGetContextReply { PrintGetContextReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_x_print_print_destroy_context. */
pub static XCB_X_PRINT_PRINT_DESTROY_CONTEXT : u8 = 5;
pub struct  PrintGetScreenOfContextCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_screen_of_context_cookie_t> }

/** Opcode for xcb_x_print_print_get_screen_of_context. */
pub static XCB_X_PRINT_PRINT_GET_SCREEN_OF_CONTEXT : u8 = 6;
pub struct PrintGetScreenOfContextReply { base:  base::Reply<xcb_x_print_print_get_screen_of_context_reply_t> }
fn mk_reply_xcb_x_print_print_get_screen_of_context_reply_t(reply:*mut xcb_x_print_print_get_screen_of_context_reply_t) -> PrintGetScreenOfContextReply { PrintGetScreenOfContextReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_x_print_print_start_job. */
pub static XCB_X_PRINT_PRINT_START_JOB : u8 = 7;
/** Opcode for xcb_x_print_print_end_job. */
pub static XCB_X_PRINT_PRINT_END_JOB : u8 = 8;
/** Opcode for xcb_x_print_print_start_doc. */
pub static XCB_X_PRINT_PRINT_START_DOC : u8 = 9;
/** Opcode for xcb_x_print_print_end_doc. */
pub static XCB_X_PRINT_PRINT_END_DOC : u8 = 10;
/** Opcode for xcb_x_print_print_put_document_data. */
pub static XCB_X_PRINT_PRINT_PUT_DOCUMENT_DATA : u8 = 11;
pub struct  PrintGetDocumentDataCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_document_data_cookie_t> }

/** Opcode for xcb_x_print_print_get_document_data. */
pub static XCB_X_PRINT_PRINT_GET_DOCUMENT_DATA : u8 = 12;
/** Opcode for xcb_x_print_print_start_page. */
pub static XCB_X_PRINT_PRINT_START_PAGE : u8 = 13;
/** Opcode for xcb_x_print_print_end_page. */
pub static XCB_X_PRINT_PRINT_END_PAGE : u8 = 14;
/** Opcode for xcb_x_print_print_select_input. */
pub static XCB_X_PRINT_PRINT_SELECT_INPUT : u8 = 15;
pub struct  PrintInputSelectedCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_input_selected_cookie_t> }

/** Opcode for xcb_x_print_print_input_selected. */
pub static XCB_X_PRINT_PRINT_INPUT_SELECTED : u8 = 16;
pub struct  PrintGetAttributesCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_attributes_cookie_t> }

/** Opcode for xcb_x_print_print_get_attributes. */
pub static XCB_X_PRINT_PRINT_GET_ATTRIBUTES : u8 = 17;
pub struct PrintGetAttributesReply { base:  base::Reply<xcb_x_print_print_get_attributes_reply_t> }
fn mk_reply_xcb_x_print_print_get_attributes_reply_t(reply:*mut xcb_x_print_print_get_attributes_reply_t) -> PrintGetAttributesReply { PrintGetAttributesReply { base : base::mk_reply(reply) } }
pub struct  PrintGetOneAttributesCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_one_attributes_cookie_t> }

/** Opcode for xcb_x_print_print_get_one_attributes. */
pub static XCB_X_PRINT_PRINT_GET_ONE_ATTRIBUTES : u8 = 19;
pub struct PrintGetOneAttributesReply { base:  base::Reply<xcb_x_print_print_get_one_attributes_reply_t> }
fn mk_reply_xcb_x_print_print_get_one_attributes_reply_t(reply:*mut xcb_x_print_print_get_one_attributes_reply_t) -> PrintGetOneAttributesReply { PrintGetOneAttributesReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_x_print_print_set_attributes. */
pub static XCB_X_PRINT_PRINT_SET_ATTRIBUTES : u8 = 18;
pub struct  PrintGetPageDimensionsCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_page_dimensions_cookie_t> }

/** Opcode for xcb_x_print_print_get_page_dimensions. */
pub static XCB_X_PRINT_PRINT_GET_PAGE_DIMENSIONS : u8 = 21;
pub struct PrintGetPageDimensionsReply { base:  base::Reply<xcb_x_print_print_get_page_dimensions_reply_t> }
fn mk_reply_xcb_x_print_print_get_page_dimensions_reply_t(reply:*mut xcb_x_print_print_get_page_dimensions_reply_t) -> PrintGetPageDimensionsReply { PrintGetPageDimensionsReply { base : base::mk_reply(reply) } }
pub struct  PrintQueryScreensCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_query_screens_cookie_t> }

/** Opcode for xcb_x_print_print_query_screens. */
pub static XCB_X_PRINT_PRINT_QUERY_SCREENS : u8 = 22;
pub struct  PrintSetImageResolutionCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_set_image_resolution_cookie_t> }

/** Opcode for xcb_x_print_print_set_image_resolution. */
pub static XCB_X_PRINT_PRINT_SET_IMAGE_RESOLUTION : u8 = 23;
pub struct PrintSetImageResolutionReply { base:  base::Reply<xcb_x_print_print_set_image_resolution_reply_t> }
fn mk_reply_xcb_x_print_print_set_image_resolution_reply_t(reply:*mut xcb_x_print_print_set_image_resolution_reply_t) -> PrintSetImageResolutionReply { PrintSetImageResolutionReply { base : base::mk_reply(reply) } }
pub struct  PrintGetImageResolutionCookie<'s> { pub base : base::Cookie<'s, xcb_x_print_print_get_image_resolution_cookie_t> }

/** Opcode for xcb_x_print_print_get_image_resolution. */
pub static XCB_X_PRINT_PRINT_GET_IMAGE_RESOLUTION : u8 = 24;
pub struct PrintGetImageResolutionReply { base:  base::Reply<xcb_x_print_print_get_image_resolution_reply_t> }
fn mk_reply_xcb_x_print_print_get_image_resolution_reply_t(reply:*mut xcb_x_print_print_get_image_resolution_reply_t) -> PrintGetImageResolutionReply { PrintGetImageResolutionReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_x_print_notify. */
pub static XCB_X_PRINT_NOTIFY : u8 = 0;
pub struct NotifyEvent {pub base : base::Event<xcb_x_print_notify_event_t>}
/** Opcode for xcb_x_print_attribut_notify. */
pub static XCB_X_PRINT_ATTRIBUT_NOTIFY : u8 = 1;
pub struct AttributNotifyEvent {pub base : base::Event<xcb_x_print_attribut_notify_event_t>}
/** Opcode for xcb_x_print_bad_context. */
pub static XCB_X_PRINT_BAD_CONTEXT : u8 = 0;
pub struct BadContextError { pub base : base::Error<xcb_x_print_bad_context_error_t> }
/** Opcode for xcb_x_print_bad_sequence. */
pub static XCB_X_PRINT_BAD_SEQUENCE : u8 = 1;
pub struct BadSequenceError { pub base : base::Error<xcb_x_print_bad_sequence_error_t> }


impl Iterator for String8Iterator {
    type Item = String8;
    fn next(&mut self) -> Option<String8> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_x_print_string8_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_x_print_string8_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Printer {pub base : base::Struct<xcb_x_print_printer_t> }


impl Printer {
  pub fn name(&mut self) -> Vec<String8> {
    unsafe { accessor!(String8, xcb_x_print_printer_name_length, xcb_x_print_printer_name, self.base.strct) }
  }

  pub fn description(&mut self) -> Vec<String8> {
    unsafe { accessor!(String8, xcb_x_print_printer_description_length, xcb_x_print_printer_description, self.base.strct) }
  }

}

impl Iterator for PrinterIterator {
    type Item = Printer;
    fn next(&mut self) -> Option<Printer> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_x_print_printer_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_x_print_printer_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Pcontext = xcb_x_print_pcontext_t;


impl Iterator for PcontextIterator {
    type Item = Pcontext;
    fn next(&mut self) -> Option<Pcontext> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_x_print_pcontext_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_x_print_pcontext_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn PrintQueryVersion<'r> (c : &'r Connection) -> PrintQueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_query_version(c.get_raw_conn());
    PrintQueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintQueryVersionUnchecked<'r> (c : &'r Connection) -> PrintQueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_query_version_unchecked(c.get_raw_conn());
    PrintQueryVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintQueryVersionReply {
  pub fn major_version(&mut self) -> u16 {
    unsafe { accessor!(major_version -> u16, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u16 {
    unsafe { accessor!(minor_version -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintQueryVersionCookie<'s>, mk_reply_xcb_x_print_print_query_version_reply_t, PrintQueryVersionReply, xcb_x_print_print_query_version_reply);

pub fn PrintGetPrinterList<'r> (c : &'r Connection,
                            printer_name : &[String8],
                            locale : &[String8]) -> PrintGetPrinterListCookie<'r> {
  unsafe {
    let printer_name_len = printer_name.len();
    let printer_name_ptr = printer_name.as_ptr();
    let locale_len = locale.len();
    let locale_ptr = locale.as_ptr();
    let cookie = xcb_x_print_print_get_printer_list(c.get_raw_conn(),
        printer_name_len as u32, //1
        locale_len as u32, //2
        printer_name_ptr as *mut xcb_x_print_string8_t, //3
        locale_ptr as *mut xcb_x_print_string8_t); //4
    PrintGetPrinterListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetPrinterListUnchecked<'r> (c : &'r Connection,
                                     printer_name : &[String8],
                                     locale : &[String8]) -> PrintGetPrinterListCookie<'r> {
  unsafe {
    let printer_name_len = printer_name.len();
    let printer_name_ptr = printer_name.as_ptr();
    let locale_len = locale.len();
    let locale_ptr = locale.as_ptr();
    let cookie = xcb_x_print_print_get_printer_list_unchecked(c.get_raw_conn(),
        printer_name_len as u32, //1
        locale_len as u32, //2
        printer_name_ptr as *mut xcb_x_print_string8_t, //3
        locale_ptr as *mut xcb_x_print_string8_t); //4
    PrintGetPrinterListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetPrinterListReply {
  pub fn printers(&mut self) -> PrinterIterator {
    unsafe { accessor!(PrinterIterator, xcb_x_print_print_get_printer_list_printers_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetPrinterListCookie<'s>, mk_reply_xcb_x_print_print_get_printer_list_reply_t, PrintGetPrinterListReply, xcb_x_print_print_get_printer_list_reply);

pub fn PrintRehashPrinterListChecked<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_rehash_printer_list_checked(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintRehashPrinterList<'r> (c : &'r Connection) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_rehash_printer_list(c.get_raw_conn());
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateContextChecked<'r> (c : &'r Connection,
                             context_id : u32,
                             printerName : &[String8],
                             locale : &[String8]) -> base::VoidCookie<'r> {
  unsafe {
    let printerName_len = printerName.len();
    let printerName_ptr = printerName.as_ptr();
    let locale_len = locale.len();
    let locale_ptr = locale.as_ptr();
    let cookie = xcb_x_print_create_context_checked(c.get_raw_conn(),
        context_id as u32, //1
        printerName_len as u32, //2
        locale_len as u32, //3
        printerName_ptr as *mut xcb_x_print_string8_t, //4
        locale_ptr as *mut xcb_x_print_string8_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateContext<'r> (c : &'r Connection,
                      context_id : u32,
                      printerName : &[String8],
                      locale : &[String8]) -> base::VoidCookie<'r> {
  unsafe {
    let printerName_len = printerName.len();
    let printerName_ptr = printerName.as_ptr();
    let locale_len = locale.len();
    let locale_ptr = locale.as_ptr();
    let cookie = xcb_x_print_create_context(c.get_raw_conn(),
        context_id as u32, //1
        printerName_len as u32, //2
        locale_len as u32, //3
        printerName_ptr as *mut xcb_x_print_string8_t, //4
        locale_ptr as *mut xcb_x_print_string8_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintSetContextChecked<'r> (c : &'r Connection,
                               context : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_set_context_checked(c.get_raw_conn(),
        context as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintSetContext<'r> (c : &'r Connection,
                        context : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_set_context(c.get_raw_conn(),
        context as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetContext<'r> (c : &'r Connection) -> PrintGetContextCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_context(c.get_raw_conn());
    PrintGetContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetContextUnchecked<'r> (c : &'r Connection) -> PrintGetContextCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_context_unchecked(c.get_raw_conn());
    PrintGetContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetContextReply {
  pub fn context(&mut self) -> u32 {
    unsafe { accessor!(context -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetContextCookie<'s>, mk_reply_xcb_x_print_print_get_context_reply_t, PrintGetContextReply, xcb_x_print_print_get_context_reply);

pub fn PrintDestroyContextChecked<'r> (c : &'r Connection,
                                   context : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_destroy_context_checked(c.get_raw_conn(),
        context as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintDestroyContext<'r> (c : &'r Connection,
                            context : u32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_destroy_context(c.get_raw_conn(),
        context as u32); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetScreenOfContext<'r> (c : &'r Connection) -> PrintGetScreenOfContextCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_screen_of_context(c.get_raw_conn());
    PrintGetScreenOfContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetScreenOfContextUnchecked<'r> (c : &'r Connection) -> PrintGetScreenOfContextCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_screen_of_context_unchecked(c.get_raw_conn());
    PrintGetScreenOfContextCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetScreenOfContextReply {
  pub fn root(&mut self) -> xproto::Window {
    unsafe { accessor!(root -> xproto::Window, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetScreenOfContextCookie<'s>, mk_reply_xcb_x_print_print_get_screen_of_context_reply_t, PrintGetScreenOfContextReply, xcb_x_print_print_get_screen_of_context_reply);

pub fn PrintStartJobChecked<'r> (c : &'r Connection,
                             output_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_start_job_checked(c.get_raw_conn(),
        output_mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintStartJob<'r> (c : &'r Connection,
                      output_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_start_job(c.get_raw_conn(),
        output_mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintEndJobChecked<'r> (c : &'r Connection,
                           cancel : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_end_job_checked(c.get_raw_conn(),
        cancel as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintEndJob<'r> (c : &'r Connection,
                    cancel : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_end_job(c.get_raw_conn(),
        cancel as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintStartDocChecked<'r> (c : &'r Connection,
                             driver_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_start_doc_checked(c.get_raw_conn(),
        driver_mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintStartDoc<'r> (c : &'r Connection,
                      driver_mode : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_start_doc(c.get_raw_conn(),
        driver_mode as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintEndDocChecked<'r> (c : &'r Connection,
                           cancel : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_end_doc_checked(c.get_raw_conn(),
        cancel as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintEndDoc<'r> (c : &'r Connection,
                    cancel : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_end_doc(c.get_raw_conn(),
        cancel as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintPutDocumentDataChecked<'r> (c : &'r Connection,
                                    drawable : xproto::Drawable,
                                    len_fmt : u16,
                                    len_options : u16,
                                    data : &[u8],
                                    doc_format : &[String8],
                                    options : &[String8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let doc_format_len = doc_format.len();
    let doc_format_ptr = doc_format.as_ptr();
    let options_len = options.len();
    let options_ptr = options.as_ptr();
    let cookie = xcb_x_print_print_put_document_data_checked(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        data_len as u32, //2
        len_fmt as u16, //3
        len_options as u16, //4
        data_ptr as *mut u8, //5
        doc_format_len as u32, //6
        doc_format_ptr as *mut xcb_x_print_string8_t, //7
        options_len as u32, //8
        options_ptr as *mut xcb_x_print_string8_t); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintPutDocumentData<'r> (c : &'r Connection,
                             drawable : xproto::Drawable,
                             len_fmt : u16,
                             len_options : u16,
                             data : &[u8],
                             doc_format : &[String8],
                             options : &[String8]) -> base::VoidCookie<'r> {
  unsafe {
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let doc_format_len = doc_format.len();
    let doc_format_ptr = doc_format.as_ptr();
    let options_len = options.len();
    let options_ptr = options.as_ptr();
    let cookie = xcb_x_print_print_put_document_data(c.get_raw_conn(),
        drawable as ffi::xproto::xcb_drawable_t, //1
        data_len as u32, //2
        len_fmt as u16, //3
        len_options as u16, //4
        data_ptr as *mut u8, //5
        doc_format_len as u32, //6
        doc_format_ptr as *mut xcb_x_print_string8_t, //7
        options_len as u32, //8
        options_ptr as *mut xcb_x_print_string8_t); //9
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct PrintGetDocumentDataReply { base:  base::Reply<xcb_x_print_print_get_document_data_reply_t> }
fn mk_reply_xcb_x_print_print_get_document_data_reply_t(reply:*mut xcb_x_print_print_get_document_data_reply_t) -> PrintGetDocumentDataReply { PrintGetDocumentDataReply { base : base::mk_reply(reply) } }
pub fn PrintGetDocumentData<'r> (c : &'r Connection,
                             context : Pcontext,
                             max_bytes : u32) -> PrintGetDocumentDataCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_document_data(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        max_bytes as u32); //2
    PrintGetDocumentDataCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetDocumentDataUnchecked<'r> (c : &'r Connection,
                                      context : Pcontext,
                                      max_bytes : u32) -> PrintGetDocumentDataCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_document_data_unchecked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        max_bytes as u32); //2
    PrintGetDocumentDataCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetDocumentDataReply {
  pub fn status_code(&mut self) -> u32 {
    unsafe { accessor!(status_code -> u32, (*self.base.reply)) }
  }

  pub fn finished_flag(&mut self) -> u32 {
    unsafe { accessor!(finished_flag -> u32, (*self.base.reply)) }
  }

  pub fn data(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_x_print_print_get_document_data_data_length, xcb_x_print_print_get_document_data_data, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetDocumentDataCookie<'s>, mk_reply_xcb_x_print_print_get_document_data_reply_t, PrintGetDocumentDataReply, xcb_x_print_print_get_document_data_reply);

pub fn PrintStartPageChecked<'r> (c : &'r Connection,
                              window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_start_page_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintStartPage<'r> (c : &'r Connection,
                       window : xproto::Window) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_start_page(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintEndPageChecked<'r> (c : &'r Connection,
                            cancel : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_end_page_checked(c.get_raw_conn(),
        cancel as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintEndPage<'r> (c : &'r Connection,
                     cancel : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_end_page(c.get_raw_conn(),
        cancel as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintSelectInputChecked<'r> (c : &'r Connection,
                                context : Pcontext,
                                event_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut event_list_copy = event_list.to_vec();
    let (event_list_mask, event_list_vec) = pack_bitfield(&mut event_list_copy);
    let event_list_ptr = event_list_vec.as_ptr();
    let cookie = xcb_x_print_print_select_input_checked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        event_list_mask as u32, //2
        event_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintSelectInput<'r> (c : &'r Connection,
                         context : Pcontext,
                         event_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut event_list_copy = event_list.to_vec();
    let (event_list_mask, event_list_vec) = pack_bitfield(&mut event_list_copy);
    let event_list_ptr = event_list_vec.as_ptr();
    let cookie = xcb_x_print_print_select_input(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        event_list_mask as u32, //2
        event_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct PrintInputSelectedReply { base:  base::Reply<xcb_x_print_print_input_selected_reply_t> }
fn mk_reply_xcb_x_print_print_input_selected_reply_t(reply:*mut xcb_x_print_print_input_selected_reply_t) -> PrintInputSelectedReply { PrintInputSelectedReply { base : base::mk_reply(reply) } }
pub fn PrintInputSelected<'r> (c : &'r Connection,
                           context : Pcontext) -> PrintInputSelectedCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_input_selected(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t); //1
    PrintInputSelectedCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintInputSelectedUnchecked<'r> (c : &'r Connection,
                                    context : Pcontext) -> PrintInputSelectedCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_input_selected_unchecked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t); //1
    PrintInputSelectedCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintInputSelectedReply {
  pub fn event_list(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_x_print_print_input_selected_event_list_length, xcb_x_print_print_input_selected_event_list, (*self.base.reply)) }
  }

  pub fn all_events_list(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_x_print_print_input_selected_all_events_list_length, xcb_x_print_print_input_selected_all_events_list, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintInputSelectedCookie<'s>, mk_reply_xcb_x_print_print_input_selected_reply_t, PrintInputSelectedReply, xcb_x_print_print_input_selected_reply);

pub fn PrintGetAttributes<'r> (c : &'r Connection,
                           context : Pcontext,
                           pool : u8) -> PrintGetAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_attributes(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        pool as u8); //2
    PrintGetAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetAttributesUnchecked<'r> (c : &'r Connection,
                                    context : Pcontext,
                                    pool : u8) -> PrintGetAttributesCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_attributes_unchecked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        pool as u8); //2
    PrintGetAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetAttributesReply {
  pub fn stringLen(&mut self) -> u32 {
    unsafe { accessor!(stringLen -> u32, (*self.base.reply)) }
  }

  pub fn attributes(&mut self) -> String8 {
    unsafe { accessor!(attributes -> String8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetAttributesCookie<'s>, mk_reply_xcb_x_print_print_get_attributes_reply_t, PrintGetAttributesReply, xcb_x_print_print_get_attributes_reply);

pub fn PrintGetOneAttributes<'r> (c : &'r Connection,
                              context : Pcontext,
                              pool : u8,
                              name : &[String8]) -> PrintGetOneAttributesCookie<'r> {
  unsafe {
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_x_print_print_get_one_attributes(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        name_len as u32, //2
        pool as u8, //3
        name_ptr as *mut xcb_x_print_string8_t); //4
    PrintGetOneAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetOneAttributesUnchecked<'r> (c : &'r Connection,
                                       context : Pcontext,
                                       pool : u8,
                                       name : &[String8]) -> PrintGetOneAttributesCookie<'r> {
  unsafe {
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_x_print_print_get_one_attributes_unchecked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        name_len as u32, //2
        pool as u8, //3
        name_ptr as *mut xcb_x_print_string8_t); //4
    PrintGetOneAttributesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetOneAttributesReply {
  pub fn value(&mut self) -> Vec<String8> {
    unsafe { accessor!(String8, xcb_x_print_print_get_one_attributes_value_length, xcb_x_print_print_get_one_attributes_value, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetOneAttributesCookie<'s>, mk_reply_xcb_x_print_print_get_one_attributes_reply_t, PrintGetOneAttributesReply, xcb_x_print_print_get_one_attributes_reply);

pub fn PrintSetAttributesChecked<'r> (c : &'r Connection,
                                  context : Pcontext,
                                  stringLen : u32,
                                  pool : u8,
                                  rule : u8,
                                  attributes : &[String8]) -> base::VoidCookie<'r> {
  unsafe {
    let attributes_len = attributes.len();
    let attributes_ptr = attributes.as_ptr();
    let cookie = xcb_x_print_print_set_attributes_checked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        stringLen as u32, //2
        pool as u8, //3
        rule as u8, //4
        attributes_len as u32, //5
        attributes_ptr as *mut xcb_x_print_string8_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn PrintSetAttributes<'r> (c : &'r Connection,
                           context : Pcontext,
                           stringLen : u32,
                           pool : u8,
                           rule : u8,
                           attributes : &[String8]) -> base::VoidCookie<'r> {
  unsafe {
    let attributes_len = attributes.len();
    let attributes_ptr = attributes.as_ptr();
    let cookie = xcb_x_print_print_set_attributes(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        stringLen as u32, //2
        pool as u8, //3
        rule as u8, //4
        attributes_len as u32, //5
        attributes_ptr as *mut xcb_x_print_string8_t); //6
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetPageDimensions<'r> (c : &'r Connection,
                               context : Pcontext) -> PrintGetPageDimensionsCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_page_dimensions(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t); //1
    PrintGetPageDimensionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetPageDimensionsUnchecked<'r> (c : &'r Connection,
                                        context : Pcontext) -> PrintGetPageDimensionsCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_page_dimensions_unchecked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t); //1
    PrintGetPageDimensionsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetPageDimensionsReply {
  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, (*self.base.reply)) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, (*self.base.reply)) }
  }

  pub fn offset_x(&mut self) -> u16 {
    unsafe { accessor!(offset_x -> u16, (*self.base.reply)) }
  }

  pub fn offset_y(&mut self) -> u16 {
    unsafe { accessor!(offset_y -> u16, (*self.base.reply)) }
  }

  pub fn reproducible_width(&mut self) -> u16 {
    unsafe { accessor!(reproducible_width -> u16, (*self.base.reply)) }
  }

  pub fn reproducible_height(&mut self) -> u16 {
    unsafe { accessor!(reproducible_height -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetPageDimensionsCookie<'s>, mk_reply_xcb_x_print_print_get_page_dimensions_reply_t, PrintGetPageDimensionsReply, xcb_x_print_print_get_page_dimensions_reply);

pub struct PrintQueryScreensReply { base:  base::Reply<xcb_x_print_print_query_screens_reply_t> }
fn mk_reply_xcb_x_print_print_query_screens_reply_t(reply:*mut xcb_x_print_print_query_screens_reply_t) -> PrintQueryScreensReply { PrintQueryScreensReply { base : base::mk_reply(reply) } }
pub fn PrintQueryScreens<'r> (c : &'r Connection) -> PrintQueryScreensCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_query_screens(c.get_raw_conn());
    PrintQueryScreensCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintQueryScreensUnchecked<'r> (c : &'r Connection) -> PrintQueryScreensCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_query_screens_unchecked(c.get_raw_conn());
    PrintQueryScreensCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintQueryScreensReply {
  pub fn roots(&mut self) -> Vec<xproto::Window> {
    unsafe { accessor!(xproto::Window, xcb_x_print_print_query_screens_roots_length, xcb_x_print_print_query_screens_roots, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintQueryScreensCookie<'s>, mk_reply_xcb_x_print_print_query_screens_reply_t, PrintQueryScreensReply, xcb_x_print_print_query_screens_reply);

pub fn PrintSetImageResolution<'r> (c : &'r Connection,
                                context : Pcontext,
                                image_resolution : u16) -> PrintSetImageResolutionCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_set_image_resolution(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        image_resolution as u16); //2
    PrintSetImageResolutionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintSetImageResolutionUnchecked<'r> (c : &'r Connection,
                                         context : Pcontext,
                                         image_resolution : u16) -> PrintSetImageResolutionCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_set_image_resolution_unchecked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t, //1
        image_resolution as u16); //2
    PrintSetImageResolutionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintSetImageResolutionReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

  pub fn previous_resolutions(&mut self) -> u16 {
    unsafe { accessor!(previous_resolutions -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintSetImageResolutionCookie<'s>, mk_reply_xcb_x_print_print_set_image_resolution_reply_t, PrintSetImageResolutionReply, xcb_x_print_print_set_image_resolution_reply);

pub fn PrintGetImageResolution<'r> (c : &'r Connection,
                                context : Pcontext) -> PrintGetImageResolutionCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_image_resolution(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t); //1
    PrintGetImageResolutionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn PrintGetImageResolutionUnchecked<'r> (c : &'r Connection,
                                         context : Pcontext) -> PrintGetImageResolutionCookie<'r> {
  unsafe {
    let cookie = xcb_x_print_print_get_image_resolution_unchecked(c.get_raw_conn(),
        context as xcb_x_print_pcontext_t); //1
    PrintGetImageResolutionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl PrintGetImageResolutionReply {
  pub fn image_resolution(&mut self) -> u16 {
    unsafe { accessor!(image_resolution -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(PrintGetImageResolutionCookie<'s>, mk_reply_xcb_x_print_print_get_image_resolution_reply_t, PrintGetImageResolutionReply, xcb_x_print_print_get_image_resolution_reply);


impl NotifyEvent {
  pub fn detail(&mut self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.base.event)) }
  }

  pub fn context(&mut self) -> Pcontext {
    unsafe { accessor!(context -> Pcontext, (*self.base.event)) }
  }

  pub fn cancel(&mut self) -> u8 {
    unsafe { accessor!(cancel -> u8, (*self.base.event)) }
  }

  pub fn new(detail : u8,
         context : Pcontext,
         cancel : u8) -> NotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_x_print_notify_event_t;
      (*raw).detail = detail;
      (*raw).context = context;
      (*raw).cancel = cancel;
      NotifyEvent { base : Event { event : raw as *mut xcb_x_print_notify_event_t }}
    }
  }
}

impl AttributNotifyEvent {
  pub fn detail(&mut self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.base.event)) }
  }

  pub fn context(&mut self) -> Pcontext {
    unsafe { accessor!(context -> Pcontext, (*self.base.event)) }
  }

  pub fn new(detail : u8,
         context : Pcontext) -> AttributNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_x_print_attribut_notify_event_t;
      (*raw).detail = detail;
      (*raw).context = context;
      AttributNotifyEvent { base : Event { event : raw as *mut xcb_x_print_attribut_notify_event_t }}
    }
  }
}
