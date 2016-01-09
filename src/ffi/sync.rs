/*
 * This file generated automatically from sync.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub const SYNC_MAJOR_VERSION : c_uint = 3;
pub const SYNC_MINOR_VERSION : c_uint = 1;

pub type xcb_sync_alarm_t = u32;
/**
 * @brief xcb_sync_alarm_iterator_t
 **/
#[repr(C)]
pub struct xcb_sync_alarm_iterator_t {
    pub data : *mut xcb_sync_alarm_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_sync_counter_t = u32;
/**
 * @brief xcb_sync_counter_iterator_t
 **/
#[repr(C)]
pub struct xcb_sync_counter_iterator_t {
    pub data : *mut xcb_sync_counter_t,
    pub rem  : c_int,
    pub index: c_int
}


pub type xcb_sync_fence_t = u32;
/**
 * @brief xcb_sync_fence_iterator_t
 **/
#[repr(C)]
pub struct xcb_sync_fence_iterator_t {
    pub data : *mut xcb_sync_fence_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_sync_int64_t {
     pub hi :   i32,
     pub lo :   u32
}

impl Copy for xcb_sync_int64_t {}
impl Clone for xcb_sync_int64_t {
    fn clone(&self) -> xcb_sync_int64_t { *self }
}
/**
 * @brief xcb_sync_int64_iterator_t
 **/
#[repr(C)]
pub struct xcb_sync_int64_iterator_t {
    pub data : *mut xcb_sync_int64_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_sync_systemcounter_t {
     pub counter :      xcb_sync_counter_t,
     pub resolution :   xcb_sync_int64_t,
     pub name_len :     u16
}

impl Copy for xcb_sync_systemcounter_t {}
impl Clone for xcb_sync_systemcounter_t {
    fn clone(&self) -> xcb_sync_systemcounter_t { *self }
}
/**
 * @brief xcb_sync_systemcounter_iterator_t
 **/
#[repr(C)]
pub struct xcb_sync_systemcounter_iterator_t {
    pub data : *mut xcb_sync_systemcounter_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_sync_trigger_t {
     pub counter :      xcb_sync_counter_t,
     pub wait_type :    u32,
     pub wait_value :   xcb_sync_int64_t,
     pub test_type :    u32
}

impl Copy for xcb_sync_trigger_t {}
impl Clone for xcb_sync_trigger_t {
    fn clone(&self) -> xcb_sync_trigger_t { *self }
}
/**
 * @brief xcb_sync_trigger_iterator_t
 **/
#[repr(C)]
pub struct xcb_sync_trigger_iterator_t {
    pub data : *mut xcb_sync_trigger_t,
    pub rem  : c_int,
    pub index: c_int
}


#[repr(C)]
pub struct xcb_sync_waitcondition_t {
     pub trigger :           xcb_sync_trigger_t,
     pub event_threshold :   xcb_sync_int64_t
}

impl Copy for xcb_sync_waitcondition_t {}
impl Clone for xcb_sync_waitcondition_t {
    fn clone(&self) -> xcb_sync_waitcondition_t { *self }
}
/**
 * @brief xcb_sync_waitcondition_iterator_t
 **/
#[repr(C)]
pub struct xcb_sync_waitcondition_iterator_t {
    pub data : *mut xcb_sync_waitcondition_t,
    pub rem  : c_int,
    pub index: c_int
}



#[repr(C)]
pub struct xcb_sync_counter_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16,
     pub bad_counter :     u32,
     pub minor_opcode :    u16,
     pub major_opcode :    u8
}

impl Copy for xcb_sync_counter_error_t {}
impl Clone for xcb_sync_counter_error_t {
    fn clone(&self) -> xcb_sync_counter_error_t { *self }
}


#[repr(C)]
pub struct xcb_sync_alarm_error_t {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16,
     pub bad_alarm :       u32,
     pub minor_opcode :    u16,
     pub major_opcode :    u8
}

impl Copy for xcb_sync_alarm_error_t {}
impl Clone for xcb_sync_alarm_error_t {
    fn clone(&self) -> xcb_sync_alarm_error_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_sync_initialize_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_sync_initialize_request_t {
     pub major_opcode :            u8,
     pub minor_opcode :            u8,
     pub length :                  u16,
     pub desired_major_version :   u8,
     pub desired_minor_version :   u8
}

impl Copy for xcb_sync_initialize_request_t {}
impl Clone for xcb_sync_initialize_request_t {
    fn clone(&self) -> xcb_sync_initialize_request_t { *self }
}

#[repr(C)]
pub struct xcb_sync_initialize_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u8,
     pub minor_version :   u8,
     pub pad1 :            [u8; 22]
}

impl Copy for xcb_sync_initialize_reply_t {}
impl Clone for xcb_sync_initialize_reply_t {
    fn clone(&self) -> xcb_sync_initialize_reply_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_sync_list_system_counters_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_sync_list_system_counters_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_sync_list_system_counters_request_t {}
impl Clone for xcb_sync_list_system_counters_request_t {
    fn clone(&self) -> xcb_sync_list_system_counters_request_t { *self }
}

#[repr(C)]
pub struct xcb_sync_list_system_counters_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub counters_len :    u32,
     pub pad1 :            [u8; 20]
}

impl Copy for xcb_sync_list_system_counters_reply_t {}
impl Clone for xcb_sync_list_system_counters_reply_t {
    fn clone(&self) -> xcb_sync_list_system_counters_reply_t { *self }
}


#[repr(C)]
pub struct xcb_sync_create_counter_request_t {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub id :              xcb_sync_counter_t,
     pub initial_value :   xcb_sync_int64_t
}

impl Copy for xcb_sync_create_counter_request_t {}
impl Clone for xcb_sync_create_counter_request_t {
    fn clone(&self) -> xcb_sync_create_counter_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_destroy_counter_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        xcb_sync_counter_t
}

impl Copy for xcb_sync_destroy_counter_request_t {}
impl Clone for xcb_sync_destroy_counter_request_t {
    fn clone(&self) -> xcb_sync_destroy_counter_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_sync_query_counter_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_sync_query_counter_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        xcb_sync_counter_t
}

impl Copy for xcb_sync_query_counter_request_t {}
impl Clone for xcb_sync_query_counter_request_t {
    fn clone(&self) -> xcb_sync_query_counter_request_t { *self }
}

#[repr(C)]
pub struct xcb_sync_query_counter_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub counter_value :   xcb_sync_int64_t
}

impl Copy for xcb_sync_query_counter_reply_t {}
impl Clone for xcb_sync_query_counter_reply_t {
    fn clone(&self) -> xcb_sync_query_counter_reply_t { *self }
}


#[repr(C)]
pub struct xcb_sync_await_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_sync_await_request_t {}
impl Clone for xcb_sync_await_request_t {
    fn clone(&self) -> xcb_sync_await_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_change_counter_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        xcb_sync_counter_t,
     pub amount :         xcb_sync_int64_t
}

impl Copy for xcb_sync_change_counter_request_t {}
impl Clone for xcb_sync_change_counter_request_t {
    fn clone(&self) -> xcb_sync_change_counter_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_set_counter_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        xcb_sync_counter_t,
     pub value :          xcb_sync_int64_t
}

impl Copy for xcb_sync_set_counter_request_t {}
impl Clone for xcb_sync_set_counter_request_t {
    fn clone(&self) -> xcb_sync_set_counter_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_create_alarm_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             xcb_sync_alarm_t,
     pub value_mask :     u32
}

impl Copy for xcb_sync_create_alarm_request_t {}
impl Clone for xcb_sync_create_alarm_request_t {
    fn clone(&self) -> xcb_sync_create_alarm_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_change_alarm_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             xcb_sync_alarm_t,
     pub value_mask :     u32
}

impl Copy for xcb_sync_change_alarm_request_t {}
impl Clone for xcb_sync_change_alarm_request_t {
    fn clone(&self) -> xcb_sync_change_alarm_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_destroy_alarm_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub alarm :          xcb_sync_alarm_t
}

impl Copy for xcb_sync_destroy_alarm_request_t {}
impl Clone for xcb_sync_destroy_alarm_request_t {
    fn clone(&self) -> xcb_sync_destroy_alarm_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_sync_query_alarm_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_sync_query_alarm_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub alarm :          xcb_sync_alarm_t
}

impl Copy for xcb_sync_query_alarm_request_t {}
impl Clone for xcb_sync_query_alarm_request_t {
    fn clone(&self) -> xcb_sync_query_alarm_request_t { *self }
}

#[repr(C)]
pub struct xcb_sync_query_alarm_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub trigger :         xcb_sync_trigger_t,
     pub delta :           xcb_sync_int64_t,
     pub events :          u8,
     pub state :           u8,
     pub pad1 :            [u8; 2]
}

impl Copy for xcb_sync_query_alarm_reply_t {}
impl Clone for xcb_sync_query_alarm_reply_t {
    fn clone(&self) -> xcb_sync_query_alarm_reply_t { *self }
}


#[repr(C)]
pub struct xcb_sync_set_priority_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             u32,
     pub priority :       i32
}

impl Copy for xcb_sync_set_priority_request_t {}
impl Clone for xcb_sync_set_priority_request_t {
    fn clone(&self) -> xcb_sync_set_priority_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_sync_get_priority_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_sync_get_priority_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             u32
}

impl Copy for xcb_sync_get_priority_request_t {}
impl Clone for xcb_sync_get_priority_request_t {
    fn clone(&self) -> xcb_sync_get_priority_request_t { *self }
}

#[repr(C)]
pub struct xcb_sync_get_priority_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub priority :        i32
}

impl Copy for xcb_sync_get_priority_reply_t {}
impl Clone for xcb_sync_get_priority_reply_t {
    fn clone(&self) -> xcb_sync_get_priority_reply_t { *self }
}


#[repr(C)]
pub struct xcb_sync_create_fence_request_t {
     pub major_opcode :          u8,
     pub minor_opcode :          u8,
     pub length :                u16,
     pub drawable :              ffi::xproto::xcb_drawable_t,
     pub fence :                 xcb_sync_fence_t,
     pub initially_triggered :   u8
}

impl Copy for xcb_sync_create_fence_request_t {}
impl Clone for xcb_sync_create_fence_request_t {
    fn clone(&self) -> xcb_sync_create_fence_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_trigger_fence_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          xcb_sync_fence_t
}

impl Copy for xcb_sync_trigger_fence_request_t {}
impl Clone for xcb_sync_trigger_fence_request_t {
    fn clone(&self) -> xcb_sync_trigger_fence_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_reset_fence_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          xcb_sync_fence_t
}

impl Copy for xcb_sync_reset_fence_request_t {}
impl Clone for xcb_sync_reset_fence_request_t {
    fn clone(&self) -> xcb_sync_reset_fence_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_destroy_fence_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          xcb_sync_fence_t
}

impl Copy for xcb_sync_destroy_fence_request_t {}
impl Clone for xcb_sync_destroy_fence_request_t {
    fn clone(&self) -> xcb_sync_destroy_fence_request_t { *self }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_sync_query_fence_cookie_t {
    sequence : c_uint
}


#[repr(C)]
pub struct xcb_sync_query_fence_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          xcb_sync_fence_t
}

impl Copy for xcb_sync_query_fence_request_t {}
impl Clone for xcb_sync_query_fence_request_t {
    fn clone(&self) -> xcb_sync_query_fence_request_t { *self }
}

#[repr(C)]
pub struct xcb_sync_query_fence_reply_t {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub triggered :       u8,
     pub pad1 :            [u8; 23]
}

impl Copy for xcb_sync_query_fence_reply_t {}
impl Clone for xcb_sync_query_fence_reply_t {
    fn clone(&self) -> xcb_sync_query_fence_reply_t { *self }
}


#[repr(C)]
pub struct xcb_sync_await_fence_request_t {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}

impl Copy for xcb_sync_await_fence_request_t {}
impl Clone for xcb_sync_await_fence_request_t {
    fn clone(&self) -> xcb_sync_await_fence_request_t { *self }
}


#[repr(C)]
pub struct xcb_sync_counter_notify_event_t {
     pub response_type :   u8,
     pub kind :            u8,
     pub sequence :        u16,
     pub counter :         xcb_sync_counter_t,
     pub wait_value :      xcb_sync_int64_t,
     pub counter_value :   xcb_sync_int64_t,
     pub timestamp :       ffi::xproto::xcb_timestamp_t,
     pub count :           u16,
     pub destroyed :       u8,
     pub pad0 :            u8
}

impl Copy for xcb_sync_counter_notify_event_t {}
impl Clone for xcb_sync_counter_notify_event_t {
    fn clone(&self) -> xcb_sync_counter_notify_event_t { *self }
}


#[repr(C)]
pub struct xcb_sync_alarm_notify_event_t {
     pub response_type :   u8,
     pub kind :            u8,
     pub sequence :        u16,
     pub alarm :           xcb_sync_alarm_t,
     pub counter_value :   xcb_sync_int64_t,
     pub alarm_value :     xcb_sync_int64_t,
     pub timestamp :       ffi::xproto::xcb_timestamp_t,
     pub state :           u8,
     pub pad0 :            [u8; 3]
}

impl Copy for xcb_sync_alarm_notify_event_t {}
impl Clone for xcb_sync_alarm_notify_event_t {
    fn clone(&self) -> xcb_sync_alarm_notify_event_t { *self }
}
#[link(name="xcb-sync")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_sync_alarm_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_sync_alarm_t)
 *
 *
 */
pub fn xcb_sync_alarm_next (i:*mut xcb_sync_alarm_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_sync_alarm_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_alarm_end (i:xcb_sync_alarm_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_sync_counter_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_sync_counter_t)
 *
 *
 */
pub fn xcb_sync_counter_next (i:*mut xcb_sync_counter_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_sync_counter_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_counter_end (i:xcb_sync_counter_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_sync_fence_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_sync_fence_t)
 *
 *
 */
pub fn xcb_sync_fence_next (i:*mut xcb_sync_fence_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_sync_fence_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_fence_end (i:xcb_sync_fence_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_sync_int64_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_sync_int64_t)
 *
 *
 */
pub fn xcb_sync_int64_next (i:*mut xcb_sync_int64_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_sync_int64_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_int64_end (i:xcb_sync_int64_iterator_t) -> ffi::base::xcb_generic_iterator_t;

pub fn xcb_sync_systemcounter_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_sync_systemcounter_name (R : *mut xcb_sync_systemcounter_t) -> *mut c_char;


pub fn xcb_sync_systemcounter_name_length (R : *mut xcb_sync_systemcounter_t) -> c_int;


pub fn xcb_sync_systemcounter_name_end (R : *mut xcb_sync_systemcounter_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_sync_systemcounter_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_sync_systemcounter_t)
 *
 *
 */
pub fn xcb_sync_systemcounter_next (i:*mut xcb_sync_systemcounter_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_sync_systemcounter_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_systemcounter_end (i:xcb_sync_systemcounter_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_sync_trigger_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_sync_trigger_t)
 *
 *
 */
pub fn xcb_sync_trigger_next (i:*mut xcb_sync_trigger_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_sync_trigger_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_trigger_end (i:xcb_sync_trigger_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_sync_waitcondition_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_sync_waitcondition_t)
 *
 *
 */
pub fn xcb_sync_waitcondition_next (i:*mut xcb_sync_waitcondition_iterator_t) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_sync_waitcondition_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_waitcondition_end (i:xcb_sync_waitcondition_iterator_t) -> ffi::base::xcb_generic_iterator_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_initialize (c : *mut ffi::base::xcb_connection_t,
                               desired_major_version :  u8,
                               desired_minor_version :  u8) -> xcb_sync_initialize_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_initialize_unchecked (c : *mut ffi::base::xcb_connection_t,
                                         desired_major_version :  u8,
                                         desired_minor_version :  u8) -> xcb_sync_initialize_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_initialize_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_initialize_reply (c : *mut ffi::base::xcb_connection_t,
                                     cookie : xcb_sync_initialize_cookie_t,
                                     e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_sync_initialize_reply_t;

pub fn xcb_sync_list_system_counters_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_list_system_counters (c : *mut ffi::base::xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_list_system_counters_unchecked (c : *mut ffi::base::xcb_connection_t) -> xcb_sync_list_system_counters_cookie_t;


pub fn xcb_sync_list_system_counters_counters_length (R : *mut xcb_sync_list_system_counters_reply_t) -> c_int;

pub fn xcb_sync_list_system_counters_counters_iterator (R : *mut xcb_sync_list_system_counters_reply_t) -> xcb_sync_systemcounter_iterator_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_list_system_counters_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_list_system_counters_reply (c : *mut ffi::base::xcb_connection_t,
                                               cookie : xcb_sync_list_system_counters_cookie_t,
                                               e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_sync_list_system_counters_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_create_counter_checked (c : *mut ffi::base::xcb_connection_t,
                                           id :  xcb_sync_counter_t,
                                           initial_value :  xcb_sync_int64_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_create_counter (c : *mut ffi::base::xcb_connection_t,
                                   id :  xcb_sync_counter_t,
                                   initial_value :  xcb_sync_int64_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_destroy_counter_checked (c : *mut ffi::base::xcb_connection_t,
                                            counter :  xcb_sync_counter_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_destroy_counter (c : *mut ffi::base::xcb_connection_t,
                                    counter :  xcb_sync_counter_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_query_counter (c : *mut ffi::base::xcb_connection_t,
                                  counter :  xcb_sync_counter_t) -> xcb_sync_query_counter_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_query_counter_unchecked (c : *mut ffi::base::xcb_connection_t,
                                            counter :  xcb_sync_counter_t) -> xcb_sync_query_counter_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_counter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_counter_reply (c : *mut ffi::base::xcb_connection_t,
                                        cookie : xcb_sync_query_counter_cookie_t,
                                        e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_sync_query_counter_reply_t;

pub fn xcb_sync_await_sizeof (_buffer :  *mut c_void,
                       wait_list_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_await_checked (c : *mut ffi::base::xcb_connection_t,
                                  wait_list_len :  u32,
                                  wait_list : *mut xcb_sync_waitcondition_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_await (c : *mut ffi::base::xcb_connection_t,
                          wait_list_len :  u32,
                          wait_list : *mut xcb_sync_waitcondition_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_change_counter_checked (c : *mut ffi::base::xcb_connection_t,
                                           counter :  xcb_sync_counter_t,
                                           amount :  xcb_sync_int64_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_change_counter (c : *mut ffi::base::xcb_connection_t,
                                   counter :  xcb_sync_counter_t,
                                   amount :  xcb_sync_int64_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_set_counter_checked (c : *mut ffi::base::xcb_connection_t,
                                        counter :  xcb_sync_counter_t,
                                        value :  xcb_sync_int64_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_set_counter (c : *mut ffi::base::xcb_connection_t,
                                counter :  xcb_sync_counter_t,
                                value :  xcb_sync_int64_t) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_sync_create_alarm_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_create_alarm_checked (c : *mut ffi::base::xcb_connection_t,
                                         id :  xcb_sync_alarm_t,
                                         value_mask :  u32,
                                         value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_create_alarm (c : *mut ffi::base::xcb_connection_t,
                                 id :  xcb_sync_alarm_t,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

pub fn xcb_sync_change_alarm_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_change_alarm_checked (c : *mut ffi::base::xcb_connection_t,
                                         id :  xcb_sync_alarm_t,
                                         value_mask :  u32,
                                         value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_change_alarm (c : *mut ffi::base::xcb_connection_t,
                                 id :  xcb_sync_alarm_t,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_destroy_alarm_checked (c : *mut ffi::base::xcb_connection_t,
                                          alarm :  xcb_sync_alarm_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_destroy_alarm (c : *mut ffi::base::xcb_connection_t,
                                  alarm :  xcb_sync_alarm_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_query_alarm (c : *mut ffi::base::xcb_connection_t,
                                alarm :  xcb_sync_alarm_t) -> xcb_sync_query_alarm_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_query_alarm_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          alarm :  xcb_sync_alarm_t) -> xcb_sync_query_alarm_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_alarm_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_alarm_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_sync_query_alarm_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_sync_query_alarm_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_set_priority_checked (c : *mut ffi::base::xcb_connection_t,
                                         id :  u32,
                                         priority :  i32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_set_priority (c : *mut ffi::base::xcb_connection_t,
                                 id :  u32,
                                 priority :  i32) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_get_priority (c : *mut ffi::base::xcb_connection_t,
                                 id :  u32) -> xcb_sync_get_priority_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_get_priority_unchecked (c : *mut ffi::base::xcb_connection_t,
                                           id :  u32) -> xcb_sync_get_priority_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_get_priority_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_get_priority_reply (c : *mut ffi::base::xcb_connection_t,
                                       cookie : xcb_sync_get_priority_cookie_t,
                                       e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_sync_get_priority_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_create_fence_checked (c : *mut ffi::base::xcb_connection_t,
                                         drawable :  ffi::xproto::xcb_drawable_t,
                                         fence :  xcb_sync_fence_t,
                                         initially_triggered :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_create_fence (c : *mut ffi::base::xcb_connection_t,
                                 drawable :  ffi::xproto::xcb_drawable_t,
                                 fence :  xcb_sync_fence_t,
                                 initially_triggered :  u8) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_trigger_fence_checked (c : *mut ffi::base::xcb_connection_t,
                                          fence :  xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_trigger_fence (c : *mut ffi::base::xcb_connection_t,
                                  fence :  xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_reset_fence_checked (c : *mut ffi::base::xcb_connection_t,
                                        fence :  xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_reset_fence (c : *mut ffi::base::xcb_connection_t,
                                fence :  xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_destroy_fence_checked (c : *mut ffi::base::xcb_connection_t,
                                          fence :  xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_destroy_fence (c : *mut ffi::base::xcb_connection_t,
                                  fence :  xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_query_fence (c : *mut ffi::base::xcb_connection_t,
                                fence :  xcb_sync_fence_t) -> xcb_sync_query_fence_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_query_fence_unchecked (c : *mut ffi::base::xcb_connection_t,
                                          fence :  xcb_sync_fence_t) -> xcb_sync_query_fence_cookie_t;

/**
 * Return the reply
 * @param c      The xcb_connection_t
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_fence_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_fence_reply (c : *mut ffi::base::xcb_connection_t,
                                      cookie : xcb_sync_query_fence_cookie_t,
                                      e : *mut *mut ffi::base::xcb_generic_error_t) -> *mut xcb_sync_query_fence_reply_t;

pub fn xcb_sync_await_fence_sizeof (_buffer :  *mut c_void,
                             fence_list_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_await_fence_checked (c : *mut ffi::base::xcb_connection_t,
                                        fence_list_len :  u32,
                                        fence_list : *mut xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
pub fn xcb_sync_await_fence (c : *mut ffi::base::xcb_connection_t,
                                fence_list_len :  u32,
                                fence_list : *mut xcb_sync_fence_t) -> ffi::base::xcb_void_cookie_t;
}

