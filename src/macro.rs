#![macro_escape]


macro_rules! impl_reply_cookie {
    ($cookie:ty, $mk_func:ident, $reply:ty, $func:ident) => (
        impl<'s> base::ReplyCookie<$reply> for $cookie {
            fn get_reply(&self) -> Result<$reply,base::GenericError> {
                use ffi;
                unsafe {
                    let mut err : *mut ffi::base::generic_error = ::std::ptr::mut_null();
                    let reply = if self.base.checked {
                        $func(self.base.conn.get_raw_conn(), self.base.cookie, &mut err)
                    } else {
                        $func(self.base.conn.get_raw_conn(), self.base.cookie, ::std::ptr::mut_null())
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
            std::string::raw::from_buf_len(_data as *const u8, _len as uint)
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
            std::vec::raw::from_buf(&*_data, _len as uint)
        }
    );
}
