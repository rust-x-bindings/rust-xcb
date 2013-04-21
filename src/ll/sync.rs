/*
 * This file generated automatically from sync.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core::libc::*;
use ll::base::*;
use ll::xproto;

pub static SYNC_MAJOR_VERSION : c_uint = 3;
pub static SYNC_MINOR_VERSION : c_uint = 1;

pub type alarm = u32;

/**
 * @brief alarm_iterator
 **/
pub struct alarm_iterator {
    data : *alarm,
    rem  : c_int,
    index: c_int
}

pub type alarmstate = c_uint;//{
    pub static XCB_SYNC_ALARMSTATE_ACTIVE : alarmstate = 1;
    pub static XCB_SYNC_ALARMSTATE_INACTIVE : alarmstate = 2;
    pub static XCB_SYNC_ALARMSTATE_DESTROYED : alarmstate = 3;
//}

pub type counter = u32;

/**
 * @brief counter_iterator
 **/
pub struct counter_iterator {
    data : *counter,
    rem  : c_int,
    index: c_int
}

pub type fence = u32;

/**
 * @brief fence_iterator
 **/
pub struct fence_iterator {
    data : *fence,
    rem  : c_int,
    index: c_int
}

pub type testtype = c_uint;//{
    pub static XCB_SYNC_TESTTYPE_POSITIVE_TRANSITION : testtype = 1;
    pub static XCB_SYNC_TESTTYPE_NEGATIVE_TRANSITION : testtype = 2;
    pub static XCB_SYNC_TESTTYPE_POSITIVE_COMPARISON : testtype = 3;
    pub static XCB_SYNC_TESTTYPE_NEGATIVE_COMPARISON : testtype = 4;
//}

pub type valuetype = c_uint;//{
    pub static XCB_SYNC_VALUETYPE_ABSOLUTE : valuetype = 1;
    pub static XCB_SYNC_VALUETYPE_RELATIVE : valuetype = 2;
//}

pub type ca = c_uint;//{
    pub static XCB_SYNC_CA_COUNTER : ca = 1;
    pub static XCB_SYNC_CA_VALUE_TYPE : ca = 2;
    pub static XCB_SYNC_CA_VALUE : ca = 4;
    pub static XCB_SYNC_CA_TEST_TYPE : ca = 8;
    pub static XCB_SYNC_CA_DELTA : ca = 16;
    pub static XCB_SYNC_CA_EVENTS : ca = 32;
//}

pub struct int64 {
    hi :   i32,
    lo :   u32
}

/**
 * @brief int64_iterator
 **/
pub struct int64_iterator {
    data : *int64,
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
    data : *systemcounter,
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
    data : *trigger,
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
    data : *waitcondition,
    rem  : c_int,
    index: c_int
}

/** Opcode for xcb_sync_counter. */
pub static XCB_SYNC_COUNTER : c_int = 0;

pub struct counter_error {
    response_type :   u8,
    error_code :      u8,
    sequence :        u16,
    bad_counter :     u32,
    minor_opcode :    u16,
    major_opcode :    u8
}

/** Opcode for xcb_sync_alarm. */
pub static XCB_SYNC_ALARM : c_int = 1;

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

/** Opcode for xcb_sync_initialize. */
pub static XCB_SYNC_INITIALIZE : c_int = 0;

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

/** Opcode for xcb_sync_list_system_counters. */
pub static XCB_SYNC_LIST_SYSTEM_COUNTERS : c_int = 1;

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

/** Opcode for xcb_sync_create_counter. */
pub static XCB_SYNC_CREATE_COUNTER : c_int = 2;

pub struct create_counter_request {
    major_opcode :    u8,
    minor_opcode :    u8,
    length :          u16,
    id :              counter,
    initial_value :   int64
}

/** Opcode for xcb_sync_destroy_counter. */
pub static XCB_SYNC_DESTROY_COUNTER : c_int = 6;

pub struct destroy_counter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    counter :        counter
}

pub struct query_counter_cookie {
    sequence : c_uint
}

/** Opcode for xcb_sync_query_counter. */
pub static XCB_SYNC_QUERY_COUNTER : c_int = 5;

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

/** Opcode for xcb_sync_await. */
pub static XCB_SYNC_AWAIT : c_int = 7;

pub struct await_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

/** Opcode for xcb_sync_change_counter. */
pub static XCB_SYNC_CHANGE_COUNTER : c_int = 4;

pub struct change_counter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    counter :        counter,
    amount :         int64
}

/** Opcode for xcb_sync_set_counter. */
pub static XCB_SYNC_SET_COUNTER : c_int = 3;

pub struct set_counter_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    counter :        counter,
    value :          int64
}

/** Opcode for xcb_sync_create_alarm. */
pub static XCB_SYNC_CREATE_ALARM : c_int = 8;

pub struct create_alarm_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    id :             alarm,
    value_mask :     u32
}

/** Opcode for xcb_sync_change_alarm. */
pub static XCB_SYNC_CHANGE_ALARM : c_int = 9;

pub struct change_alarm_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    id :             alarm,
    value_mask :     u32
}

/** Opcode for xcb_sync_destroy_alarm. */
pub static XCB_SYNC_DESTROY_ALARM : c_int = 11;

pub struct destroy_alarm_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    alarm :          alarm
}

pub struct query_alarm_cookie {
    sequence : c_uint
}

/** Opcode for xcb_sync_query_alarm. */
pub static XCB_SYNC_QUERY_ALARM : c_int = 10;

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

/** Opcode for xcb_sync_set_priority. */
pub static XCB_SYNC_SET_PRIORITY : c_int = 12;

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

/** Opcode for xcb_sync_get_priority. */
pub static XCB_SYNC_GET_PRIORITY : c_int = 13;

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

/** Opcode for xcb_sync_create_fence. */
pub static XCB_SYNC_CREATE_FENCE : c_int = 14;

pub struct create_fence_request {
    major_opcode :          u8,
    minor_opcode :          u8,
    length :                u16,
    drawable :              xproto::drawable,
    fence :                 fence,
    initially_triggered :   u8
}

/** Opcode for xcb_sync_trigger_fence. */
pub static XCB_SYNC_TRIGGER_FENCE : c_int = 15;

pub struct trigger_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    fence :          fence
}

/** Opcode for xcb_sync_reset_fence. */
pub static XCB_SYNC_RESET_FENCE : c_int = 16;

pub struct reset_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    fence :          fence
}

/** Opcode for xcb_sync_destroy_fence. */
pub static XCB_SYNC_DESTROY_FENCE : c_int = 17;

pub struct destroy_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    fence :          fence
}

pub struct query_fence_cookie {
    sequence : c_uint
}

/** Opcode for xcb_sync_query_fence. */
pub static XCB_SYNC_QUERY_FENCE : c_int = 18;

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

/** Opcode for xcb_sync_await_fence. */
pub static XCB_SYNC_AWAIT_FENCE : c_int = 19;

pub struct await_fence_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16
}

/** Opcode for xcb_sync_counter_notify. */
pub static XCB_SYNC_COUNTER_NOTIFY : c_int = 0;

pub struct counter_notify_event {
    response_type :   u8,
    kind :            u8,
    sequence :        u16,
    counter :         counter,
    wait_value :      int64,
    counter_value :   int64,
    timestamp :       xproto::timestamp,
    count :           u16,
    destroyed :       u8,
    pad0 :            u8
}

/** Opcode for xcb_sync_alarm_notify. */
pub static XCB_SYNC_ALARM_NOTIFY : c_int = 1;

pub struct alarm_notify_event {
    response_type :   u8,
    kind :            u8,
    sequence :        u16,
    alarm :           alarm,
    counter_value :   int64,
    alarm_value :     int64,
    timestamp :       xproto::timestamp,
    state :           u8,
    pad0 :            [u8,..3]
}
#[link_args="-lxcb-sync"]
extern "C" {

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
unsafe fn xcb_sync_alarm_next (i:*alarm_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An alarm_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_sync_alarm_end (i:alarm_iterator) -> generic_iterator;

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
unsafe fn xcb_sync_counter_next (i:*counter_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An counter_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_sync_counter_end (i:counter_iterator) -> generic_iterator;

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
unsafe fn xcb_sync_fence_next (i:*fence_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An fence_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_sync_fence_end (i:fence_iterator) -> generic_iterator;

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
unsafe fn xcb_sync_int64_next (i:*int64_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An int64_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_sync_int64_end (i:int64_iterator) -> generic_iterator;

unsafe fn xcb_sync_systemcounter_sizeof (_buffer :  *c_void) -> c_int;

unsafe fn xcb_sync_systemcounter_name (R : *systemcounter) -> *u8;


unsafe fn xcb_sync_systemcounter_name_length (R : *systemcounter) -> c_int;


unsafe fn xcb_sync_systemcounter_name_end (R : *systemcounter) -> generic_iterator;

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
unsafe fn xcb_sync_systemcounter_next (i:*systemcounter_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An systemcounter_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_sync_systemcounter_end (i:systemcounter_iterator) -> generic_iterator;

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
unsafe fn xcb_sync_trigger_next (i:*trigger_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An trigger_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_sync_trigger_end (i:trigger_iterator) -> generic_iterator;

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
unsafe fn xcb_sync_waitcondition_next (i:*waitcondition_iterator) -> ();

/**
 * Return the iterator pointing to the last element
 * @param i An waitcondition_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
unsafe fn xcb_sync_waitcondition_end (i:waitcondition_iterator) -> generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_initialize (c : *connection,
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
unsafe fn xcb_sync_initialize_unchecked (c : *connection,
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
unsafe fn xcb_sync_initialize_reply (c : *connection,
                                     cookie : initialize_cookie,
                                     e : **generic_error) -> *initialize_reply;

unsafe fn xcb_sync_list_system_counters_sizeof (_buffer :  *c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_list_system_counters (c : *connection) -> list_system_counters_cookie;

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
unsafe fn xcb_sync_list_system_counters_unchecked (c : *connection) -> list_system_counters_cookie;


unsafe fn xcb_sync_list_system_counters_counters_length (R : *list_system_counters_reply) -> c_int;

unsafe fn xcb_sync_list_system_counters_counters_iterator (R : *list_system_counters_reply) -> systemcounter_iterator;

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
unsafe fn xcb_sync_list_system_counters_reply (c : *connection,
                                               cookie : list_system_counters_cookie,
                                               e : **generic_error) -> *list_system_counters_reply;

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
unsafe fn xcb_sync_create_counter_checked (c : *connection,
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
unsafe fn xcb_sync_create_counter (c : *connection,
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
unsafe fn xcb_sync_destroy_counter_checked (c : *connection,
                                            counter :  counter) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_destroy_counter (c : *connection,
                                    counter :  counter) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_query_counter (c : *connection,
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
unsafe fn xcb_sync_query_counter_unchecked (c : *connection,
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
unsafe fn xcb_sync_query_counter_reply (c : *connection,
                                        cookie : query_counter_cookie,
                                        e : **generic_error) -> *query_counter_reply;

unsafe fn xcb_sync_await_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_sync_await_checked (c : *connection,
                                  wait_list_len :  u32,
                                  wait_list : *waitcondition) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_await (c : *connection,
                          wait_list_len :  u32,
                          wait_list : *waitcondition) -> void_cookie;

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
unsafe fn xcb_sync_change_counter_checked (c : *connection,
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
unsafe fn xcb_sync_change_counter (c : *connection,
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
unsafe fn xcb_sync_set_counter_checked (c : *connection,
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
unsafe fn xcb_sync_set_counter (c : *connection,
                                counter :  counter,
                                value :  int64) -> void_cookie;

unsafe fn xcb_sync_create_alarm_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_sync_create_alarm_checked (c : *connection,
                                         id :  alarm,
                                         value_mask :  u32,
                                         value_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_create_alarm (c : *connection,
                                 id :  alarm,
                                 value_mask :  u32,
                                 value_list : *u32) -> void_cookie;

unsafe fn xcb_sync_change_alarm_sizeof (_buffer :  *c_void) -> c_int;

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
unsafe fn xcb_sync_change_alarm_checked (c : *connection,
                                         id :  alarm,
                                         value_mask :  u32,
                                         value_list : *u32) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_change_alarm (c : *connection,
                                 id :  alarm,
                                 value_mask :  u32,
                                 value_list : *u32) -> void_cookie;

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
unsafe fn xcb_sync_destroy_alarm_checked (c : *connection,
                                          alarm :  alarm) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_destroy_alarm (c : *connection,
                                  alarm :  alarm) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_query_alarm (c : *connection,
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
unsafe fn xcb_sync_query_alarm_unchecked (c : *connection,
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
unsafe fn xcb_sync_query_alarm_reply (c : *connection,
                                      cookie : query_alarm_cookie,
                                      e : **generic_error) -> *query_alarm_reply;

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
unsafe fn xcb_sync_set_priority_checked (c : *connection,
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
unsafe fn xcb_sync_set_priority (c : *connection,
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
unsafe fn xcb_sync_get_priority (c : *connection,
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
unsafe fn xcb_sync_get_priority_unchecked (c : *connection,
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
unsafe fn xcb_sync_get_priority_reply (c : *connection,
                                       cookie : get_priority_cookie,
                                       e : **generic_error) -> *get_priority_reply;

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
unsafe fn xcb_sync_create_fence_checked (c : *connection,
                                         drawable :  xproto::drawable,
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
unsafe fn xcb_sync_create_fence (c : *connection,
                                 drawable :  xproto::drawable,
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
unsafe fn xcb_sync_trigger_fence_checked (c : *connection,
                                          fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_trigger_fence (c : *connection,
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
unsafe fn xcb_sync_reset_fence_checked (c : *connection,
                                        fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_reset_fence (c : *connection,
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
unsafe fn xcb_sync_destroy_fence_checked (c : *connection,
                                          fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_destroy_fence (c : *connection,
                                  fence :  fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_query_fence (c : *connection,
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
unsafe fn xcb_sync_query_fence_unchecked (c : *connection,
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
unsafe fn xcb_sync_query_fence_reply (c : *connection,
                                      cookie : query_fence_cookie,
                                      e : **generic_error) -> *query_fence_reply;

unsafe fn xcb_sync_await_fence_sizeof (_buffer :  *c_void,
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
unsafe fn xcb_sync_await_fence_checked (c : *connection,
                                        fence_list_len :  u32,
                                        fence_list : *fence) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_sync_await_fence (c : *connection,
                                fence_list_len :  u32,
                                fence_list : *fence) -> void_cookie;
}

