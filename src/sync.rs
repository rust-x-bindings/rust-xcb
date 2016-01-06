/*
 * This file generated automatically from sync.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::sync::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub type Alarm = alarm;

pub type AlarmIterator = alarm_iterator;


pub type alarmstate = c_uint;//{
    pub static XCB_SYNC_ALARMSTATE_ACTIVE : alarmstate = 1;
    pub static XCB_SYNC_ALARMSTATE_INACTIVE : alarmstate = 2;
    pub static XCB_SYNC_ALARMSTATE_DESTROYED : alarmstate = 3;
//}
pub type Counter = counter;

pub type CounterIterator = counter_iterator;

pub type FenceIterator = fence_iterator;


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
pub struct Int64 {pub base : base::Struct<int64> }

pub type Int64Iterator = int64_iterator;

pub type SystemcounterIterator = systemcounter_iterator;

pub type TriggerIterator = trigger_iterator;

pub type WaitconditionIterator = waitcondition_iterator;

/** Opcode for xcb_sync_counter. */
pub static XCB_SYNC_COUNTER : u8 = 0;
pub struct CounterError { pub base : base::Error<counter_error> }
/** Opcode for xcb_sync_alarm. */
pub static XCB_SYNC_ALARM : u8 = 1;
pub struct AlarmError { pub base : base::Error<alarm_error> }
pub struct  InitializeCookie<'s> { pub base : base::Cookie<'s, initialize_cookie> }

/** Opcode for xcb_sync_initialize. */
pub static XCB_SYNC_INITIALIZE : u8 = 0;
pub struct InitializeReply { base:  base::Reply<initialize_reply> }
fn mk_reply_initialize_reply(reply:*mut initialize_reply) -> InitializeReply { InitializeReply { base : base::mk_reply(reply) } }
pub struct  ListSystemCountersCookie<'s> { pub base : base::Cookie<'s, list_system_counters_cookie> }

/** Opcode for xcb_sync_list_system_counters. */
pub static XCB_SYNC_LIST_SYSTEM_COUNTERS : u8 = 1;
/** Opcode for xcb_sync_create_counter. */
pub static XCB_SYNC_CREATE_COUNTER : u8 = 2;
/** Opcode for xcb_sync_destroy_counter. */
pub static XCB_SYNC_DESTROY_COUNTER : u8 = 6;
pub struct  QueryCounterCookie<'s> { pub base : base::Cookie<'s, query_counter_cookie> }

/** Opcode for xcb_sync_query_counter. */
pub static XCB_SYNC_QUERY_COUNTER : u8 = 5;
pub struct QueryCounterReply { base:  base::Reply<query_counter_reply> }
fn mk_reply_query_counter_reply(reply:*mut query_counter_reply) -> QueryCounterReply { QueryCounterReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_sync_await. */
pub static XCB_SYNC_AWAIT : u8 = 7;
/** Opcode for xcb_sync_change_counter. */
pub static XCB_SYNC_CHANGE_COUNTER : u8 = 4;
/** Opcode for xcb_sync_set_counter. */
pub static XCB_SYNC_SET_COUNTER : u8 = 3;
/** Opcode for xcb_sync_create_alarm. */
pub static XCB_SYNC_CREATE_ALARM : u8 = 8;
/** Opcode for xcb_sync_change_alarm. */
pub static XCB_SYNC_CHANGE_ALARM : u8 = 9;
/** Opcode for xcb_sync_destroy_alarm. */
pub static XCB_SYNC_DESTROY_ALARM : u8 = 11;
pub struct  QueryAlarmCookie<'s> { pub base : base::Cookie<'s, query_alarm_cookie> }

/** Opcode for xcb_sync_query_alarm. */
pub static XCB_SYNC_QUERY_ALARM : u8 = 10;
pub struct QueryAlarmReply { base:  base::Reply<query_alarm_reply> }
fn mk_reply_query_alarm_reply(reply:*mut query_alarm_reply) -> QueryAlarmReply { QueryAlarmReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_sync_set_priority. */
pub static XCB_SYNC_SET_PRIORITY : u8 = 12;
pub struct  GetPriorityCookie<'s> { pub base : base::Cookie<'s, get_priority_cookie> }

/** Opcode for xcb_sync_get_priority. */
pub static XCB_SYNC_GET_PRIORITY : u8 = 13;
pub struct GetPriorityReply { base:  base::Reply<get_priority_reply> }
fn mk_reply_get_priority_reply(reply:*mut get_priority_reply) -> GetPriorityReply { GetPriorityReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_sync_create_fence. */
pub static XCB_SYNC_CREATE_FENCE : u8 = 14;
/** Opcode for xcb_sync_trigger_fence. */
pub static XCB_SYNC_TRIGGER_FENCE : u8 = 15;
/** Opcode for xcb_sync_reset_fence. */
pub static XCB_SYNC_RESET_FENCE : u8 = 16;
/** Opcode for xcb_sync_destroy_fence. */
pub static XCB_SYNC_DESTROY_FENCE : u8 = 17;
pub struct  QueryFenceCookie<'s> { pub base : base::Cookie<'s, query_fence_cookie> }

/** Opcode for xcb_sync_query_fence. */
pub static XCB_SYNC_QUERY_FENCE : u8 = 18;
pub struct QueryFenceReply { base:  base::Reply<query_fence_reply> }
fn mk_reply_query_fence_reply(reply:*mut query_fence_reply) -> QueryFenceReply { QueryFenceReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_sync_await_fence. */
pub static XCB_SYNC_AWAIT_FENCE : u8 = 19;
/** Opcode for xcb_sync_counter_notify. */
pub static XCB_SYNC_COUNTER_NOTIFY : u8 = 0;
pub struct CounterNotifyEvent {pub base : base::Event<counter_notify_event>}
/** Opcode for xcb_sync_alarm_notify. */
pub static XCB_SYNC_ALARM_NOTIFY : u8 = 1;
pub struct AlarmNotifyEvent {pub base : base::Event<alarm_notify_event>}

impl Iterator for AlarmIterator {
    type Item = Alarm;
    fn next(&mut self) -> Option<Alarm> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut alarm_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_sync_alarm_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Iterator for CounterIterator {
    type Item = Counter;
    fn next(&mut self) -> Option<Counter> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut counter_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_sync_counter_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type Fence = fence;


impl Iterator for FenceIterator {
    type Item = Fence;
    fn next(&mut self) -> Option<Fence> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut fence_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_sync_fence_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl Int64 {
  pub fn hi(&mut self) -> i32 {
    unsafe { accessor!(hi -> i32, self.base.strct) }
  }

  pub fn lo(&mut self) -> u32 {
    unsafe { accessor!(lo -> u32, self.base.strct) }
  }

}

impl Iterator for Int64Iterator {
    type Item = Int64;
    fn next(&mut self) -> Option<Int64> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut int64_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_sync_int64_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Systemcounter {pub base : base::Struct<systemcounter> }


impl Systemcounter {
  pub fn counter(&mut self) -> Counter {
    unsafe { accessor!(counter -> Counter, self.base.strct) }
  }

  pub fn resolution(&self) -> Int64 {
    unsafe { mem::transmute(self.base.strct.resolution) }
  }
  pub fn name(&mut self) -> String {
    unsafe { accessor!(str, xcb_sync_systemcounter_name_length, xcb_sync_systemcounter_name, self.base.strct) }
  }

}

impl Iterator for SystemcounterIterator {
    type Item = Systemcounter;
    fn next(&mut self) -> Option<Systemcounter> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut systemcounter_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_sync_systemcounter_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Trigger {pub base : base::Struct<trigger> }


impl Trigger {
  pub fn counter(&mut self) -> Counter {
    unsafe { accessor!(counter -> Counter, self.base.strct) }
  }

  pub fn wait_type(&mut self) -> u32 {
    unsafe { accessor!(wait_type -> u32, self.base.strct) }
  }

  pub fn wait_value(&self) -> Int64 {
    unsafe { mem::transmute(self.base.strct.wait_value) }
  }
  pub fn test_type(&mut self) -> u32 {
    unsafe { accessor!(test_type -> u32, self.base.strct) }
  }

}

impl Iterator for TriggerIterator {
    type Item = Trigger;
    fn next(&mut self) -> Option<Trigger> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut trigger_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_sync_trigger_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct Waitcondition {pub base : base::Struct<waitcondition> }


impl Waitcondition {
  pub fn trigger(&self) -> Trigger {
    unsafe { mem::transmute(self.base.strct.trigger) }
  }
  pub fn event_threshold(&self) -> Int64 {
    unsafe { mem::transmute(self.base.strct.event_threshold) }
  }
}

impl Iterator for WaitconditionIterator {
    type Item = Waitcondition;
    fn next(&mut self) -> Option<Waitcondition> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut waitcondition_iterator = mem::transmute(self);
            let data = (*iter).data;
            xcb_sync_waitcondition_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn Initialize<'r> (c : &'r Connection,
                   desired_major_version : u8,
                   desired_minor_version : u8) -> InitializeCookie<'r> {
  unsafe {
    let cookie = xcb_sync_initialize(c.get_raw_conn(),
        desired_major_version as u8, //1
        desired_minor_version as u8); //2
    InitializeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn InitializeUnchecked<'r> (c : &'r Connection,
                            desired_major_version : u8,
                            desired_minor_version : u8) -> InitializeCookie<'r> {
  unsafe {
    let cookie = xcb_sync_initialize_unchecked(c.get_raw_conn(),
        desired_major_version as u8, //1
        desired_minor_version as u8); //2
    InitializeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl InitializeReply {
  pub fn major_version(&mut self) -> u8 {
    unsafe { accessor!(major_version -> u8, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u8 {
    unsafe { accessor!(minor_version -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(InitializeCookie<'s>, mk_reply_initialize_reply, InitializeReply, xcb_sync_initialize_reply);

pub struct ListSystemCountersReply { base:  base::Reply<list_system_counters_reply> }
fn mk_reply_list_system_counters_reply(reply:*mut list_system_counters_reply) -> ListSystemCountersReply { ListSystemCountersReply { base : base::mk_reply(reply) } }
pub fn ListSystemCounters<'r> (c : &'r Connection) -> ListSystemCountersCookie<'r> {
  unsafe {
    let cookie = xcb_sync_list_system_counters(c.get_raw_conn());
    ListSystemCountersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListSystemCountersUnchecked<'r> (c : &'r Connection) -> ListSystemCountersCookie<'r> {
  unsafe {
    let cookie = xcb_sync_list_system_counters_unchecked(c.get_raw_conn());
    ListSystemCountersCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListSystemCountersReply {
  pub fn counters(&mut self) -> SystemcounterIterator {
    unsafe { accessor!(SystemcounterIterator, xcb_sync_list_system_counters_counters_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListSystemCountersCookie<'s>, mk_reply_list_system_counters_reply, ListSystemCountersReply, xcb_sync_list_system_counters_reply);

pub fn CreateCounterChecked<'r> (c : &'r Connection,
                             id : Counter,
                             initial_value : Int64) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_create_counter_checked(c.get_raw_conn(),
        id as counter, //1
        initial_value.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateCounter<'r> (c : &'r Connection,
                      id : Counter,
                      initial_value : Int64) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_create_counter(c.get_raw_conn(),
        id as counter, //1
        initial_value.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyCounterChecked<'r> (c : &'r Connection,
                              counter : Counter) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_destroy_counter_checked(c.get_raw_conn(),
        counter as counter); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyCounter<'r> (c : &'r Connection,
                       counter : Counter) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_destroy_counter(c.get_raw_conn(),
        counter as counter); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryCounter<'r> (c : &'r Connection,
                     counter : Counter) -> QueryCounterCookie<'r> {
  unsafe {
    let cookie = xcb_sync_query_counter(c.get_raw_conn(),
        counter as counter); //1
    QueryCounterCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryCounterUnchecked<'r> (c : &'r Connection,
                              counter : Counter) -> QueryCounterCookie<'r> {
  unsafe {
    let cookie = xcb_sync_query_counter_unchecked(c.get_raw_conn(),
        counter as counter); //1
    QueryCounterCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryCounterReply {
  pub fn counter_value(&self) -> Int64 {
    unsafe { mem::transmute((*self.base.reply).counter_value) }
  }
}
impl_reply_cookie!(QueryCounterCookie<'s>, mk_reply_query_counter_reply, QueryCounterReply, xcb_sync_query_counter_reply);

pub fn AwaitChecked<'r> (c : &'r Connection,
                     wait_list : &[Waitcondition]) -> base::VoidCookie<'r> {
  unsafe {
    let wait_list_len = wait_list.len();
    let wait_list_ptr = wait_list.as_ptr();
    let cookie = xcb_sync_await_checked(c.get_raw_conn(),
        wait_list_len as u32, //1
        wait_list_ptr as *mut waitcondition); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn Await<'r> (c : &'r Connection,
              wait_list : &[Waitcondition]) -> base::VoidCookie<'r> {
  unsafe {
    let wait_list_len = wait_list.len();
    let wait_list_ptr = wait_list.as_ptr();
    let cookie = xcb_sync_await(c.get_raw_conn(),
        wait_list_len as u32, //1
        wait_list_ptr as *mut waitcondition); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeCounterChecked<'r> (c : &'r Connection,
                             counter : Counter,
                             amount : Int64) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_change_counter_checked(c.get_raw_conn(),
        counter as counter, //1
        amount.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeCounter<'r> (c : &'r Connection,
                      counter : Counter,
                      amount : Int64) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_change_counter(c.get_raw_conn(),
        counter as counter, //1
        amount.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetCounterChecked<'r> (c : &'r Connection,
                          counter : Counter,
                          value : Int64) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_set_counter_checked(c.get_raw_conn(),
        counter as counter, //1
        value.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetCounter<'r> (c : &'r Connection,
                   counter : Counter,
                   value : Int64) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_set_counter(c.get_raw_conn(),
        counter as counter, //1
        value.base.strct); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn CreateAlarmChecked<'r> (c : &'r Connection,
                           id : Alarm,
                           value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_sync_create_alarm_checked(c.get_raw_conn(),
        id as alarm, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateAlarm<'r> (c : &'r Connection,
                    id : Alarm,
                    value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_sync_create_alarm(c.get_raw_conn(),
        id as alarm, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeAlarmChecked<'r> (c : &'r Connection,
                           id : Alarm,
                           value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_sync_change_alarm_checked(c.get_raw_conn(),
        id as alarm, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeAlarm<'r> (c : &'r Connection,
                    id : Alarm,
                    value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_sync_change_alarm(c.get_raw_conn(),
        id as alarm, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyAlarmChecked<'r> (c : &'r Connection,
                            alarm : Alarm) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_destroy_alarm_checked(c.get_raw_conn(),
        alarm as alarm); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyAlarm<'r> (c : &'r Connection,
                     alarm : Alarm) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_destroy_alarm(c.get_raw_conn(),
        alarm as alarm); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryAlarm<'r> (c : &'r Connection,
                   alarm : Alarm) -> QueryAlarmCookie<'r> {
  unsafe {
    let cookie = xcb_sync_query_alarm(c.get_raw_conn(),
        alarm as alarm); //1
    QueryAlarmCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryAlarmUnchecked<'r> (c : &'r Connection,
                            alarm : Alarm) -> QueryAlarmCookie<'r> {
  unsafe {
    let cookie = xcb_sync_query_alarm_unchecked(c.get_raw_conn(),
        alarm as alarm); //1
    QueryAlarmCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryAlarmReply {
  pub fn trigger(&self) -> Trigger {
    unsafe { mem::transmute((*self.base.reply).trigger) }
  }
  pub fn delta(&self) -> Int64 {
    unsafe { mem::transmute((*self.base.reply).delta) }
  }
  pub fn events(&mut self) -> u8 {
    unsafe { accessor!(events -> u8, (*self.base.reply)) }
  }

  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryAlarmCookie<'s>, mk_reply_query_alarm_reply, QueryAlarmReply, xcb_sync_query_alarm_reply);

pub fn SetPriorityChecked<'r> (c : &'r Connection,
                           id : u32,
                           priority : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_set_priority_checked(c.get_raw_conn(),
        id as u32, //1
        priority as i32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetPriority<'r> (c : &'r Connection,
                    id : u32,
                    priority : i32) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_set_priority(c.get_raw_conn(),
        id as u32, //1
        priority as i32); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPriority<'r> (c : &'r Connection,
                    id : u32) -> GetPriorityCookie<'r> {
  unsafe {
    let cookie = xcb_sync_get_priority(c.get_raw_conn(),
        id as u32); //1
    GetPriorityCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetPriorityUnchecked<'r> (c : &'r Connection,
                             id : u32) -> GetPriorityCookie<'r> {
  unsafe {
    let cookie = xcb_sync_get_priority_unchecked(c.get_raw_conn(),
        id as u32); //1
    GetPriorityCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetPriorityReply {
  pub fn priority(&mut self) -> i32 {
    unsafe { accessor!(priority -> i32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetPriorityCookie<'s>, mk_reply_get_priority_reply, GetPriorityReply, xcb_sync_get_priority_reply);

pub fn CreateFenceChecked<'r> (c : &'r Connection,
                           drawable : xproto::Drawable,
                           fence : Fence,
                           initially_triggered : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_create_fence_checked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        fence as fence, //2
        initially_triggered as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CreateFence<'r> (c : &'r Connection,
                    drawable : xproto::Drawable,
                    fence : Fence,
                    initially_triggered : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_create_fence(c.get_raw_conn(),
        drawable as ffi::xproto::drawable, //1
        fence as fence, //2
        initially_triggered as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn TriggerFenceChecked<'r> (c : &'r Connection,
                            fence : Fence) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_trigger_fence_checked(c.get_raw_conn(),
        fence as fence); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn TriggerFence<'r> (c : &'r Connection,
                     fence : Fence) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_trigger_fence(c.get_raw_conn(),
        fence as fence); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ResetFenceChecked<'r> (c : &'r Connection,
                          fence : Fence) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_reset_fence_checked(c.get_raw_conn(),
        fence as fence); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ResetFence<'r> (c : &'r Connection,
                   fence : Fence) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_reset_fence(c.get_raw_conn(),
        fence as fence); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DestroyFenceChecked<'r> (c : &'r Connection,
                            fence : Fence) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_destroy_fence_checked(c.get_raw_conn(),
        fence as fence); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DestroyFence<'r> (c : &'r Connection,
                     fence : Fence) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_sync_destroy_fence(c.get_raw_conn(),
        fence as fence); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryFence<'r> (c : &'r Connection,
                   fence : Fence) -> QueryFenceCookie<'r> {
  unsafe {
    let cookie = xcb_sync_query_fence(c.get_raw_conn(),
        fence as fence); //1
    QueryFenceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryFenceUnchecked<'r> (c : &'r Connection,
                            fence : Fence) -> QueryFenceCookie<'r> {
  unsafe {
    let cookie = xcb_sync_query_fence_unchecked(c.get_raw_conn(),
        fence as fence); //1
    QueryFenceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryFenceReply {
  pub fn triggered(&mut self) -> u8 {
    unsafe { accessor!(triggered -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryFenceCookie<'s>, mk_reply_query_fence_reply, QueryFenceReply, xcb_sync_query_fence_reply);

pub fn AwaitFenceChecked<'r> (c : &'r Connection,
                          fence_list : &[Fence]) -> base::VoidCookie<'r> {
  unsafe {
    let fence_list_len = fence_list.len();
    let fence_list_ptr = fence_list.as_ptr();
    let cookie = xcb_sync_await_fence_checked(c.get_raw_conn(),
        fence_list_len as u32, //1
        fence_list_ptr as *mut fence); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn AwaitFence<'r> (c : &'r Connection,
                   fence_list : &[Fence]) -> base::VoidCookie<'r> {
  unsafe {
    let fence_list_len = fence_list.len();
    let fence_list_ptr = fence_list.as_ptr();
    let cookie = xcb_sync_await_fence(c.get_raw_conn(),
        fence_list_len as u32, //1
        fence_list_ptr as *mut fence); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl CounterNotifyEvent {
  pub fn kind(&mut self) -> u8 {
    unsafe { accessor!(kind -> u8, (*self.base.event)) }
  }

  pub fn counter(&mut self) -> Counter {
    unsafe { accessor!(counter -> Counter, (*self.base.event)) }
  }

  pub fn wait_value(&self) -> Int64 {
    unsafe { mem::transmute((*self.base.event).wait_value) }
  }
  pub fn counter_value(&self) -> Int64 {
    unsafe { mem::transmute((*self.base.event).counter_value) }
  }
  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn count(&mut self) -> u16 {
    unsafe { accessor!(count -> u16, (*self.base.event)) }
  }

  pub fn destroyed(&mut self) -> u8 {
    unsafe { accessor!(destroyed -> u8, (*self.base.event)) }
  }

  pub fn new(kind : u8,
         counter : Counter,
         wait_value : Int64,
         counter_value : Int64,
         timestamp : xproto::Timestamp,
         count : u16,
         destroyed : u8) -> CounterNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut counter_notify_event;
      (*raw).kind = kind;
      (*raw).counter = counter;
      (*raw).wait_value = wait_value.base.strct;
      (*raw).counter_value = counter_value.base.strct;
      (*raw).timestamp = timestamp;
      (*raw).count = count;
      (*raw).destroyed = destroyed;
      CounterNotifyEvent { base : Event { event : raw as *mut counter_notify_event }}
    }
  }
}

impl AlarmNotifyEvent {
  pub fn kind(&mut self) -> u8 {
    unsafe { accessor!(kind -> u8, (*self.base.event)) }
  }

  pub fn alarm(&mut self) -> Alarm {
    unsafe { accessor!(alarm -> Alarm, (*self.base.event)) }
  }

  pub fn counter_value(&self) -> Int64 {
    unsafe { mem::transmute((*self.base.event).counter_value) }
  }
  pub fn alarm_value(&self) -> Int64 {
    unsafe { mem::transmute((*self.base.event).alarm_value) }
  }
  pub fn timestamp(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(timestamp -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u8 {
    unsafe { accessor!(state -> u8, (*self.base.event)) }
  }

  pub fn new(kind : u8,
         alarm : Alarm,
         counter_value : Int64,
         alarm_value : Int64,
         timestamp : xproto::Timestamp,
         state : u8) -> AlarmNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut alarm_notify_event;
      (*raw).kind = kind;
      (*raw).alarm = alarm;
      (*raw).counter_value = counter_value.base.strct;
      (*raw).alarm_value = alarm_value.base.strct;
      (*raw).timestamp = timestamp;
      (*raw).state = state;
      AlarmNotifyEvent { base : Event { event : raw as *mut alarm_notify_event }}
    }
  }
}

