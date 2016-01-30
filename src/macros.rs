
use std::ptr;

macro_rules! impl_reply_cookie {
    ($cookie:ty, $mk_func:ident, $reply:ty, $func:ident) => (
        impl<'s> base::ReplyCookie<$reply> for $cookie {
            fn get_reply(&self) -> Result<$reply,base::GenericError> {
                use ffi;
                unsafe {
                    let mut err : *mut ffi::base::xcb_generic_error_t = std::ptr::null_mut();
                    let reply = if self.base.checked {
                        $func(self.base.conn, self.base.cookie, &mut err)
                    } else {
                        $func(self.base.conn, self.base.cookie, std::ptr::null_mut())
                    };
                    if err.is_null() {
                        return Ok($mk_func(reply));
                    } else {
                        ::libc::free(reply as *mut ::libc::c_void);
                        return Err(base::GenericError { base : base::mk_error(err)});
                    }
                }
            }
        }
    )
}
