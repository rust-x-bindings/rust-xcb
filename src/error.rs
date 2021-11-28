use crate::base::ResolveWireError;
use crate::ext::{Extension, ExtensionData};
use crate::ffi::*;
use crate::x;

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
#[derive(Debug)]
pub enum ProtocolError {
    /// The error is from the core X protocol.
    X(x::Error),

    #[cfg(feature = "damage")]
    /// The error is issued from the `DAMAGE` extension.
    Damage(damage::Error),

    #[cfg(feature = "glx")]
    /// The error is issued from the `GLX` extension.
    Glx(glx::Error),

    #[cfg(feature = "randr")]
    /// The error is issued from the `RANDR` extension.
    RandR(randr::Error),

    #[cfg(feature = "render")]
    /// The error is issued from the `RENDER` extension.
    Render(render::Error),

    #[cfg(feature = "shm")]
    /// The error is issued from the `MIT-SHM` extension.
    Shm(shm::Error),

    #[cfg(feature = "sync")]
    /// The error is issued from the `SYNC` extension.
    Sync(sync::Error),

    #[cfg(feature = "xf86vidmode")]
    /// The error is issued from the `XFree86-VidModeExtension` extension.
    Xf86VidMode(xf86vidmode::Error),

    #[cfg(feature = "xfixes")]
    /// The error is issued from the `XFIXES` extension.
    XFixes(xfixes::Error),

    #[cfg(feature = "xinput")]
    /// The error is issued from the `XInputExtension` extension.
    Input(xinput::Error),

    #[cfg(feature = "xkb")]
    /// The error is issued from the `XKEYBOARD` extension.
    Xkb(xkb::Error),

    #[cfg(feature = "xprint")]
    /// The error is issued from the `XpExtension` extension.
    XPrint(xprint::Error),

    #[cfg(feature = "xv")]
    /// The error is issued from the `XVideo` extension.
    Xv(xv::Error),
}

pub(crate) unsafe fn resolve_error(
    error: *mut xcb_generic_error_t,
    extension_data: &[ExtensionData],
) -> ProtocolError {
    debug_assert!(!error.is_null());

    let error_code = (*error).error_code;

    let best = {
        let mut best: Option<&ExtensionData> = None;
        for data in extension_data {
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
        best
    };

    if let Some(ext_data) = best {
        match ext_data.ext {
            #[cfg(feature = "damage")]
            Extension::Damage => ProtocolError::Damage(damage::Error::resolve_wire_error(
                ext_data.first_error,
                error,
            )),

            #[cfg(feature = "glx")]
            Extension::Glx => {
                ProtocolError::Glx(glx::Error::resolve_wire_error(ext_data.first_error, error))
            }

            #[cfg(feature = "randr")]
            Extension::RandR => ProtocolError::RandR(randr::Error::resolve_wire_error(
                ext_data.first_error,
                error,
            )),

            #[cfg(feature = "shm")]
            Extension::Shm => {
                ProtocolError::Shm(shm::Error::resolve_wire_error(ext_data.first_error, error))
            }

            #[cfg(feature = "sync")]
            Extension::Sync => {
                ProtocolError::Sync(sync::Error::resolve_wire_error(ext_data.first_error, error))
            }

            #[cfg(feature = "xf86vidmode")]
            Extension::Xf86VidMode => ProtocolError::Xf86VidMode(
                xf86vidmode::Error::resolve_wire_error(ext_data.first_error, error),
            ),

            #[cfg(feature = "xfixes")]
            Extension::XFixes => ProtocolError::XFixes(xfixes::Error::resolve_wire_error(
                ext_data.first_error,
                error,
            )),

            #[cfg(feature = "xinput")]
            Extension::Input => ProtocolError::Input(xinput::Error::resolve_wire_error(
                ext_data.first_error,
                error,
            )),

            #[cfg(feature = "xkb")]
            Extension::Xkb => {
                ProtocolError::Xkb(xkb::Error::resolve_wire_error(ext_data.first_error, error))
            }

            #[cfg(feature = "xprint")]
            Extension::XPrint => ProtocolError::XPrint(xprint::Error::resolve_wire_error(
                ext_data.first_error,
                error,
            )),

            #[cfg(feature = "xv")]
            Extension::Xv => {
                ProtocolError::Xv(xv::Error::resolve_wire_error(ext_data.first_error, error))
            }

            _ => unreachable!("Could not match extension event"),
        }
    } else {
        ProtocolError::X(x::Error::resolve_wire_error(0, error))
    }
}
