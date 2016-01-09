
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
            // std::string::raw::from_buf_len(_data as *const u8, _len as uint)
            // from_buf_len is deprecated
            // equivalent is from_raw_parts, but it seem to take ownership of the data
            // this code creates a vec, copies data into it, and move it into a string

            let mut vec = std::vec::Vec::with_capacity(_len as usize);
            let dst = vec.as_mut_ptr();
            std::ptr::copy_nonoverlapping(_data as *const u8, dst, _len as usize);
            vec.set_len(_len as usize);
            // would unchecked be safe here?
            std::string::String::from_utf8(vec).unwrap()
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
            std::vec::Vec::from_raw_parts(_data, _len as usize, _len as usize)
        }
    );
}
