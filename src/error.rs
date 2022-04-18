use crate::base::ResolveWireError;
use crate::ext::{Extension, ExtensionData};
use crate::ffi::*;
use crate::x;
use std::mem;

#[cfg(feature = "damage")]
use crate::damage;

#[cfg(feature = "glx")]
use crate::glx;

#[cfg(feature = "randr")]
use crate::randr;

#[cfg(feature = "render")]
use crate::render;

#[cfg(feature = "shm")]
use crate::shm;

#[cfg(feature = "sync")]
use crate::sync;

#[cfg(feature = "xf86vidmode")]
use crate::xf86vidmode;

#[cfg(feature = "xfixes")]
use crate::xfixes;

#[cfg(feature = "xinput")]
use crate::xinput;

#[cfg(feature = "xkb")]
use crate::xkb;

#[cfg(feature = "xprint")]
use crate::xprint;

#[cfg(feature = "xv")]
use crate::xv;

/// A protocol error issued from the X server
///
/// The second member is the name of the request that emitted the error (if any).
#[derive(Debug)]
pub enum ProtocolError {
    /// The error is from the core X protocol.
    X(x::Error, Option<&'static str>),

    #[cfg(feature = "damage")]
    /// The error is issued from the `DAMAGE` extension.
    Damage(damage::Error, Option<&'static str>),

    #[cfg(feature = "glx")]
    /// The error is issued from the `GLX` extension.
    Glx(glx::Error, Option<&'static str>),

    #[cfg(feature = "randr")]
    /// The error is issued from the `RANDR` extension.
    RandR(randr::Error, Option<&'static str>),

    #[cfg(feature = "render")]
    /// The error is issued from the `RENDER` extension.
    Render(render::Error, Option<&'static str>),

    #[cfg(feature = "shm")]
    /// The error is issued from the `MIT-SHM` extension.
    Shm(shm::Error, Option<&'static str>),

    #[cfg(feature = "sync")]
    /// The error is issued from the `SYNC` extension.
    Sync(sync::Error, Option<&'static str>),

    #[cfg(feature = "xf86vidmode")]
    /// The error is issued from the `XFree86-VidModeExtension` extension.
    Xf86VidMode(xf86vidmode::Error, Option<&'static str>),

    #[cfg(feature = "xfixes")]
    /// The error is issued from the `XFIXES` extension.
    XFixes(xfixes::Error, Option<&'static str>),

    #[cfg(feature = "xinput")]
    /// The error is issued from the `XInputExtension` extension.
    Input(xinput::Error, Option<&'static str>),

    #[cfg(feature = "xkb")]
    /// The error is issued from the `XKEYBOARD` extension.
    Xkb(xkb::Error, Option<&'static str>),

    #[cfg(feature = "xprint")]
    /// The error is issued from the `XpExtension` extension.
    XPrint(xprint::Error, Option<&'static str>),

    #[cfg(feature = "xv")]
    /// The error is issued from the `XVideo` extension.
    Xv(xv::Error, Option<&'static str>),
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for ProtocolError {}

pub(crate) unsafe fn resolve_error(
    error: *mut xcb_generic_error_t,
    extension_data: &[ExtensionData],
) -> ProtocolError {
    debug_assert!(!error.is_null());

    let error_code = (*error).error_code;
    let major_code = (*error).major_code;
    let minor_code = (*error).minor_code;

    let (best, emitting_ext) = {
        let mut best: Option<&ExtensionData> = None;
        let mut emitting_ext = None;
        for data in extension_data {
            if data.major_opcode == major_code {
                emitting_ext = Some(data.ext);
            }
            if error_code >= data.first_error {
                if let Some(ed) = best {
                    if data.first_error > ed.first_error {
                        best = Some(data);
                    }
                } else {
                    best = Some(data);
                }
            }
        }
        (best, emitting_ext)
    };

    let emitted_by = if let Some(ext) = emitting_ext {
        match ext {
            Extension::BigRequests => crate::bigreq::request_name(minor_code),
            Extension::XcMisc => crate::xc_misc::request_name(minor_code),

            #[cfg(feature = "composite")]
            Extension::Composite => crate::composite::request_name(minor_code),

            #[cfg(feature = "damage")]
            Extension::Damage => crate::damage::request_name(minor_code),

            #[cfg(feature = "dpms")]
            Extension::Dpms => crate::dpms::request_name(minor_code),

            #[cfg(feature = "dri2")]
            Extension::Dri2 => crate::dri2::request_name(minor_code),

            #[cfg(feature = "dri3")]
            Extension::Dri3 => crate::dri3::request_name(minor_code),

            #[cfg(feature = "ge")]
            Extension::GenericEvent => crate::ge::request_name(minor_code),

            #[cfg(feature = "glx")]
            Extension::Glx => crate::glx::request_name(minor_code),

            #[cfg(feature = "present")]
            Extension::Present => crate::present::request_name(minor_code),

            #[cfg(feature = "randr")]
            Extension::RandR => crate::randr::request_name(minor_code),

            #[cfg(feature = "record")]
            Extension::Record => crate::record::request_name(minor_code),

            #[cfg(feature = "render")]
            Extension::Render => crate::render::request_name(minor_code),

            #[cfg(feature = "res")]
            Extension::Res => crate::res::request_name(minor_code),

            #[cfg(feature = "screensaver")]
            Extension::ScreenSaver => crate::screensaver::request_name(minor_code),

            #[cfg(feature = "shape")]
            Extension::Shape => crate::shape::request_name(minor_code),

            #[cfg(feature = "shm")]
            Extension::Shm => crate::shm::request_name(minor_code),

            #[cfg(feature = "sync")]
            Extension::Sync => crate::sync::request_name(minor_code),

            #[cfg(feature = "xevie")]
            Extension::Xevie => crate::xevie::request_name(minor_code),

            #[cfg(feature = "xf86dri")]
            Extension::Xf86Dri => crate::xf86dri::request_name(minor_code),

            #[cfg(feature = "xf86vidmode")]
            Extension::Xf86VidMode => crate::xf86vidmode::request_name(minor_code),

            #[cfg(feature = "xfixes")]
            Extension::XFixes => crate::xfixes::request_name(minor_code),

            #[cfg(feature = "xinerama")]
            Extension::Xinerama => crate::xinerama::request_name(minor_code),

            #[cfg(feature = "xinput")]
            Extension::Input => crate::xinput::request_name(minor_code),

            #[cfg(feature = "xkb")]
            Extension::Xkb => crate::xkb::request_name(minor_code),

            #[cfg(feature = "xprint")]
            Extension::XPrint => crate::xprint::request_name(minor_code),

            #[cfg(feature = "xselinux")]
            Extension::SeLinux => crate::xselinux::request_name(minor_code),

            #[cfg(feature = "xtest")]
            Extension::Test => crate::xtest::request_name(minor_code),

            #[cfg(feature = "xv")]
            Extension::Xv => crate::xv::request_name(minor_code),

            #[cfg(feature = "xvmc")]
            Extension::XvMc => crate::xvmc::request_name(minor_code),
        }
    } else {
        crate::x::request_name(minor_code)
    };

    if let Some(ext_data) = best {
        match ext_data.ext {
            #[cfg(feature = "damage")]
            Extension::Damage => ProtocolError::Damage(
                damage::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "glx")]
            Extension::Glx => ProtocolError::Glx(
                glx::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "randr")]
            Extension::RandR => ProtocolError::RandR(
                randr::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "shm")]
            Extension::Shm => ProtocolError::Shm(
                shm::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "sync")]
            Extension::Sync => ProtocolError::Sync(
                sync::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "xf86vidmode")]
            Extension::Xf86VidMode => ProtocolError::Xf86VidMode(
                xf86vidmode::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "xfixes")]
            Extension::XFixes => ProtocolError::XFixes(
                xfixes::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "xinput")]
            Extension::Input => ProtocolError::Input(
                xinput::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "xkb")]
            Extension::Xkb => ProtocolError::Xkb(
                xkb::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "xprint")]
            Extension::XPrint => ProtocolError::XPrint(
                xprint::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            #[cfg(feature = "xv")]
            Extension::Xv => ProtocolError::Xv(
                xv::Error::resolve_wire_error(ext_data.first_error, error),
                emitted_by,
            ),

            _ => unreachable!("Could not match extension event"),
        }
    } else {
        ProtocolError::X(x::Error::resolve_wire_error(0, error), emitted_by)
    }
}

#[cfg(all(feature = "xinput", feature = "xkb"))]
#[test]
fn test_resolve_error() {
    // resolve a core error with core request
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 2,
        sequence: 12000,
        resource_id: 12,
        minor_code: 0,
        major_code: 4,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::X(x::Error::Value(_), Some(req_name)) if req_name == "x::DestroyWindow")
    );
    mem::forget(err);

    // resolve a core error with xinput request
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 2,
        sequence: 12000,
        resource_id: 12,
        minor_code: 4,
        major_code: 100,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [
        ExtensionData {
            ext: Extension::Input,
            major_opcode: 100,
            first_event: 50,
            first_error: 20,
        },
        ExtensionData {
            ext: Extension::Xkb,
            major_opcode: 200,
            first_event: 80,
            first_error: 40,
        },
    ];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::X(x::Error::Value(_), Some(req_name)) if req_name == "xinput::CloseDevice")
    );
    mem::forget(err);

    // resolve a xinput error with xinput request
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 22,
        sequence: 12000,
        resource_id: 12,
        minor_code: 4,
        major_code: 100,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [
        ExtensionData {
            ext: Extension::Input,
            major_opcode: 100,
            first_event: 50,
            first_error: 20,
        },
        ExtensionData {
            ext: Extension::Xkb,
            major_opcode: 200,
            first_event: 80,
            first_error: 40,
        },
    ];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::Input(xinput::Error::Mode(_), Some(req_name)) if req_name == "xinput::CloseDevice")
    );
    mem::forget(err);

    // same as previous, but reverse order of extension data
    let mut error = xcb_generic_error_t {
        response_type: 0,
        error_code: 22,
        sequence: 12000,
        resource_id: 12,
        minor_code: 4,
        major_code: 100,
        pad0: 0,
        pad: [0; 5],
        full_sequence: 12000,
    };
    let extension_data = [
        ExtensionData {
            ext: Extension::Input,
            major_opcode: 100,
            first_event: 50,
            first_error: 20,
        },
        ExtensionData {
            ext: Extension::Xkb,
            major_opcode: 200,
            first_event: 80,
            first_error: 40,
        },
    ];
    let err = unsafe { resolve_error(&mut error as *mut _, &extension_data) };
    assert!(
        matches!(err, ProtocolError::Input(xinput::Error::Mode(_), Some(req_name)) if req_name == "xinput::CloseDevice")
    );
    mem::forget(err);
}
