#[macro_escape];


macro_rules! impl_reply_cookie {
    ($cookie:ty, $crep:ty, $reply:ty, $func:ident) => (
        impl<'self> base::ReplyCookie<$crep> for $cookie {
            pub fn get_reply(&self) -> Result<$reply,base::GenericError> {
                use ll;
                unsafe {
                    let err : *ll::base::generic_error = ::std::ptr::null();
                    let reply = if self.checked {
                        $func(self.conn.get_raw_conn(), self.cookie, &err)
                    } else {
                        $func(self.conn.get_raw_conn(), self.cookie, ::std::ptr::null())
                    };
                    if err.is_null() {
                        return Ok(base::mk_reply(reply));
                    } else {
                        ::std::libc::free(reply as *::std::libc::c_void);
                        return Err(base::mk_error(err));
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
            let _len = $lenfn(&$fexpr);
            let _data = $datafn(&$fexpr);
            std::str::raw::from_buf_len(_data as *u8, _len as uint)
        }
    );
    ($ty:ty, $iterfn:ident, $fexpr:expr) => ( //Iterator
        unsafe {
            $iterfn(&$fexpr)
        }
    );
    ($ty:ty, $lenfn:ident, $datafn:ident, $fexpr:expr) => ( //list with fixed-size members
        unsafe {
            let _len = $lenfn(&$fexpr);
            let _data = $datafn(&$fexpr);
            std::vec::raw::from_buf_raw(_data, _len as uint)
        }
    );
}
