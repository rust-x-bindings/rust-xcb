
use std::ptr;

macro_rules! impl_reply_cookie {
    ($cookie:ty, $mk_func:ident, $reply:ty, $func:ident) => (
        impl<'s> base::ReplyCookie<$reply> for $cookie {
            fn get_reply(&self) -> Result<$reply,base::GenericError> {
                use ffi;
                unsafe {
                    let mut err : *mut ffi::base::xcb_generic_error_t = std::ptr::null_mut();
                    let reply = if self.base.checked {
                        $func(self.base.conn.get_raw_conn(), self.base.cookie, &mut err)
                    } else {
                        $func(self.base.conn.get_raw_conn(), self.base.cookie, std::ptr::null_mut())
                    };
                    if err.is_null() {
                        return Ok($mk_func(reply));
                    } else {
                        ::libc::free(reply as *mut ::libc::c_void);
                        return Err(GenericError { base : base::mk_error(err)});
                    }
                }
            }
        }
    )
}

macro_rules! accessor {
    ($fname:ident -> $ty:ty, $fexpr:expr) => ( //Basic access
        $fexpr.$fname as $ty
    );
    (str, $lenfn:ident, $datafn:ident, $fexpr:expr) => ( //String special case
        unsafe {
            let _len = $lenfn(&mut $fexpr);
            let _data = $datafn(&mut $fexpr);

            let _slice = std::slice::from_raw_parts(_data as *const u8, _len as usize);

            // should we check what comes from X server?
            std::str::from_utf8_unchecked(&_slice)
        }
    );
    ($ty:ty, $iterfn:ident, $fexpr:expr) => ( //Iterator
        unsafe {
            $iterfn(&mut $fexpr)
        }
    );
    ($ty:ty, $lenfn:ident, $datafn:ident, $fexpr:expr) => ( //list with fixed-size members
        unsafe {
            let _len = $lenfn(&mut $fexpr);
            let _data = $datafn(&mut $fexpr);
            std::slice::from_raw_parts(_data, _len as usize)
        }
    );
}
