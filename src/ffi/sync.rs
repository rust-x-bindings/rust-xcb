/*
 * This file generated automatically from sync.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use ffi;
use ffi::xproto;

pub static SYNC_MAJOR_VERSION : c_uint = 3;
pub static SYNC_MINOR_VERSION : c_uint = 1;

pub type alarm = u32;
/**
 * @brief alarm_iterator
 **/
pub struct alarm_iterator {
    data : *mut alarm,
    rem  : c_int,
    index: c_int
}


pub type counter = u32;
/**
 * @brief counter_iterator
 **/
pub struct counter_iterator {
    data : *mut counter,
    rem  : c_int,
    index: c_int
}


pub type fence = u32;
/**
 * @brief fence_iterator
 **/
pub struct fence_iterator {
    data : *mut fence,
    rem  : c_int,
    index: c_int
}


pub struct int64 {
    hi :   i32,
    lo :   u32
}

/**
 * @brief int64_iterator
 **/
pub struct int64_iterator {
    data : *mut int64,
    rem  : c_int,
    index: c_int
}


pub struct systemcounter {
    counter :      counter,
    resolution :   int64,
    name_len :     u16
}

/**
 * @brief systemcounter_iterator
 **/
pub struct systemcounter_iterator {
    data : *mut systemcounter,
    rem  : c_int,
    index: c_int
}


pub struct trigger {
    counter :      counter,
    wait_type :    u32,
    wait_value :   int64,
    test_type :    u32
}

/**
 * @brief trigger_iterator
 **/
pub struct trigger_iterator {
    data : *mut trigger,
    rem  : c_int,
    index: c_int
}


pub struct waitcondition {
    trigger :           trigger,
    event_threshold :   int64
}

/**
 * @brief waitcondition_iterator
 **/
pub struct waitcondition_iterator {
    data : *mut waitcondition,
    rem  : c_int,
    index: c_int
}



pub struct counter_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    bad_counter :     u32,
    minor_opcode :    u16,
    major_opcode :    u8
}



pub struct alarm_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    bad_alarm :       u32,
    minor_opcode :    u16,
    major_opcode :    u8
}


pub struct initialize_cookie {
    sequence : c_uint
}


pub struct initialize_request {
    major_opcode :            u8,
    minor_opcode :            u8,
    length :                  u16,
    desired_major_version :   u8,
    desired_minor_version :   u8
}


pub struct initialize_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u8,
    minor_version :   u8,
    pad1 :            [u8,..22]
}


pub struct list_system_counters_cookie {
    sequence : c_uint
}


pub struct list_system_counters_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}


pub struct list_system_counters_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    counters_len :    u32,
    pad1 :            [u8,..20]
}



pub struct create_counter_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    id :              counter,
    initial_value :   int64
}



pub struct destroy_counter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    counter :        counter
}


pub struct query_counter_cookie {
    sequence : c_uint
}


pub struct query_counter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    counter :        counter
}


pub struct query_counter_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    counter_value :   int64
}



pub struct await_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}



pub struct change_counter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    counter :        counter,
    amount :         int64
}



pub struct set_counter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    counter :        counter,
    value :          int64
}



pub struct create_alarm_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    id :             alarm,
    value_mask :     u32
}



pub struct change_alarm_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    id :             alarm,
    value_mask :     u32
}



pub struct destroy_alarm_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    alarm :          alarm
}


pub struct query_alarm_cookie {
    sequence : c_uint
}


pub struct query_alarm_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    alarm :          alarm
}


pub struct query_alarm_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    trigger :         trigger,
    delta :           int64,
    events :          u8,
    state :           u8,
    pad1 :            [u8,..2]
}



pub struct set_priority_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    id :             u32,
    priority :       i32
}


pub struct get_priority_cookie {
    sequence : c_uint
}


pub struct get_priority_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    id :             u32
}


pub struct get_priority_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    priority :        i32
}



pub struct create_fence_request {
    major_opcode :          u8,
    minor_opcode :          u8,
    length :                u16,
    drawable :              ffi::xproto::drawable,
    fence :                 fence,
    initially_triggered :   u8
}



pub struct trigger_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    fence :          fence
}



pub struct reset_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    fence :          fence
}



pub struct destroy_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    fence :          fence
}


pub struct query_fence_cookie {
    sequence : c_uint
}


pub struct query_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    fence :          fence
}


pub struct query_fence_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    triggered :       u8,
    pad1 :            [u8,..23]
}



pub struct await_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}



pub struct counter_notify_event {
    response_type :   u8,
    kind :            u8,
    sequence :        u16,
    counter :         counter,
    wait_value :      int64,
    counter_value :   int64,
    timestamp :       ffi::xproto::timestamp,
    count :           u16,
    destroyed :       u8,
    pad0 :            u8
}



pub struct alarm_notify_event {
    response_type :   u8,
    kind :            u8,
    sequence :        u16,
    alarm :           alarm,
    counter_value :   int64,
    alarm_value :     int64,
    timestamp :       ffi::xproto::timestamp,
    state :           u8,
    pad0 :            [u8,..3]
}

#[link(name="lxcb-sync")]
pub extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a alarm_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(alarm)
 *
 *
 */
pub fn xcb_sync_alarm_next (i:*mut alarm_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An alarm_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_alarm_end (i:alarm_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a counter_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(counter)
 *
 *
 */
pub fn xcb_sync_counter_next (i:*mut counter_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An counter_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_counter_end (i:counter_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a fence_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(fence)
 *
 *
 */
pub fn xcb_sync_fence_next (i:*mut fence_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An fence_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_fence_end (i:fence_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a int64_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(int64)
 *
 *
 */
pub fn xcb_sync_int64_next (i:*mut int64_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An int64_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_int64_end (i:int64_iterator) -> generic_iterator;

pub fn xcb_sync_systemcounter_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_sync_systemcounter_name (R : *mut systemcounter) -> *mut c_char;


pub fn xcb_sync_systemcounter_name_length (R : *mut systemcounter) -> c_int;


pub fn xcb_sync_systemcounter_name_end (R : *mut systemcounter) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a systemcounter_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(systemcounter)
 *
 *
 */
pub fn xcb_sync_systemcounter_next (i:*mut systemcounter_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An systemcounter_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_systemcounter_end (i:systemcounter_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a trigger_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(trigger)
 *
 *
 */
pub fn xcb_sync_trigger_next (i:*mut trigger_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An trigger_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_trigger_end (i:trigger_iterator) -> generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a waitcondition_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(waitcondition)
 *
 *
 */
pub fn xcb_sync_waitcondition_next (i:*mut waitcondition_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An waitcondition_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_waitcondition_end (i:waitcondition_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_initialize (c : *mut connection,
                               desired_major_version :  u8,
                               desired_minor_version :  u8) -> initialize_cookie;

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
pub fn xcb_sync_initialize_unchecked (c : *mut connection,
                                         desired_major_version :  u8,
                                         desired_minor_version :  u8) -> initialize_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_initialize_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_initialize_reply (c : *mut connection,
                                     cookie : initialize_cookie,
                                     e : *mut *mut generic_error) -> *mut initialize_reply;

pub fn xcb_sync_list_system_counters_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_list_system_counters (c : *mut connection) -> list_system_counters_cookie;

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
pub fn xcb_sync_list_system_counters_unchecked (c : *mut connection) -> list_system_counters_cookie;


pub fn xcb_sync_list_system_counters_counters_length (R : *mut list_system_counters_reply) -> c_int;

pub fn xcb_sync_list_system_counters_counters_iterator (R : *mut list_system_counters_reply) -> systemcounter_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_list_system_counters_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_list_system_counters_reply (c : *mut connection,
                                               cookie : list_system_counters_cookie,
                                               e : *mut *mut generic_error) -> *mut list_system_counters_reply;

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
pub fn xcb_sync_create_counter_checked (c : *mut connection,
                                           id :  counter,
                                           initial_value :  int64) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_create_counter (c : *mut connection,
                                   id :  counter,
                                   initial_value :  int64) -> void_cookie;

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
pub fn xcb_sync_destroy_counter_checked (c : *mut connection,
                                            counter :  counter) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_destroy_counter (c : *mut connection,
                                    counter :  counter) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_query_counter (c : *mut connection,
                                  counter :  counter) -> query_counter_cookie;

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
pub fn xcb_sync_query_counter_unchecked (c : *mut connection,
                                            counter :  counter) -> query_counter_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_counter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_counter_reply (c : *mut connection,
                                        cookie : query_counter_cookie,
                                        e : *mut *mut generic_error) -> *mut query_counter_reply;

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
pub fn xcb_sync_await_checked (c : *mut connection,
                                  wait_list_len :  u32,
                                  wait_list : *mut waitcondition) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_await (c : *mut connection,
                          wait_list_len :  u32,
                          wait_list : *mut waitcondition) -> void_cookie;

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
pub fn xcb_sync_change_counter_checked (c : *mut connection,
                                           counter :  counter,
                                           amount :  int64) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_change_counter (c : *mut connection,
                                   counter :  counter,
                                   amount :  int64) -> void_cookie;

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
pub fn xcb_sync_set_counter_checked (c : *mut connection,
                                        counter :  counter,
                                        value :  int64) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_set_counter (c : *mut connection,
                                counter :  counter,
                                value :  int64) -> void_cookie;

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
pub fn xcb_sync_create_alarm_checked (c : *mut connection,
                                         id :  alarm,
                                         value_mask :  u32,
                                         value_list : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_create_alarm (c : *mut connection,
                                 id :  alarm,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> void_cookie;

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
pub fn xcb_sync_change_alarm_checked (c : *mut connection,
                                         id :  alarm,
                                         value_mask :  u32,
                                         value_list : *mut u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_change_alarm (c : *mut connection,
                                 id :  alarm,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> void_cookie;

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
pub fn xcb_sync_destroy_alarm_checked (c : *mut connection,
                                          alarm :  alarm) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_destroy_alarm (c : *mut connection,
                                  alarm :  alarm) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_query_alarm (c : *mut connection,
                                alarm :  alarm) -> query_alarm_cookie;

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
pub fn xcb_sync_query_alarm_unchecked (c : *mut connection,
                                          alarm :  alarm) -> query_alarm_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_alarm_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_alarm_reply (c : *mut connection,
                                      cookie : query_alarm_cookie,
                                      e : *mut *mut generic_error) -> *mut query_alarm_reply;

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
pub fn xcb_sync_set_priority_checked (c : *mut connection,
                                         id :  u32,
                                         priority :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_set_priority (c : *mut connection,
                                 id :  u32,
                                 priority :  i32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_get_priority (c : *mut connection,
                                 id :  u32) -> get_priority_cookie;

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
pub fn xcb_sync_get_priority_unchecked (c : *mut connection,
                                           id :  u32) -> get_priority_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_get_priority_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_get_priority_reply (c : *mut connection,
                                       cookie : get_priority_cookie,
                                       e : *mut *mut generic_error) -> *mut get_priority_reply;

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
pub fn xcb_sync_create_fence_checked (c : *mut connection,
                                         drawable :  ffi::xproto::drawable,
                                         fence :  fence,
                                         initially_triggered :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_create_fence (c : *mut connection,
                                 drawable :  ffi::xproto::drawable,
                                 fence :  fence,
                                 initially_triggered :  u8) -> void_cookie;

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
pub fn xcb_sync_trigger_fence_checked (c : *mut connection,
                                          fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_trigger_fence (c : *mut connection,
                                  fence :  fence) -> void_cookie;

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
pub fn xcb_sync_reset_fence_checked (c : *mut connection,
                                        fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_reset_fence (c : *mut connection,
                                fence :  fence) -> void_cookie;

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
pub fn xcb_sync_destroy_fence_checked (c : *mut connection,
                                          fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_destroy_fence (c : *mut connection,
                                  fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_query_fence (c : *mut connection,
                                fence :  fence) -> query_fence_cookie;

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
pub fn xcb_sync_query_fence_unchecked (c : *mut connection,
                                          fence :  fence) -> query_fence_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_fence_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_fence_reply (c : *mut connection,
                                      cookie : query_fence_cookie,
                                      e : *mut *mut generic_error) -> *mut query_fence_reply;

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
pub fn xcb_sync_await_fence_checked (c : *mut connection,
                                        fence_list_len :  u32,
                                        fence_list : *mut fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_await_fence (c : *mut connection,
                                fence_list_len :  u32,
                                fence_list : *mut fence) -> void_cookie;
}

