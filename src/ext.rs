use crate::base::{Connection, Reply};
use crate::ffi::{
    xcb_connection_t, xcb_extension_t, xcb_get_extension_data, xcb_prefetch_extension_data,
};
use crate::x;

use std::fmt;
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Extension {
    BigRequests,
    XcMisc,

    #[cfg(feature = "composite")]
    Composite,

    #[cfg(feature = "damage")]
    Damage,

    #[cfg(feature = "dpms")]
    Dpms,

    #[cfg(feature = "dri2")]
    Dri2,

    #[cfg(feature = "dri3")]
    Dri3,

    #[cfg(feature = "ge")]
    GenericEvent,

    #[cfg(feature = "glx")]
    Glx,

    #[cfg(feature = "present")]
    Present,

    #[cfg(feature = "randr")]
    RandR,

    #[cfg(feature = "record")]
    Record,

    #[cfg(feature = "render")]
    Render,

    #[cfg(feature = "res")]
    Res,

    #[cfg(feature = "screensaver")]
    ScreenSaver,

    #[cfg(feature = "shape")]
    Shape,

    #[cfg(feature = "shm")]
    Shm,

    #[cfg(feature = "sync")]
    Sync,

    #[cfg(feature = "xevie")]
    Xevie,

    #[cfg(feature = "xf86dri")]
    Xf86Dri,

    #[cfg(feature = "xf86vidmode")]
    Xf86VidMode,

    #[cfg(feature = "xfixes")]
    XFixes,

    #[cfg(feature = "xinerama")]
    Xinerama,

    #[cfg(feature = "xinput")]
    Input,

    #[cfg(feature = "xkb")]
    Xkb,

    #[cfg(feature = "xprint")]
    XPrint,

    #[cfg(feature = "xselinux")]
    SeLinux,

    #[cfg(feature = "xtest")]
    Test,

    #[cfg(feature = "xv")]
    Xv,

    #[cfg(feature = "xvmc")]
    XvMc,
}

impl Extension {
    fn xname(&self) -> &'static str {
        match self {
            Extension::BigRequests => crate::bigreq::XNAME,
            Extension::XcMisc => crate::xc_misc::XNAME,

            #[cfg(feature = "composite")]
            Extension::Composite => crate::composite::XNAME,

            #[cfg(feature = "damage")]
            Extension::Damage => crate::damage::XNAME,

            #[cfg(feature = "dpms")]
            Extension::Dpms => crate::dpms::XNAME,

            #[cfg(feature = "dri2")]
            Extension::Dri2 => crate::dri2::XNAME,

            #[cfg(feature = "dri3")]
            Extension::Dri3 => crate::dri3::XNAME,

            #[cfg(feature = "ge")]
            Extension::GenericEvent => crate::ge::XNAME,

            #[cfg(feature = "glx")]
            Extension::Glx => crate::glx::XNAME,

            #[cfg(feature = "present")]
            Extension::Present => crate::present::XNAME,

            #[cfg(feature = "randr")]
            Extension::RandR => crate::randr::XNAME,

            #[cfg(feature = "record")]
            Extension::Record => crate::record::XNAME,

            #[cfg(feature = "render")]
            Extension::Render => crate::render::XNAME,

            #[cfg(feature = "res")]
            Extension::Res => crate::res::XNAME,

            #[cfg(feature = "screensaver")]
            Extension::ScreenSaver => crate::screensaver::XNAME,

            #[cfg(feature = "shape")]
            Extension::Shape => crate::shape::XNAME,

            #[cfg(feature = "shm")]
            Extension::Shm => crate::shm::XNAME,

            #[cfg(feature = "sync")]
            Extension::Sync => crate::sync::XNAME,

            #[cfg(feature = "xevie")]
            Extension::Xevie => crate::xevie::XNAME,

            #[cfg(feature = "xf86dri")]
            Extension::Xf86Dri => crate::xf86dri::XNAME,

            #[cfg(feature = "xf86vidmode")]
            Extension::Xf86VidMode => crate::xf86vidmode::XNAME,

            #[cfg(feature = "xfixes")]
            Extension::XFixes => crate::xfixes::XNAME,

            #[cfg(feature = "xinerama")]
            Extension::Xinerama => crate::xinerama::XNAME,

            #[cfg(feature = "xinput")]
            Extension::Input => crate::xinput::XNAME,

            #[cfg(feature = "xkb")]
            Extension::Xkb => crate::xkb::XNAME,

            #[cfg(feature = "xprint")]
            Extension::XPrint => crate::xprint::XNAME,

            #[cfg(feature = "xselinux")]
            Extension::SeLinux => crate::xselinux::XNAME,

            #[cfg(feature = "xtest")]
            Extension::Test => crate::xtest::XNAME,

            #[cfg(feature = "xv")]
            Extension::Xv => crate::xv::XNAME,

            #[cfg(feature = "xvmc")]
            Extension::XvMc => crate::xvmc::XNAME,
        }
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.xname().fmt(f)
    }
}

/// Extension data as returned by each extensions `get_extension_data`.
/// See [crate::bigreq::get_extension_data] as example.
pub struct ExtensionData {
    pub ext: Extension,
    pub major_opcode: u8,
    pub first_event: u8,
    pub first_error: u8,
}

pub(crate) fn cache_extensions_data(
    conn: *mut xcb_connection_t,
    mandatory: &[Extension],
    optional: &[Extension],
) -> (Vec<EventExtensionData>, Vec<ErrorExtensionData>) {
    unsafe {
        for ext in mandatory {
            let ext_id = get_extension_id(*ext);
            xcb_prefetch_extension_data(conn, ext_id as *mut _);
        }
        for ext in optional {
            let ext_id = get_extension_id(*ext);
            xcb_prefetch_extension_data(conn, ext_id as *mut _);
        }

        let mut event_data = Vec::new();
        let mut error_data = Vec::new();

        for ext in mandatory {
            let ext_id = get_extension_id(*ext);
            let raw = xcb_get_extension_data(conn, ext_id as *mut _);
            let reply = x::QueryExtensionReply::from_raw(raw);

            assert!(
                reply.present(),
                "mandatory extension {} is not present on this system",
                ext
            );
            event_data.push(EventExtensionData {
                ext: *ext,
                major_opcode: reply.major_opcode(),
                first_event: reply.first_event(),
            });
            error_data.push(ErrorExtensionData {
                ext: *ext,
                first_error: reply.first_error(),
            });
            mem::forget(reply);
        }

        for ext in optional {
            let ext_id = get_extension_id(*ext);
            let raw = xcb_get_extension_data(conn, ext_id as *mut _);
            let reply = x::QueryExtensionReply::from_raw(raw);

            if !reply.present() {
                mem::forget(reply);
                continue;
            }

            event_data.push(EventExtensionData {
                ext: *ext,
                major_opcode: reply.major_opcode(),
                first_event: reply.first_event(),
            });
            error_data.push(ErrorExtensionData {
                ext: *ext,
                first_error: reply.first_error(),
            });
            mem::forget(reply);
        }

        event_data.sort_by(|a, b| b.first_event.cmp(&a.first_event));
        error_data.sort_by(|a, b| b.first_error.cmp(&a.first_error));

        (event_data, error_data)
    }
}

unsafe fn get_extension_id(ext: Extension) -> &'static mut xcb_extension_t {
    match ext {
        Extension::BigRequests => &mut crate::bigreq::FFI_EXT,
        Extension::XcMisc => &mut crate::xc_misc::FFI_EXT,

        #[cfg(feature = "composite")]
        Extension::Composite => &mut crate::composite::FFI_EXT,

        #[cfg(feature = "damage")]
        Extension::Damage => &mut crate::damage::FFI_EXT,

        #[cfg(feature = "dpms")]
        Extension::Dpms => &mut crate::dpms::FFI_EXT,

        #[cfg(feature = "dri2")]
        Extension::Dri2 => &mut crate::dri2::FFI_EXT,

        #[cfg(feature = "dri3")]
        Extension::Dri3 => &mut crate::dri3::FFI_EXT,

        #[cfg(feature = "ge")]
        Extension::GenericEvent => &mut crate::ge::FFI_EXT,

        #[cfg(feature = "glx")]
        Extension::Glx => &mut crate::glx::FFI_EXT,

        #[cfg(feature = "present")]
        Extension::Present => &mut crate::present::FFI_EXT,

        #[cfg(feature = "randr")]
        Extension::RandR => &mut crate::randr::FFI_EXT,

        #[cfg(feature = "record")]
        Extension::Record => &mut crate::record::FFI_EXT,

        #[cfg(feature = "render")]
        Extension::Render => &mut crate::render::FFI_EXT,

        #[cfg(feature = "res")]
        Extension::Res => &mut crate::res::FFI_EXT,

        #[cfg(feature = "screensaver")]
        Extension::ScreenSaver => &mut crate::screensaver::FFI_EXT,

        #[cfg(feature = "shape")]
        Extension::Shape => &mut crate::shape::FFI_EXT,

        #[cfg(feature = "shm")]
        Extension::Shm => &mut crate::shm::FFI_EXT,

        #[cfg(feature = "sync")]
        Extension::Sync => &mut crate::sync::FFI_EXT,

        #[cfg(feature = "xevie")]
        Extension::Xevie => &mut crate::xevie::FFI_EXT,

        #[cfg(feature = "xf86dri")]
        Extension::Xf86Dri => &mut crate::xf86dri::FFI_EXT,

        #[cfg(feature = "xf86vidmode")]
        Extension::Xf86VidMode => &mut crate::xf86vidmode::FFI_EXT,

        #[cfg(feature = "xfixes")]
        Extension::XFixes => &mut crate::xfixes::FFI_EXT,

        #[cfg(feature = "xinerama")]
        Extension::Xinerama => &mut crate::xinerama::FFI_EXT,

        #[cfg(feature = "xinput")]
        Extension::Input => &mut crate::xinput::FFI_EXT,

        #[cfg(feature = "xkb")]
        Extension::Xkb => &mut crate::xkb::FFI_EXT,

        #[cfg(feature = "xprint")]
        Extension::XPrint => &mut crate::xprint::FFI_EXT,

        #[cfg(feature = "xselinux")]
        Extension::SeLinux => &mut crate::xselinux::FFI_EXT,

        #[cfg(feature = "xtest")]
        Extension::Test => &mut crate::xtest::FFI_EXT,

        #[cfg(feature = "xv")]
        Extension::Xv => &mut crate::xv::FFI_EXT,

        #[cfg(feature = "xvmc")]
        Extension::XvMc => &mut crate::xvmc::FFI_EXT,
    }
}

pub(crate) struct EventExtensionData {
    pub(crate) ext: Extension,
    pub(crate) major_opcode: u8,
    pub(crate) first_event: u8,
}

pub(crate) struct ErrorExtensionData {
    pub(crate) ext: Extension,
    pub(crate) first_error: u8,
}
