/*
 * This file generated automatically from xinput.xml by r_client.py.
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
use ffi::xinput::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;
pub type KeyCode = xcb_input_key_code_t;

pub type KeyCodeIterator = xcb_input_key_code_iterator_t;

pub type EventClassIterator = xcb_input_event_class_iterator_t;


pub type xcb_input_valuator_mode_t = c_uint;//{
    pub static XCB_INPUT_VALUATOR_MODE_RELATIVE : xcb_input_valuator_mode_t = 0;
    pub static XCB_INPUT_VALUATOR_MODE_ABSOLUTE : xcb_input_valuator_mode_t = 1;
//}

pub type xcb_input_propagate_mode_t = c_uint;//{
    pub static XCB_INPUT_PROPAGATE_MODE_ADD_TO_LIST : xcb_input_propagate_mode_t = 0;
    pub static XCB_INPUT_PROPAGATE_MODE_DELETE_FROM_LIST : xcb_input_propagate_mode_t = 1;
//}
pub struct  GetExtensionVersionCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_extension_version_cookie_t> }

/** Opcode for xcb_input_get_extension_version. */
pub static XCB_INPUT_GET_EXTENSION_VERSION : u8 = 1;
pub struct GetExtensionVersionReply { base:  base::Reply<xcb_input_get_extension_version_reply_t> }
fn mk_reply_xcb_input_get_extension_version_reply_t(reply:*mut xcb_input_get_extension_version_reply_t) -> GetExtensionVersionReply { GetExtensionVersionReply { base : base::mk_reply(reply) } }

pub type xcb_input_device_use_t = c_uint;//{
    pub static XCB_INPUT_DEVICE_USE_IS_X_POINTER : xcb_input_device_use_t = 0;
    pub static XCB_INPUT_DEVICE_USE_IS_X_KEYBOARD : xcb_input_device_use_t = 1;
    pub static XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_DEVICE : xcb_input_device_use_t = 2;
    pub static XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_KEYBOARD : xcb_input_device_use_t = 3;
    pub static XCB_INPUT_DEVICE_USE_IS_X_EXTENSION_POINTER : xcb_input_device_use_t = 4;
//}
pub struct DeviceInfo {pub base : base::Struct<xcb_input_device_info_t> }

pub type DeviceInfoIterator = xcb_input_device_info_iterator_t;

pub struct  ListInputDevicesCookie<'s> { pub base : base::Cookie<'s, xcb_input_list_input_devices_cookie_t> }

/** Opcode for xcb_input_list_input_devices. */
pub static XCB_INPUT_LIST_INPUT_DEVICES : u8 = 2;

pub type xcb_input_input_class_t = c_uint;//{
    pub static XCB_INPUT_INPUT_CLASS_KEY : xcb_input_input_class_t = 0;
    pub static XCB_INPUT_INPUT_CLASS_BUTTON : xcb_input_input_class_t = 1;
    pub static XCB_INPUT_INPUT_CLASS_VALUATOR : xcb_input_input_class_t = 2;
    pub static XCB_INPUT_INPUT_CLASS_FEEDBACK : xcb_input_input_class_t = 3;
    pub static XCB_INPUT_INPUT_CLASS_PROXIMITY : xcb_input_input_class_t = 4;
    pub static XCB_INPUT_INPUT_CLASS_FOCUS : xcb_input_input_class_t = 5;
    pub static XCB_INPUT_INPUT_CLASS_OTHER : xcb_input_input_class_t = 6;
//}
pub struct InputInfo {pub base : base::Struct<xcb_input_input_info_t> }

pub type InputInfoIterator = xcb_input_input_info_iterator_t;

pub type KeyInfoIterator = xcb_input_key_info_iterator_t;

pub type ButtonInfoIterator = xcb_input_button_info_iterator_t;

pub type AxisInfoIterator = xcb_input_axis_info_iterator_t;

pub type ValuatorInfoIterator = xcb_input_valuator_info_iterator_t;

pub type InputClassInfoIterator = xcb_input_input_class_info_iterator_t;

pub struct  OpenDeviceCookie<'s> { pub base : base::Cookie<'s, xcb_input_open_device_cookie_t> }

/** Opcode for xcb_input_open_device. */
pub static XCB_INPUT_OPEN_DEVICE : u8 = 3;
/** Opcode for xcb_input_close_device. */
pub static XCB_INPUT_CLOSE_DEVICE : u8 = 4;
pub struct  SetDeviceModeCookie<'s> { pub base : base::Cookie<'s, xcb_input_set_device_mode_cookie_t> }

/** Opcode for xcb_input_set_device_mode. */
pub static XCB_INPUT_SET_DEVICE_MODE : u8 = 5;
pub struct SetDeviceModeReply { base:  base::Reply<xcb_input_set_device_mode_reply_t> }
fn mk_reply_xcb_input_set_device_mode_reply_t(reply:*mut xcb_input_set_device_mode_reply_t) -> SetDeviceModeReply { SetDeviceModeReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_input_select_extension_event. */
pub static XCB_INPUT_SELECT_EXTENSION_EVENT : u8 = 6;
pub struct  GetSelectedExtensionEventsCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_selected_extension_events_cookie_t> }

/** Opcode for xcb_input_get_selected_extension_events. */
pub static XCB_INPUT_GET_SELECTED_EXTENSION_EVENTS : u8 = 7;
/** Opcode for xcb_input_change_device_dont_propagate_list. */
pub static XCB_INPUT_CHANGE_DEVICE_DONT_PROPAGATE_LIST : u8 = 8;
pub struct  GetDeviceDontPropagateListCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_device_dont_propagate_list_cookie_t> }

/** Opcode for xcb_input_get_device_dont_propagate_list. */
pub static XCB_INPUT_GET_DEVICE_DONT_PROPAGATE_LIST : u8 = 9;
pub struct  GetDeviceMotionEventsCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_device_motion_events_cookie_t> }

/** Opcode for xcb_input_get_device_motion_events. */
pub static XCB_INPUT_GET_DEVICE_MOTION_EVENTS : u8 = 10;
pub struct GetDeviceMotionEventsReply { base:  base::Reply<xcb_input_get_device_motion_events_reply_t> }
fn mk_reply_xcb_input_get_device_motion_events_reply_t(reply:*mut xcb_input_get_device_motion_events_reply_t) -> GetDeviceMotionEventsReply { GetDeviceMotionEventsReply { base : base::mk_reply(reply) } }
pub type DeviceTimeCoordIterator = xcb_input_device_time_coord_iterator_t;

pub struct  ChangeKeyboardDeviceCookie<'s> { pub base : base::Cookie<'s, xcb_input_change_keyboard_device_cookie_t> }

/** Opcode for xcb_input_change_keyboard_device. */
pub static XCB_INPUT_CHANGE_KEYBOARD_DEVICE : u8 = 11;
pub struct ChangeKeyboardDeviceReply { base:  base::Reply<xcb_input_change_keyboard_device_reply_t> }
fn mk_reply_xcb_input_change_keyboard_device_reply_t(reply:*mut xcb_input_change_keyboard_device_reply_t) -> ChangeKeyboardDeviceReply { ChangeKeyboardDeviceReply { base : base::mk_reply(reply) } }
pub struct  ChangePointerDeviceCookie<'s> { pub base : base::Cookie<'s, xcb_input_change_pointer_device_cookie_t> }

/** Opcode for xcb_input_change_pointer_device. */
pub static XCB_INPUT_CHANGE_POINTER_DEVICE : u8 = 12;
pub struct ChangePointerDeviceReply { base:  base::Reply<xcb_input_change_pointer_device_reply_t> }
fn mk_reply_xcb_input_change_pointer_device_reply_t(reply:*mut xcb_input_change_pointer_device_reply_t) -> ChangePointerDeviceReply { ChangePointerDeviceReply { base : base::mk_reply(reply) } }
pub struct  GrabDeviceCookie<'s> { pub base : base::Cookie<'s, xcb_input_grab_device_cookie_t> }

/** Opcode for xcb_input_grab_device. */
pub static XCB_INPUT_GRAB_DEVICE : u8 = 13;
pub struct GrabDeviceReply { base:  base::Reply<xcb_input_grab_device_reply_t> }
fn mk_reply_xcb_input_grab_device_reply_t(reply:*mut xcb_input_grab_device_reply_t) -> GrabDeviceReply { GrabDeviceReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_input_ungrab_device. */
pub static XCB_INPUT_UNGRAB_DEVICE : u8 = 14;
/** Opcode for xcb_input_grab_device_key. */
pub static XCB_INPUT_GRAB_DEVICE_KEY : u8 = 15;
/** Opcode for xcb_input_ungrab_device_key. */
pub static XCB_INPUT_UNGRAB_DEVICE_KEY : u8 = 16;
/** Opcode for xcb_input_grab_device_button. */
pub static XCB_INPUT_GRAB_DEVICE_BUTTON : u8 = 17;
/** Opcode for xcb_input_ungrab_device_button. */
pub static XCB_INPUT_UNGRAB_DEVICE_BUTTON : u8 = 18;

pub type xcb_input_device_input_mode_t = c_uint;//{
    pub static XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_THIS_DEVICE : xcb_input_device_input_mode_t = 1;
    pub static XCB_INPUT_DEVICE_INPUT_MODE_SYNC_THIS_DEVICE : xcb_input_device_input_mode_t = 2;
    pub static XCB_INPUT_DEVICE_INPUT_MODE_REPLAY_THIS_DEVICE : xcb_input_device_input_mode_t = 3;
    pub static XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_OTHER_DEVICES : xcb_input_device_input_mode_t = 4;
    pub static XCB_INPUT_DEVICE_INPUT_MODE_ASYNC_ALL : xcb_input_device_input_mode_t = 5;
    pub static XCB_INPUT_DEVICE_INPUT_MODE_SYNC_ALL : xcb_input_device_input_mode_t = 6;
//}
/** Opcode for xcb_input_allow_device_events. */
pub static XCB_INPUT_ALLOW_DEVICE_EVENTS : u8 = 19;
pub struct  GetDeviceFocusCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_device_focus_cookie_t> }

/** Opcode for xcb_input_get_device_focus. */
pub static XCB_INPUT_GET_DEVICE_FOCUS : u8 = 20;
pub struct GetDeviceFocusReply { base:  base::Reply<xcb_input_get_device_focus_reply_t> }
fn mk_reply_xcb_input_get_device_focus_reply_t(reply:*mut xcb_input_get_device_focus_reply_t) -> GetDeviceFocusReply { GetDeviceFocusReply { base : base::mk_reply(reply) } }
/** Opcode for xcb_input_set_device_focus. */
pub static XCB_INPUT_SET_DEVICE_FOCUS : u8 = 21;
pub struct  GetFeedbackControlCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_feedback_control_cookie_t> }

/** Opcode for xcb_input_get_feedback_control. */
pub static XCB_INPUT_GET_FEEDBACK_CONTROL : u8 = 22;
pub struct GetFeedbackControlReply { base:  base::Reply<xcb_input_get_feedback_control_reply_t> }
fn mk_reply_xcb_input_get_feedback_control_reply_t(reply:*mut xcb_input_get_feedback_control_reply_t) -> GetFeedbackControlReply { GetFeedbackControlReply { base : base::mk_reply(reply) } }

pub type xcb_input_feedback_class_t = c_uint;//{
    pub static XCB_INPUT_FEEDBACK_CLASS_KEYBOARD : xcb_input_feedback_class_t = 1;
    pub static XCB_INPUT_FEEDBACK_CLASS_POINTER : xcb_input_feedback_class_t = 2;
    pub static XCB_INPUT_FEEDBACK_CLASS_STRING : xcb_input_feedback_class_t = 3;
    pub static XCB_INPUT_FEEDBACK_CLASS_INTEGER : xcb_input_feedback_class_t = 4;
    pub static XCB_INPUT_FEEDBACK_CLASS_LED : xcb_input_feedback_class_t = 5;
    pub static XCB_INPUT_FEEDBACK_CLASS_BELL : xcb_input_feedback_class_t = 6;
//}
pub struct FeedbackState {pub base : base::Struct<xcb_input_feedback_state_t> }

pub type FeedbackStateIterator = xcb_input_feedback_state_iterator_t;

pub type KbdFeedbackStateIterator = xcb_input_kbd_feedback_state_iterator_t;

pub type PtrFeedbackStateIterator = xcb_input_ptr_feedback_state_iterator_t;

pub type IntegerFeedbackStateIterator = xcb_input_integer_feedback_state_iterator_t;

pub type StringFeedbackStateIterator = xcb_input_string_feedback_state_iterator_t;

pub type BellFeedbackStateIterator = xcb_input_bell_feedback_state_iterator_t;

pub type LedFeedbackStateIterator = xcb_input_led_feedback_state_iterator_t;

pub type FeedbackCtlIterator = xcb_input_feedback_ctl_iterator_t;

pub type KbdFeedbackCtlIterator = xcb_input_kbd_feedback_ctl_iterator_t;

pub type PtrFeedbackCtlIterator = xcb_input_ptr_feedback_ctl_iterator_t;

pub type IntegerFeedbackCtlIterator = xcb_input_integer_feedback_ctl_iterator_t;

pub type StringFeedbackCtlIterator = xcb_input_string_feedback_ctl_iterator_t;

pub type BellFeedbackCtlIterator = xcb_input_bell_feedback_ctl_iterator_t;

pub type LedFeedbackCtlIterator = xcb_input_led_feedback_ctl_iterator_t;

pub struct  GetDeviceKeyMappingCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_device_key_mapping_cookie_t> }

/** Opcode for xcb_input_get_device_key_mapping. */
pub static XCB_INPUT_GET_DEVICE_KEY_MAPPING : u8 = 24;
/** Opcode for xcb_input_change_device_key_mapping. */
pub static XCB_INPUT_CHANGE_DEVICE_KEY_MAPPING : u8 = 25;
pub struct  GetDeviceModifierMappingCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_device_modifier_mapping_cookie_t> }

/** Opcode for xcb_input_get_device_modifier_mapping. */
pub static XCB_INPUT_GET_DEVICE_MODIFIER_MAPPING : u8 = 26;
pub struct  SetDeviceModifierMappingCookie<'s> { pub base : base::Cookie<'s, xcb_input_set_device_modifier_mapping_cookie_t> }

/** Opcode for xcb_input_set_device_modifier_mapping. */
pub static XCB_INPUT_SET_DEVICE_MODIFIER_MAPPING : u8 = 27;
pub struct SetDeviceModifierMappingReply { base:  base::Reply<xcb_input_set_device_modifier_mapping_reply_t> }
fn mk_reply_xcb_input_set_device_modifier_mapping_reply_t(reply:*mut xcb_input_set_device_modifier_mapping_reply_t) -> SetDeviceModifierMappingReply { SetDeviceModifierMappingReply { base : base::mk_reply(reply) } }
pub struct  GetDeviceButtonMappingCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_device_button_mapping_cookie_t> }

/** Opcode for xcb_input_get_device_button_mapping. */
pub static XCB_INPUT_GET_DEVICE_BUTTON_MAPPING : u8 = 28;
pub struct  SetDeviceButtonMappingCookie<'s> { pub base : base::Cookie<'s, xcb_input_set_device_button_mapping_cookie_t> }

/** Opcode for xcb_input_set_device_button_mapping. */
pub static XCB_INPUT_SET_DEVICE_BUTTON_MAPPING : u8 = 29;
pub struct SetDeviceButtonMappingReply { base:  base::Reply<xcb_input_set_device_button_mapping_reply_t> }
fn mk_reply_xcb_input_set_device_button_mapping_reply_t(reply:*mut xcb_input_set_device_button_mapping_reply_t) -> SetDeviceButtonMappingReply { SetDeviceButtonMappingReply { base : base::mk_reply(reply) } }
pub struct  QueryDeviceStateCookie<'s> { pub base : base::Cookie<'s, xcb_input_query_device_state_cookie_t> }

/** Opcode for xcb_input_query_device_state. */
pub static XCB_INPUT_QUERY_DEVICE_STATE : u8 = 30;
pub struct QueryDeviceStateReply { base:  base::Reply<xcb_input_query_device_state_reply_t> }
fn mk_reply_xcb_input_query_device_state_reply_t(reply:*mut xcb_input_query_device_state_reply_t) -> QueryDeviceStateReply { QueryDeviceStateReply { base : base::mk_reply(reply) } }
pub type InputStateIterator = xcb_input_input_state_iterator_t;

pub type KeyStateIterator = xcb_input_key_state_iterator_t;

pub type ButtonStateIterator = xcb_input_button_state_iterator_t;

pub type ValuatorStateIterator = xcb_input_valuator_state_iterator_t;

/** Opcode for xcb_input_send_extension_event. */
pub static XCB_INPUT_SEND_EXTENSION_EVENT : u8 = 31;
/** Opcode for xcb_input_device_bell. */
pub static XCB_INPUT_DEVICE_BELL : u8 = 32;
pub struct  SetDeviceValuatorsCookie<'s> { pub base : base::Cookie<'s, xcb_input_set_device_valuators_cookie_t> }

/** Opcode for xcb_input_set_device_valuators. */
pub static XCB_INPUT_SET_DEVICE_VALUATORS : u8 = 33;
pub struct SetDeviceValuatorsReply { base:  base::Reply<xcb_input_set_device_valuators_reply_t> }
fn mk_reply_xcb_input_set_device_valuators_reply_t(reply:*mut xcb_input_set_device_valuators_reply_t) -> SetDeviceValuatorsReply { SetDeviceValuatorsReply { base : base::mk_reply(reply) } }
pub struct  GetDeviceControlCookie<'s> { pub base : base::Cookie<'s, xcb_input_get_device_control_cookie_t> }

/** Opcode for xcb_input_get_device_control. */
pub static XCB_INPUT_GET_DEVICE_CONTROL : u8 = 34;
pub struct GetDeviceControlReply { base:  base::Reply<xcb_input_get_device_control_reply_t> }
fn mk_reply_xcb_input_get_device_control_reply_t(reply:*mut xcb_input_get_device_control_reply_t) -> GetDeviceControlReply { GetDeviceControlReply { base : base::mk_reply(reply) } }
pub type DeviceStateIterator = xcb_input_device_state_iterator_t;

pub type DeviceResolutionStateIterator = xcb_input_device_resolution_state_iterator_t;

pub type DeviceAbsCalibStateIterator = xcb_input_device_abs_calib_state_iterator_t;

pub type DeviceAbsAreaStateIterator = xcb_input_device_abs_area_state_iterator_t;

pub type DeviceCoreStateIterator = xcb_input_device_core_state_iterator_t;

pub type DeviceEnableStateIterator = xcb_input_device_enable_state_iterator_t;

pub type DeviceCtlIterator = xcb_input_device_ctl_iterator_t;

pub type DeviceResolutionCtlIterator = xcb_input_device_resolution_ctl_iterator_t;

pub type DeviceAbsCalibCtlIterator = xcb_input_device_abs_calib_ctl_iterator_t;

pub type DeviceAbsAreaCtrlIterator = xcb_input_device_abs_area_ctrl_iterator_t;

pub type DeviceCoreCtrlIterator = xcb_input_device_core_ctrl_iterator_t;

pub type DeviceEnableCtrlIterator = xcb_input_device_enable_ctrl_iterator_t;

/** Opcode for xcb_input_device_valuator. */
pub static XCB_INPUT_DEVICE_VALUATOR : u8 = 0;
pub struct DeviceValuatorEvent {pub base : base::Event<xcb_input_device_valuator_event_t>}
/** Opcode for xcb_input_device_key_press. */
pub static XCB_INPUT_DEVICE_KEY_PRESS : u8 = 1;
pub struct DeviceKeyPressEvent {pub base : base::Event<xcb_input_device_key_press_event_t>}
/** Opcode for xcb_input_device_key_release. */
pub static XCB_INPUT_DEVICE_KEY_RELEASE : u8 = 2;
pub struct DeviceKeyReleaseEvent {pub base : base::Event<xcb_input_device_key_release_event_t>}
/** Opcode for xcb_input_device_button_press. */
pub static XCB_INPUT_DEVICE_BUTTON_PRESS : u8 = 3;
pub struct DeviceButtonPressEvent {pub base : base::Event<xcb_input_device_button_press_event_t>}
/** Opcode for xcb_input_device_button_release. */
pub static XCB_INPUT_DEVICE_BUTTON_RELEASE : u8 = 4;
pub struct DeviceButtonReleaseEvent {pub base : base::Event<xcb_input_device_button_release_event_t>}
/** Opcode for xcb_input_device_motion_notify. */
pub static XCB_INPUT_DEVICE_MOTION_NOTIFY : u8 = 5;
pub struct DeviceMotionNotifyEvent {pub base : base::Event<xcb_input_device_motion_notify_event_t>}
/** Opcode for xcb_input_proximity_in. */
pub static XCB_INPUT_PROXIMITY_IN : u8 = 8;
pub struct ProximityInEvent {pub base : base::Event<xcb_input_proximity_in_event_t>}
/** Opcode for xcb_input_proximity_out. */
pub static XCB_INPUT_PROXIMITY_OUT : u8 = 9;
pub struct ProximityOutEvent {pub base : base::Event<xcb_input_proximity_out_event_t>}
/** Opcode for xcb_input_focus_in. */
pub static XCB_INPUT_FOCUS_IN : u8 = 6;
pub struct FocusInEvent {pub base : base::Event<xcb_input_focus_in_event_t>}
/** Opcode for xcb_input_focus_out. */
pub static XCB_INPUT_FOCUS_OUT : u8 = 7;
pub struct FocusOutEvent {pub base : base::Event<xcb_input_focus_out_event_t>}
/** Opcode for xcb_input_device_state_notify. */
pub static XCB_INPUT_DEVICE_STATE_NOTIFY : u8 = 10;
pub struct DeviceStateNotifyEvent {pub base : base::Event<xcb_input_device_state_notify_event_t>}
/** Opcode for xcb_input_device_mapping_notify. */
pub static XCB_INPUT_DEVICE_MAPPING_NOTIFY : u8 = 11;
pub struct DeviceMappingNotifyEvent {pub base : base::Event<xcb_input_device_mapping_notify_event_t>}
/** Opcode for xcb_input_change_device_notify. */
pub static XCB_INPUT_CHANGE_DEVICE_NOTIFY : u8 = 12;
pub struct ChangeDeviceNotifyEvent {pub base : base::Event<xcb_input_change_device_notify_event_t>}
/** Opcode for xcb_input_device_key_state_notify. */
pub static XCB_INPUT_DEVICE_KEY_STATE_NOTIFY : u8 = 13;
pub struct DeviceKeyStateNotifyEvent {pub base : base::Event<xcb_input_device_key_state_notify_event_t>}
/** Opcode for xcb_input_device_button_state_notify. */
pub static XCB_INPUT_DEVICE_BUTTON_STATE_NOTIFY : u8 = 14;
pub struct DeviceButtonStateNotifyEvent {pub base : base::Event<xcb_input_device_button_state_notify_event_t>}
/** Opcode for xcb_input_device_presence_notify. */
pub static XCB_INPUT_DEVICE_PRESENCE_NOTIFY : u8 = 15;
pub struct DevicePresenceNotifyEvent {pub base : base::Event<xcb_input_device_presence_notify_event_t>}
/** Opcode for xcb_input_device. */
pub static XCB_INPUT_DEVICE : u8 = 0;
pub struct DeviceError { pub base : base::Error<xcb_input_device_error_t> }
/** Opcode for xcb_input_event. */
pub static XCB_INPUT_EVENT : u8 = 1;
pub struct EventError { pub base : base::Error<xcb_input_event_error_t> }
/** Opcode for xcb_input_mode. */
pub static XCB_INPUT_MODE : u8 = 2;
pub struct ModeError { pub base : base::Error<xcb_input_mode_error_t> }
/** Opcode for xcb_input_device_busy. */
pub static XCB_INPUT_DEVICE_BUSY : u8 = 3;
pub struct DeviceBusyError { pub base : base::Error<xcb_input_device_busy_error_t> }
/** Opcode for xcb_input_class. */
pub static XCB_INPUT_CLASS : u8 = 4;
pub struct ClassError { pub base : base::Error<xcb_input_class_error_t> }


impl Iterator for KeyCodeIterator {
    type Item = KeyCode;
    fn next(&mut self) -> Option<KeyCode> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_key_code_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_key_code_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub type EventClass = xcb_input_event_class_t;


impl Iterator for EventClassIterator {
    type Item = EventClass;
    fn next(&mut self) -> Option<EventClass> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_event_class_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_event_class_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn GetExtensionVersion<'r> (c : &'r Connection,
                            name : &str) -> GetExtensionVersionCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_input_get_extension_version(c.get_raw_conn(),
        name_len as u16, //1
        name_ptr as *mut c_char); //2
    GetExtensionVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetExtensionVersionUnchecked<'r> (c : &'r Connection,
                                     name : &str) -> GetExtensionVersionCookie<'r> {
  unsafe {
    let name = (name).as_bytes();
    let name_len = name.len();
    let name_ptr = name.as_ptr();
    let cookie = xcb_input_get_extension_version_unchecked(c.get_raw_conn(),
        name_len as u16, //1
        name_ptr as *mut c_char); //2
    GetExtensionVersionCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetExtensionVersionReply {
  pub fn server_major(&mut self) -> u16 {
    unsafe { accessor!(server_major -> u16, (*self.base.reply)) }
  }

  pub fn server_minor(&mut self) -> u16 {
    unsafe { accessor!(server_minor -> u16, (*self.base.reply)) }
  }

  pub fn present(&mut self) -> u8 {
    unsafe { accessor!(present -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetExtensionVersionCookie<'s>, mk_reply_xcb_input_get_extension_version_reply_t, GetExtensionVersionReply, xcb_input_get_extension_version_reply);


impl DeviceInfo {
  pub fn device_type(&mut self) -> xproto::Atom {
    unsafe { accessor!(device_type -> xproto::Atom, self.base.strct) }
  }

  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, self.base.strct) }
  }

  pub fn num_class_info(&mut self) -> u8 {
    unsafe { accessor!(num_class_info -> u8, self.base.strct) }
  }

  pub fn device_use(&mut self) -> u8 {
    unsafe { accessor!(device_use -> u8, self.base.strct) }
  }

}

impl Iterator for DeviceInfoIterator {
    type Item = DeviceInfo;
    fn next(&mut self) -> Option<DeviceInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ListInputDevicesReply { base:  base::Reply<xcb_input_list_input_devices_reply_t> }
fn mk_reply_xcb_input_list_input_devices_reply_t(reply:*mut xcb_input_list_input_devices_reply_t) -> ListInputDevicesReply { ListInputDevicesReply { base : base::mk_reply(reply) } }
pub fn ListInputDevices<'r> (c : &'r Connection) -> ListInputDevicesCookie<'r> {
  unsafe {
    let cookie = xcb_input_list_input_devices(c.get_raw_conn());
    ListInputDevicesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ListInputDevicesUnchecked<'r> (c : &'r Connection) -> ListInputDevicesCookie<'r> {
  unsafe {
    let cookie = xcb_input_list_input_devices_unchecked(c.get_raw_conn());
    ListInputDevicesCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ListInputDevicesReply {
  pub fn devices(&mut self) -> DeviceInfoIterator {
    unsafe { accessor!(DeviceInfoIterator, xcb_input_list_input_devices_devices_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ListInputDevicesCookie<'s>, mk_reply_xcb_input_list_input_devices_reply_t, ListInputDevicesReply, xcb_input_list_input_devices_reply);


impl InputInfo {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

}

impl Iterator for InputInfoIterator {
    type Item = InputInfo;
    fn next(&mut self) -> Option<InputInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_input_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_input_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeyInfo {pub base : base::Struct<xcb_input_key_info_t> }


impl KeyInfo {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

  pub fn min_keycode(&mut self) -> KeyCode {
    unsafe { accessor!(min_keycode -> KeyCode, self.base.strct) }
  }

  pub fn max_keycode(&mut self) -> KeyCode {
    unsafe { accessor!(max_keycode -> KeyCode, self.base.strct) }
  }

  pub fn num_keys(&mut self) -> u16 {
    unsafe { accessor!(num_keys -> u16, self.base.strct) }
  }

}

impl Iterator for KeyInfoIterator {
    type Item = KeyInfo;
    fn next(&mut self) -> Option<KeyInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_key_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_key_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ButtonInfo {pub base : base::Struct<xcb_input_button_info_t> }


impl ButtonInfo {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

  pub fn num_buttons(&mut self) -> u16 {
    unsafe { accessor!(num_buttons -> u16, self.base.strct) }
  }

}

impl Iterator for ButtonInfoIterator {
    type Item = ButtonInfo;
    fn next(&mut self) -> Option<ButtonInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_button_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_button_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct AxisInfo {pub base : base::Struct<xcb_input_axis_info_t> }


impl AxisInfo {
  pub fn resolution(&mut self) -> u32 {
    unsafe { accessor!(resolution -> u32, self.base.strct) }
  }

  pub fn minimum(&mut self) -> i32 {
    unsafe { accessor!(minimum -> i32, self.base.strct) }
  }

  pub fn maximum(&mut self) -> i32 {
    unsafe { accessor!(maximum -> i32, self.base.strct) }
  }

}

impl Iterator for AxisInfoIterator {
    type Item = AxisInfo;
    fn next(&mut self) -> Option<AxisInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_axis_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_axis_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ValuatorInfo {pub base : base::Struct<xcb_input_valuator_info_t> }


impl ValuatorInfo {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

  pub fn mode(&mut self) -> u8 {
    unsafe { accessor!(mode -> u8, self.base.strct) }
  }

  pub fn motion_size(&mut self) -> u32 {
    unsafe { accessor!(motion_size -> u32, self.base.strct) }
  }

  pub fn axes(&mut self) -> AxisInfoIterator {
    unsafe { accessor!(AxisInfoIterator, xcb_input_valuator_info_axes_iterator, self.base.strct) }
  }

}

impl Iterator for ValuatorInfoIterator {
    type Item = ValuatorInfo;
    fn next(&mut self) -> Option<ValuatorInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_valuator_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_valuator_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct InputClassInfo {pub base : base::Struct<xcb_input_input_class_info_t> }


impl InputClassInfo {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn event_type_base(&mut self) -> u8 {
    unsafe { accessor!(event_type_base -> u8, self.base.strct) }
  }

}

impl Iterator for InputClassInfoIterator {
    type Item = InputClassInfo;
    fn next(&mut self) -> Option<InputClassInfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_input_class_info_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_input_class_info_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct OpenDeviceReply { base:  base::Reply<xcb_input_open_device_reply_t> }
fn mk_reply_xcb_input_open_device_reply_t(reply:*mut xcb_input_open_device_reply_t) -> OpenDeviceReply { OpenDeviceReply { base : base::mk_reply(reply) } }
pub fn OpenDevice<'r> (c : &'r Connection,
                   device_id : u8) -> OpenDeviceCookie<'r> {
  unsafe {
    let cookie = xcb_input_open_device(c.get_raw_conn(),
        device_id as u8); //1
    OpenDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn OpenDeviceUnchecked<'r> (c : &'r Connection,
                            device_id : u8) -> OpenDeviceCookie<'r> {
  unsafe {
    let cookie = xcb_input_open_device_unchecked(c.get_raw_conn(),
        device_id as u8); //1
    OpenDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl OpenDeviceReply {
  pub fn class_info(&mut self) -> InputClassInfoIterator {
    unsafe { accessor!(InputClassInfoIterator, xcb_input_open_device_class_info_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(OpenDeviceCookie<'s>, mk_reply_xcb_input_open_device_reply_t, OpenDeviceReply, xcb_input_open_device_reply);

pub fn CloseDeviceChecked<'r> (c : &'r Connection,
                           device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_close_device_checked(c.get_raw_conn(),
        device_id as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn CloseDevice<'r> (c : &'r Connection,
                    device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_close_device(c.get_raw_conn(),
        device_id as u8); //1
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDeviceMode<'r> (c : &'r Connection,
                      device_id : u8,
                      mode : u8) -> SetDeviceModeCookie<'r> {
  unsafe {
    let cookie = xcb_input_set_device_mode(c.get_raw_conn(),
        device_id as u8, //1
        mode as u8); //2
    SetDeviceModeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDeviceModeUnchecked<'r> (c : &'r Connection,
                               device_id : u8,
                               mode : u8) -> SetDeviceModeCookie<'r> {
  unsafe {
    let cookie = xcb_input_set_device_mode_unchecked(c.get_raw_conn(),
        device_id as u8, //1
        mode as u8); //2
    SetDeviceModeCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetDeviceModeReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetDeviceModeCookie<'s>, mk_reply_xcb_input_set_device_mode_reply_t, SetDeviceModeReply, xcb_input_set_device_mode_reply);

pub fn SelectExtensionEventChecked<'r> (c : &'r Connection,
                                    window : xproto::Window,
                                    classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_select_extension_event_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        classes_len as u16, //2
        classes_ptr as *mut xcb_input_event_class_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SelectExtensionEvent<'r> (c : &'r Connection,
                             window : xproto::Window,
                             classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_select_extension_event(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        classes_len as u16, //2
        classes_ptr as *mut xcb_input_event_class_t); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetSelectedExtensionEventsReply { base:  base::Reply<xcb_input_get_selected_extension_events_reply_t> }
fn mk_reply_xcb_input_get_selected_extension_events_reply_t(reply:*mut xcb_input_get_selected_extension_events_reply_t) -> GetSelectedExtensionEventsReply { GetSelectedExtensionEventsReply { base : base::mk_reply(reply) } }
pub fn GetSelectedExtensionEvents<'r> (c : &'r Connection,
                                   window : xproto::Window) -> GetSelectedExtensionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_selected_extension_events(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetSelectedExtensionEventsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetSelectedExtensionEventsUnchecked<'r> (c : &'r Connection,
                                            window : xproto::Window) -> GetSelectedExtensionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_selected_extension_events_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetSelectedExtensionEventsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetSelectedExtensionEventsReply {
  pub fn this_classes(&mut self) -> Vec<EventClass> {
    unsafe { accessor!(EventClass, xcb_input_get_selected_extension_events_this_classes_length, xcb_input_get_selected_extension_events_this_classes, (*self.base.reply)) }
  }

  pub fn all_classes(&mut self) -> Vec<EventClass> {
    unsafe { accessor!(EventClass, xcb_input_get_selected_extension_events_all_classes_length, xcb_input_get_selected_extension_events_all_classes, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetSelectedExtensionEventsCookie<'s>, mk_reply_xcb_input_get_selected_extension_events_reply_t, GetSelectedExtensionEventsReply, xcb_input_get_selected_extension_events_reply);

pub fn ChangeDeviceDontPropagateListChecked<'r> (c : &'r Connection,
                                             window : xproto::Window,
                                             mode : u8,
                                             classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_change_device_dont_propagate_list_checked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        classes_len as u16, //2
        mode as u8, //3
        classes_ptr as *mut xcb_input_event_class_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeDeviceDontPropagateList<'r> (c : &'r Connection,
                                      window : xproto::Window,
                                      mode : u8,
                                      classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_change_device_dont_propagate_list(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t, //1
        classes_len as u16, //2
        mode as u8, //3
        classes_ptr as *mut xcb_input_event_class_t); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetDeviceDontPropagateListReply { base:  base::Reply<xcb_input_get_device_dont_propagate_list_reply_t> }
fn mk_reply_xcb_input_get_device_dont_propagate_list_reply_t(reply:*mut xcb_input_get_device_dont_propagate_list_reply_t) -> GetDeviceDontPropagateListReply { GetDeviceDontPropagateListReply { base : base::mk_reply(reply) } }
pub fn GetDeviceDontPropagateList<'r> (c : &'r Connection,
                                   window : xproto::Window) -> GetDeviceDontPropagateListCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_dont_propagate_list(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetDeviceDontPropagateListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceDontPropagateListUnchecked<'r> (c : &'r Connection,
                                            window : xproto::Window) -> GetDeviceDontPropagateListCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_dont_propagate_list_unchecked(c.get_raw_conn(),
        window as ffi::xproto::xcb_window_t); //1
    GetDeviceDontPropagateListCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceDontPropagateListReply {
  pub fn classes(&mut self) -> Vec<EventClass> {
    unsafe { accessor!(EventClass, xcb_input_get_device_dont_propagate_list_classes_length, xcb_input_get_device_dont_propagate_list_classes, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceDontPropagateListCookie<'s>, mk_reply_xcb_input_get_device_dont_propagate_list_reply_t, GetDeviceDontPropagateListReply, xcb_input_get_device_dont_propagate_list_reply);

pub fn GetDeviceMotionEvents<'r> (c : &'r Connection,
                              start : xproto::Timestamp,
                              stop : xproto::Timestamp,
                              device_id : u8) -> GetDeviceMotionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_motion_events(c.get_raw_conn(),
        start as ffi::xproto::xcb_timestamp_t, //1
        stop as ffi::xproto::xcb_timestamp_t, //2
        device_id as u8); //3
    GetDeviceMotionEventsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceMotionEventsUnchecked<'r> (c : &'r Connection,
                                       start : xproto::Timestamp,
                                       stop : xproto::Timestamp,
                                       device_id : u8) -> GetDeviceMotionEventsCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_motion_events_unchecked(c.get_raw_conn(),
        start as ffi::xproto::xcb_timestamp_t, //1
        stop as ffi::xproto::xcb_timestamp_t, //2
        device_id as u8); //3
    GetDeviceMotionEventsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceMotionEventsReply {
  pub fn num_coords(&mut self) -> u32 {
    unsafe { accessor!(num_coords -> u32, (*self.base.reply)) }
  }

  pub fn num_axes(&mut self) -> u8 {
    unsafe { accessor!(num_axes -> u8, (*self.base.reply)) }
  }

  pub fn device_mode(&mut self) -> u8 {
    unsafe { accessor!(device_mode -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceMotionEventsCookie<'s>, mk_reply_xcb_input_get_device_motion_events_reply_t, GetDeviceMotionEventsReply, xcb_input_get_device_motion_events_reply);

pub struct DeviceTimeCoord {pub base : base::Struct<xcb_input_device_time_coord_t> }


impl DeviceTimeCoord {
  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, self.base.strct) }
  }

}

impl Iterator for DeviceTimeCoordIterator {
    type Item = DeviceTimeCoord;
    fn next(&mut self) -> Option<DeviceTimeCoord> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_time_coord_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_time_coord_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn ChangeKeyboardDevice<'r> (c : &'r Connection,
                             device_id : u8) -> ChangeKeyboardDeviceCookie<'r> {
  unsafe {
    let cookie = xcb_input_change_keyboard_device(c.get_raw_conn(),
        device_id as u8); //1
    ChangeKeyboardDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangeKeyboardDeviceUnchecked<'r> (c : &'r Connection,
                                      device_id : u8) -> ChangeKeyboardDeviceCookie<'r> {
  unsafe {
    let cookie = xcb_input_change_keyboard_device_unchecked(c.get_raw_conn(),
        device_id as u8); //1
    ChangeKeyboardDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ChangeKeyboardDeviceReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ChangeKeyboardDeviceCookie<'s>, mk_reply_xcb_input_change_keyboard_device_reply_t, ChangeKeyboardDeviceReply, xcb_input_change_keyboard_device_reply);

pub fn ChangePointerDevice<'r> (c : &'r Connection,
                            x_axis : u8,
                            y_axis : u8,
                            device_id : u8) -> ChangePointerDeviceCookie<'r> {
  unsafe {
    let cookie = xcb_input_change_pointer_device(c.get_raw_conn(),
        x_axis as u8, //1
        y_axis as u8, //2
        device_id as u8); //3
    ChangePointerDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn ChangePointerDeviceUnchecked<'r> (c : &'r Connection,
                                     x_axis : u8,
                                     y_axis : u8,
                                     device_id : u8) -> ChangePointerDeviceCookie<'r> {
  unsafe {
    let cookie = xcb_input_change_pointer_device_unchecked(c.get_raw_conn(),
        x_axis as u8, //1
        y_axis as u8, //2
        device_id as u8); //3
    ChangePointerDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl ChangePointerDeviceReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(ChangePointerDeviceCookie<'s>, mk_reply_xcb_input_change_pointer_device_reply_t, ChangePointerDeviceReply, xcb_input_change_pointer_device_reply);

pub fn GrabDevice<'r> (c : &'r Connection,
                   grab_window : xproto::Window,
                   time : xproto::Timestamp,
                   this_device_mode : u8,
                   other_device_mode : u8,
                   owner_events : u8,
                   device_id : u8,
                   classes : &[EventClass]) -> GrabDeviceCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_grab_device(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        time as ffi::xproto::xcb_timestamp_t, //2
        classes_len as u16, //3
        this_device_mode as u8, //4
        other_device_mode as u8, //5
        owner_events as u8, //6
        device_id as u8, //7
        classes_ptr as *mut xcb_input_event_class_t); //8
    GrabDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabDeviceUnchecked<'r> (c : &'r Connection,
                            grab_window : xproto::Window,
                            time : xproto::Timestamp,
                            this_device_mode : u8,
                            other_device_mode : u8,
                            owner_events : u8,
                            device_id : u8,
                            classes : &[EventClass]) -> GrabDeviceCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_grab_device_unchecked(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        time as ffi::xproto::xcb_timestamp_t, //2
        classes_len as u16, //3
        this_device_mode as u8, //4
        other_device_mode as u8, //5
        owner_events as u8, //6
        device_id as u8, //7
        classes_ptr as *mut xcb_input_event_class_t); //8
    GrabDeviceCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GrabDeviceReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GrabDeviceCookie<'s>, mk_reply_xcb_input_grab_device_reply_t, GrabDeviceReply, xcb_input_grab_device_reply);

pub fn UngrabDeviceChecked<'r> (c : &'r Connection,
                            time : xproto::Timestamp,
                            device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_ungrab_device_checked(c.get_raw_conn(),
        time as ffi::xproto::xcb_timestamp_t, //1
        device_id as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabDevice<'r> (c : &'r Connection,
                     time : xproto::Timestamp,
                     device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_ungrab_device(c.get_raw_conn(),
        time as ffi::xproto::xcb_timestamp_t, //1
        device_id as u8); //2
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabDeviceKeyChecked<'r> (c : &'r Connection,
                             grab_window : xproto::Window,
                             modifiers : u16,
                             modifier_device : u8,
                             grabbed_device : u8,
                             key : u8,
                             this_device_mode : u8,
                             other_device_mode : u8,
                             owner_events : u8,
                             classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_grab_device_key_checked(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        classes_len as u16, //2
        modifiers as u16, //3
        modifier_device as u8, //4
        grabbed_device as u8, //5
        key as u8, //6
        this_device_mode as u8, //7
        other_device_mode as u8, //8
        owner_events as u8, //9
        classes_ptr as *mut xcb_input_event_class_t); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GrabDeviceKey<'r> (c : &'r Connection,
                      grab_window : xproto::Window,
                      modifiers : u16,
                      modifier_device : u8,
                      grabbed_device : u8,
                      key : u8,
                      this_device_mode : u8,
                      other_device_mode : u8,
                      owner_events : u8,
                      classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_grab_device_key(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        classes_len as u16, //2
        modifiers as u16, //3
        modifier_device as u8, //4
        grabbed_device as u8, //5
        key as u8, //6
        this_device_mode as u8, //7
        other_device_mode as u8, //8
        owner_events as u8, //9
        classes_ptr as *mut xcb_input_event_class_t); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UngrabDeviceKeyChecked<'r> (c : &'r Connection,
                               grabWindow : xproto::Window,
                               modifiers : u16,
                               modifier_device : u8,
                               key : u8,
                               grabbed_device : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_ungrab_device_key_checked(c.get_raw_conn(),
        grabWindow as ffi::xproto::xcb_window_t, //1
        modifiers as u16, //2
        modifier_device as u8, //3
        key as u8, //4
        grabbed_device as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabDeviceKey<'r> (c : &'r Connection,
                        grabWindow : xproto::Window,
                        modifiers : u16,
                        modifier_device : u8,
                        key : u8,
                        grabbed_device : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_ungrab_device_key(c.get_raw_conn(),
        grabWindow as ffi::xproto::xcb_window_t, //1
        modifiers as u16, //2
        modifier_device as u8, //3
        key as u8, //4
        grabbed_device as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GrabDeviceButtonChecked<'r> (c : &'r Connection,
                                grab_window : xproto::Window,
                                grabbed_device : u8,
                                modifier_device : u8,
                                modifiers : u16,
                                this_device_mode : u8,
                                other_device_mode : u8,
                                button : u8,
                                owner_events : u8,
                                classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_grab_device_button_checked(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        grabbed_device as u8, //2
        modifier_device as u8, //3
        classes_len as u16, //4
        modifiers as u16, //5
        this_device_mode as u8, //6
        other_device_mode as u8, //7
        button as u8, //8
        owner_events as u8, //9
        classes_ptr as *mut xcb_input_event_class_t); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn GrabDeviceButton<'r> (c : &'r Connection,
                         grab_window : xproto::Window,
                         grabbed_device : u8,
                         modifier_device : u8,
                         modifiers : u16,
                         this_device_mode : u8,
                         other_device_mode : u8,
                         button : u8,
                         owner_events : u8,
                         classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_grab_device_button(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        grabbed_device as u8, //2
        modifier_device as u8, //3
        classes_len as u16, //4
        modifiers as u16, //5
        this_device_mode as u8, //6
        other_device_mode as u8, //7
        button as u8, //8
        owner_events as u8, //9
        classes_ptr as *mut xcb_input_event_class_t); //10
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn UngrabDeviceButtonChecked<'r> (c : &'r Connection,
                                  grab_window : xproto::Window,
                                  modifiers : u16,
                                  modifier_device : u8,
                                  button : u8,
                                  grabbed_device : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_ungrab_device_button_checked(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        modifiers as u16, //2
        modifier_device as u8, //3
        button as u8, //4
        grabbed_device as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn UngrabDeviceButton<'r> (c : &'r Connection,
                           grab_window : xproto::Window,
                           modifiers : u16,
                           modifier_device : u8,
                           button : u8,
                           grabbed_device : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_ungrab_device_button(c.get_raw_conn(),
        grab_window as ffi::xproto::xcb_window_t, //1
        modifiers as u16, //2
        modifier_device as u8, //3
        button as u8, //4
        grabbed_device as u8); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn AllowDeviceEventsChecked<'r> (c : &'r Connection,
                                 time : xproto::Timestamp,
                                 mode : u8,
                                 device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_allow_device_events_checked(c.get_raw_conn(),
        time as ffi::xproto::xcb_timestamp_t, //1
        mode as u8, //2
        device_id as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn AllowDeviceEvents<'r> (c : &'r Connection,
                          time : xproto::Timestamp,
                          mode : u8,
                          device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_allow_device_events(c.get_raw_conn(),
        time as ffi::xproto::xcb_timestamp_t, //1
        mode as u8, //2
        device_id as u8); //3
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceFocus<'r> (c : &'r Connection,
                       device_id : u8) -> GetDeviceFocusCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_focus(c.get_raw_conn(),
        device_id as u8); //1
    GetDeviceFocusCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceFocusUnchecked<'r> (c : &'r Connection,
                                device_id : u8) -> GetDeviceFocusCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_focus_unchecked(c.get_raw_conn(),
        device_id as u8); //1
    GetDeviceFocusCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceFocusReply {
  pub fn focus(&mut self) -> xproto::Window {
    unsafe { accessor!(focus -> xproto::Window, (*self.base.reply)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.reply)) }
  }

  pub fn revert_to(&mut self) -> u8 {
    unsafe { accessor!(revert_to -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceFocusCookie<'s>, mk_reply_xcb_input_get_device_focus_reply_t, GetDeviceFocusReply, xcb_input_get_device_focus_reply);

pub fn SetDeviceFocusChecked<'r> (c : &'r Connection,
                              focus : xproto::Window,
                              time : xproto::Timestamp,
                              revert_to : u8,
                              device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_set_device_focus_checked(c.get_raw_conn(),
        focus as ffi::xproto::xcb_window_t, //1
        time as ffi::xproto::xcb_timestamp_t, //2
        revert_to as u8, //3
        device_id as u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SetDeviceFocus<'r> (c : &'r Connection,
                       focus : xproto::Window,
                       time : xproto::Timestamp,
                       revert_to : u8,
                       device_id : u8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_set_device_focus(c.get_raw_conn(),
        focus as ffi::xproto::xcb_window_t, //1
        time as ffi::xproto::xcb_timestamp_t, //2
        revert_to as u8, //3
        device_id as u8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetFeedbackControl<'r> (c : &'r Connection,
                           device_id : u8) -> GetFeedbackControlCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_feedback_control(c.get_raw_conn(),
        device_id as u8); //1
    GetFeedbackControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetFeedbackControlUnchecked<'r> (c : &'r Connection,
                                    device_id : u8) -> GetFeedbackControlCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_feedback_control_unchecked(c.get_raw_conn(),
        device_id as u8); //1
    GetFeedbackControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetFeedbackControlReply {
  pub fn num_feedback(&mut self) -> u16 {
    unsafe { accessor!(num_feedback -> u16, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetFeedbackControlCookie<'s>, mk_reply_xcb_input_get_feedback_control_reply_t, GetFeedbackControlReply, xcb_input_get_feedback_control_reply);


impl FeedbackState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

}

impl Iterator for FeedbackStateIterator {
    type Item = FeedbackState;
    fn next(&mut self) -> Option<FeedbackState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_feedback_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_feedback_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KbdFeedbackState {pub base : base::Struct<xcb_input_kbd_feedback_state_t> }


impl KbdFeedbackState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn pitch(&mut self) -> u16 {
    unsafe { accessor!(pitch -> u16, self.base.strct) }
  }

  pub fn duration(&mut self) -> u16 {
    unsafe { accessor!(duration -> u16, self.base.strct) }
  }

  pub fn led_mask(&mut self) -> u32 {
    unsafe { accessor!(led_mask -> u32, self.base.strct) }
  }

  pub fn led_values(&mut self) -> u32 {
    unsafe { accessor!(led_values -> u32, self.base.strct) }
  }

  pub fn global_auto_repeat(&mut self) -> u8 {
    unsafe { accessor!(global_auto_repeat -> u8, self.base.strct) }
  }

  pub fn click(&mut self) -> u8 {
    unsafe { accessor!(click -> u8, self.base.strct) }
  }

  pub fn percent(&mut self) -> u8 {
    unsafe { accessor!(percent -> u8, self.base.strct) }
  }

  pub fn auto_repeats(&self) -> Vec<u8> {
    unsafe { (self.base.strct.auto_repeats).to_vec() }
  }

}

impl Iterator for KbdFeedbackStateIterator {
    type Item = KbdFeedbackState;
    fn next(&mut self) -> Option<KbdFeedbackState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_kbd_feedback_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_kbd_feedback_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct PtrFeedbackState {pub base : base::Struct<xcb_input_ptr_feedback_state_t> }


impl PtrFeedbackState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn accel_num(&mut self) -> u16 {
    unsafe { accessor!(accel_num -> u16, self.base.strct) }
  }

  pub fn accel_denom(&mut self) -> u16 {
    unsafe { accessor!(accel_denom -> u16, self.base.strct) }
  }

  pub fn threshold(&mut self) -> u16 {
    unsafe { accessor!(threshold -> u16, self.base.strct) }
  }

}

impl Iterator for PtrFeedbackStateIterator {
    type Item = PtrFeedbackState;
    fn next(&mut self) -> Option<PtrFeedbackState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_ptr_feedback_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_ptr_feedback_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct IntegerFeedbackState {pub base : base::Struct<xcb_input_integer_feedback_state_t> }


impl IntegerFeedbackState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn resolution(&mut self) -> u32 {
    unsafe { accessor!(resolution -> u32, self.base.strct) }
  }

  pub fn min_value(&mut self) -> i32 {
    unsafe { accessor!(min_value -> i32, self.base.strct) }
  }

  pub fn max_value(&mut self) -> i32 {
    unsafe { accessor!(max_value -> i32, self.base.strct) }
  }

}

impl Iterator for IntegerFeedbackStateIterator {
    type Item = IntegerFeedbackState;
    fn next(&mut self) -> Option<IntegerFeedbackState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_integer_feedback_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_integer_feedback_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct StringFeedbackState {pub base : base::Struct<xcb_input_string_feedback_state_t> }


impl StringFeedbackState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn max_symbols(&mut self) -> u16 {
    unsafe { accessor!(max_symbols -> u16, self.base.strct) }
  }

  pub fn keysyms(&mut self) -> Vec<xproto::Keysym> {
    unsafe { accessor!(xproto::Keysym, xcb_input_string_feedback_state_keysyms_length, xcb_input_string_feedback_state_keysyms, self.base.strct) }
  }

}

impl Iterator for StringFeedbackStateIterator {
    type Item = StringFeedbackState;
    fn next(&mut self) -> Option<StringFeedbackState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_string_feedback_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_string_feedback_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct BellFeedbackState {pub base : base::Struct<xcb_input_bell_feedback_state_t> }


impl BellFeedbackState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn percent(&mut self) -> u8 {
    unsafe { accessor!(percent -> u8, self.base.strct) }
  }

  pub fn pitch(&mut self) -> u16 {
    unsafe { accessor!(pitch -> u16, self.base.strct) }
  }

  pub fn duration(&mut self) -> u16 {
    unsafe { accessor!(duration -> u16, self.base.strct) }
  }

}

impl Iterator for BellFeedbackStateIterator {
    type Item = BellFeedbackState;
    fn next(&mut self) -> Option<BellFeedbackState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_bell_feedback_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_bell_feedback_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct LedFeedbackState {pub base : base::Struct<xcb_input_led_feedback_state_t> }


impl LedFeedbackState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn led_mask(&mut self) -> u32 {
    unsafe { accessor!(led_mask -> u32, self.base.strct) }
  }

  pub fn led_values(&mut self) -> u32 {
    unsafe { accessor!(led_values -> u32, self.base.strct) }
  }

}

impl Iterator for LedFeedbackStateIterator {
    type Item = LedFeedbackState;
    fn next(&mut self) -> Option<LedFeedbackState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_led_feedback_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_led_feedback_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct FeedbackCtl {pub base : base::Struct<xcb_input_feedback_ctl_t> }


impl FeedbackCtl {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

}

impl Iterator for FeedbackCtlIterator {
    type Item = FeedbackCtl;
    fn next(&mut self) -> Option<FeedbackCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_feedback_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_feedback_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KbdFeedbackCtl {pub base : base::Struct<xcb_input_kbd_feedback_ctl_t> }


impl KbdFeedbackCtl {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn key(&mut self) -> KeyCode {
    unsafe { accessor!(key -> KeyCode, self.base.strct) }
  }

  pub fn auto_repeat_mode(&mut self) -> u8 {
    unsafe { accessor!(auto_repeat_mode -> u8, self.base.strct) }
  }

  pub fn key_click_percent(&mut self) -> i8 {
    unsafe { accessor!(key_click_percent -> i8, self.base.strct) }
  }

  pub fn bell_percent(&mut self) -> i8 {
    unsafe { accessor!(bell_percent -> i8, self.base.strct) }
  }

  pub fn bell_pitch(&mut self) -> i16 {
    unsafe { accessor!(bell_pitch -> i16, self.base.strct) }
  }

  pub fn bell_duration(&mut self) -> i16 {
    unsafe { accessor!(bell_duration -> i16, self.base.strct) }
  }

  pub fn led_mask(&mut self) -> u32 {
    unsafe { accessor!(led_mask -> u32, self.base.strct) }
  }

  pub fn led_values(&mut self) -> u32 {
    unsafe { accessor!(led_values -> u32, self.base.strct) }
  }

}

impl Iterator for KbdFeedbackCtlIterator {
    type Item = KbdFeedbackCtl;
    fn next(&mut self) -> Option<KbdFeedbackCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_kbd_feedback_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_kbd_feedback_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct PtrFeedbackCtl {pub base : base::Struct<xcb_input_ptr_feedback_ctl_t> }


impl PtrFeedbackCtl {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn num(&mut self) -> i16 {
    unsafe { accessor!(num -> i16, self.base.strct) }
  }

  pub fn denom(&mut self) -> i16 {
    unsafe { accessor!(denom -> i16, self.base.strct) }
  }

  pub fn threshold(&mut self) -> i16 {
    unsafe { accessor!(threshold -> i16, self.base.strct) }
  }

}

impl Iterator for PtrFeedbackCtlIterator {
    type Item = PtrFeedbackCtl;
    fn next(&mut self) -> Option<PtrFeedbackCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_ptr_feedback_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_ptr_feedback_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct IntegerFeedbackCtl {pub base : base::Struct<xcb_input_integer_feedback_ctl_t> }


impl IntegerFeedbackCtl {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn int_to_display(&mut self) -> i32 {
    unsafe { accessor!(int_to_display -> i32, self.base.strct) }
  }

}

impl Iterator for IntegerFeedbackCtlIterator {
    type Item = IntegerFeedbackCtl;
    fn next(&mut self) -> Option<IntegerFeedbackCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_integer_feedback_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_integer_feedback_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct StringFeedbackCtl {pub base : base::Struct<xcb_input_string_feedback_ctl_t> }


impl StringFeedbackCtl {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn keysyms(&mut self) -> Vec<xproto::Keysym> {
    unsafe { accessor!(xproto::Keysym, xcb_input_string_feedback_ctl_keysyms_length, xcb_input_string_feedback_ctl_keysyms, self.base.strct) }
  }

}

impl Iterator for StringFeedbackCtlIterator {
    type Item = StringFeedbackCtl;
    fn next(&mut self) -> Option<StringFeedbackCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_string_feedback_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_string_feedback_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct BellFeedbackCtl {pub base : base::Struct<xcb_input_bell_feedback_ctl_t> }


impl BellFeedbackCtl {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn percent(&mut self) -> i8 {
    unsafe { accessor!(percent -> i8, self.base.strct) }
  }

  pub fn pitch(&mut self) -> i16 {
    unsafe { accessor!(pitch -> i16, self.base.strct) }
  }

  pub fn duration(&mut self) -> i16 {
    unsafe { accessor!(duration -> i16, self.base.strct) }
  }

}

impl Iterator for BellFeedbackCtlIterator {
    type Item = BellFeedbackCtl;
    fn next(&mut self) -> Option<BellFeedbackCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_bell_feedback_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_bell_feedback_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct LedFeedbackCtl {pub base : base::Struct<xcb_input_led_feedback_ctl_t> }


impl LedFeedbackCtl {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn id(&mut self) -> u8 {
    unsafe { accessor!(id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn led_mask(&mut self) -> u32 {
    unsafe { accessor!(led_mask -> u32, self.base.strct) }
  }

  pub fn led_values(&mut self) -> u32 {
    unsafe { accessor!(led_values -> u32, self.base.strct) }
  }

}

impl Iterator for LedFeedbackCtlIterator {
    type Item = LedFeedbackCtl;
    fn next(&mut self) -> Option<LedFeedbackCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_led_feedback_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_led_feedback_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct GetDeviceKeyMappingReply { base:  base::Reply<xcb_input_get_device_key_mapping_reply_t> }
fn mk_reply_xcb_input_get_device_key_mapping_reply_t(reply:*mut xcb_input_get_device_key_mapping_reply_t) -> GetDeviceKeyMappingReply { GetDeviceKeyMappingReply { base : base::mk_reply(reply) } }
pub fn GetDeviceKeyMapping<'r> (c : &'r Connection,
                            device_id : u8,
                            first_keycode : KeyCode,
                            count : u8) -> GetDeviceKeyMappingCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_key_mapping(c.get_raw_conn(),
        device_id as u8, //1
        first_keycode as xcb_input_key_code_t, //2
        count as u8); //3
    GetDeviceKeyMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceKeyMappingUnchecked<'r> (c : &'r Connection,
                                     device_id : u8,
                                     first_keycode : KeyCode,
                                     count : u8) -> GetDeviceKeyMappingCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_key_mapping_unchecked(c.get_raw_conn(),
        device_id as u8, //1
        first_keycode as xcb_input_key_code_t, //2
        count as u8); //3
    GetDeviceKeyMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceKeyMappingReply {
  pub fn keysyms_per_keycode(&mut self) -> u8 {
    unsafe { accessor!(keysyms_per_keycode -> u8, (*self.base.reply)) }
  }

  pub fn keysyms(&mut self) -> Vec<xproto::Keysym> {
    unsafe { accessor!(xproto::Keysym, xcb_input_get_device_key_mapping_keysyms_length, xcb_input_get_device_key_mapping_keysyms, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceKeyMappingCookie<'s>, mk_reply_xcb_input_get_device_key_mapping_reply_t, GetDeviceKeyMappingReply, xcb_input_get_device_key_mapping_reply);

pub fn ChangeDeviceKeyMappingChecked<'r> (c : &'r Connection,
                                      device_id : u8,
                                      first_keycode : KeyCode,
                                      keysyms_per_keycode : u8,
                                      keysyms : &[xproto::Keysym]) -> base::VoidCookie<'r> {
  unsafe {
    let keysyms_len = keysyms.len();
    let keysyms_ptr = keysyms.as_ptr();
    let cookie = xcb_input_change_device_key_mapping_checked(c.get_raw_conn(),
        device_id as u8, //1
        first_keycode as xcb_input_key_code_t, //2
        keysyms_per_keycode as u8, //3
        keysyms_len as u8, //4
        keysyms_ptr as *mut ffi::xproto::xcb_keysym_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn ChangeDeviceKeyMapping<'r> (c : &'r Connection,
                               device_id : u8,
                               first_keycode : KeyCode,
                               keysyms_per_keycode : u8,
                               keysyms : &[xproto::Keysym]) -> base::VoidCookie<'r> {
  unsafe {
    let keysyms_len = keysyms.len();
    let keysyms_ptr = keysyms.as_ptr();
    let cookie = xcb_input_change_device_key_mapping(c.get_raw_conn(),
        device_id as u8, //1
        first_keycode as xcb_input_key_code_t, //2
        keysyms_per_keycode as u8, //3
        keysyms_len as u8, //4
        keysyms_ptr as *mut ffi::xproto::xcb_keysym_t); //5
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub struct GetDeviceModifierMappingReply { base:  base::Reply<xcb_input_get_device_modifier_mapping_reply_t> }
fn mk_reply_xcb_input_get_device_modifier_mapping_reply_t(reply:*mut xcb_input_get_device_modifier_mapping_reply_t) -> GetDeviceModifierMappingReply { GetDeviceModifierMappingReply { base : base::mk_reply(reply) } }
pub fn GetDeviceModifierMapping<'r> (c : &'r Connection,
                                 device_id : u8) -> GetDeviceModifierMappingCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_modifier_mapping(c.get_raw_conn(),
        device_id as u8); //1
    GetDeviceModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceModifierMappingUnchecked<'r> (c : &'r Connection,
                                          device_id : u8) -> GetDeviceModifierMappingCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_modifier_mapping_unchecked(c.get_raw_conn(),
        device_id as u8); //1
    GetDeviceModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceModifierMappingReply {
  pub fn keymaps(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_input_get_device_modifier_mapping_keymaps_length, xcb_input_get_device_modifier_mapping_keymaps, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceModifierMappingCookie<'s>, mk_reply_xcb_input_get_device_modifier_mapping_reply_t, GetDeviceModifierMappingReply, xcb_input_get_device_modifier_mapping_reply);

pub fn SetDeviceModifierMapping<'r> (c : &'r Connection,
                                 device_id : u8,
                                 keymaps : &[u8]) -> SetDeviceModifierMappingCookie<'r> {
  unsafe {
    let keymaps_len = keymaps.len();
    let keymaps_ptr = keymaps.as_ptr();
    let cookie = xcb_input_set_device_modifier_mapping(c.get_raw_conn(),
        device_id as u8, //1
        keymaps_len as u8, //2
        keymaps_ptr as *mut u8); //3
    SetDeviceModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDeviceModifierMappingUnchecked<'r> (c : &'r Connection,
                                          device_id : u8,
                                          keymaps : &[u8]) -> SetDeviceModifierMappingCookie<'r> {
  unsafe {
    let keymaps_len = keymaps.len();
    let keymaps_ptr = keymaps.as_ptr();
    let cookie = xcb_input_set_device_modifier_mapping_unchecked(c.get_raw_conn(),
        device_id as u8, //1
        keymaps_len as u8, //2
        keymaps_ptr as *mut u8); //3
    SetDeviceModifierMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetDeviceModifierMappingReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetDeviceModifierMappingCookie<'s>, mk_reply_xcb_input_set_device_modifier_mapping_reply_t, SetDeviceModifierMappingReply, xcb_input_set_device_modifier_mapping_reply);

pub struct GetDeviceButtonMappingReply { base:  base::Reply<xcb_input_get_device_button_mapping_reply_t> }
fn mk_reply_xcb_input_get_device_button_mapping_reply_t(reply:*mut xcb_input_get_device_button_mapping_reply_t) -> GetDeviceButtonMappingReply { GetDeviceButtonMappingReply { base : base::mk_reply(reply) } }
pub fn GetDeviceButtonMapping<'r> (c : &'r Connection,
                               device_id : u8) -> GetDeviceButtonMappingCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_button_mapping(c.get_raw_conn(),
        device_id as u8); //1
    GetDeviceButtonMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceButtonMappingUnchecked<'r> (c : &'r Connection,
                                        device_id : u8) -> GetDeviceButtonMappingCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_button_mapping_unchecked(c.get_raw_conn(),
        device_id as u8); //1
    GetDeviceButtonMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceButtonMappingReply {
  pub fn map(&mut self) -> Vec<u8> {
    unsafe { accessor!(u8, xcb_input_get_device_button_mapping_map_length, xcb_input_get_device_button_mapping_map, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceButtonMappingCookie<'s>, mk_reply_xcb_input_get_device_button_mapping_reply_t, GetDeviceButtonMappingReply, xcb_input_get_device_button_mapping_reply);

pub fn SetDeviceButtonMapping<'r> (c : &'r Connection,
                               device_id : u8,
                               map : &[u8]) -> SetDeviceButtonMappingCookie<'r> {
  unsafe {
    let map_len = map.len();
    let map_ptr = map.as_ptr();
    let cookie = xcb_input_set_device_button_mapping(c.get_raw_conn(),
        device_id as u8, //1
        map_len as u8, //2
        map_ptr as *mut u8); //3
    SetDeviceButtonMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDeviceButtonMappingUnchecked<'r> (c : &'r Connection,
                                        device_id : u8,
                                        map : &[u8]) -> SetDeviceButtonMappingCookie<'r> {
  unsafe {
    let map_len = map.len();
    let map_ptr = map.as_ptr();
    let cookie = xcb_input_set_device_button_mapping_unchecked(c.get_raw_conn(),
        device_id as u8, //1
        map_len as u8, //2
        map_ptr as *mut u8); //3
    SetDeviceButtonMappingCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetDeviceButtonMappingReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetDeviceButtonMappingCookie<'s>, mk_reply_xcb_input_set_device_button_mapping_reply_t, SetDeviceButtonMappingReply, xcb_input_set_device_button_mapping_reply);

pub fn QueryDeviceState<'r> (c : &'r Connection,
                         device_id : u8) -> QueryDeviceStateCookie<'r> {
  unsafe {
    let cookie = xcb_input_query_device_state(c.get_raw_conn(),
        device_id as u8); //1
    QueryDeviceStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn QueryDeviceStateUnchecked<'r> (c : &'r Connection,
                                  device_id : u8) -> QueryDeviceStateCookie<'r> {
  unsafe {
    let cookie = xcb_input_query_device_state_unchecked(c.get_raw_conn(),
        device_id as u8); //1
    QueryDeviceStateCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl QueryDeviceStateReply {
  pub fn num_classes(&mut self) -> u8 {
    unsafe { accessor!(num_classes -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryDeviceStateCookie<'s>, mk_reply_xcb_input_query_device_state_reply_t, QueryDeviceStateReply, xcb_input_query_device_state_reply);

pub struct InputState {pub base : base::Struct<xcb_input_input_state_t> }


impl InputState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

  pub fn num_items(&mut self) -> u8 {
    unsafe { accessor!(num_items -> u8, self.base.strct) }
  }

}

impl Iterator for InputStateIterator {
    type Item = InputState;
    fn next(&mut self) -> Option<InputState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_input_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_input_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct KeyState {pub base : base::Struct<xcb_input_key_state_t> }


impl KeyState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

  pub fn num_keys(&mut self) -> u8 {
    unsafe { accessor!(num_keys -> u8, self.base.strct) }
  }

  pub fn keys(&self) -> Vec<u8> {
    unsafe { (self.base.strct.keys).to_vec() }
  }

}

impl Iterator for KeyStateIterator {
    type Item = KeyState;
    fn next(&mut self) -> Option<KeyState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_key_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_key_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ButtonState {pub base : base::Struct<xcb_input_button_state_t> }


impl ButtonState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

  pub fn num_buttons(&mut self) -> u8 {
    unsafe { accessor!(num_buttons -> u8, self.base.strct) }
  }

  pub fn buttons(&self) -> Vec<u8> {
    unsafe { (self.base.strct.buttons).to_vec() }
  }

}

impl Iterator for ButtonStateIterator {
    type Item = ButtonState;
    fn next(&mut self) -> Option<ButtonState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_button_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_button_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct ValuatorState {pub base : base::Struct<xcb_input_valuator_state_t> }


impl ValuatorState {
  pub fn class_id(&mut self) -> u8 {
    unsafe { accessor!(class_id -> u8, self.base.strct) }
  }

  pub fn len(&mut self) -> u8 {
    unsafe { accessor!(len -> u8, self.base.strct) }
  }

  pub fn mode(&mut self) -> u8 {
    unsafe { accessor!(mode -> u8, self.base.strct) }
  }

  pub fn valuators(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_input_valuator_state_valuators_length, xcb_input_valuator_state_valuators, self.base.strct) }
  }

}

impl Iterator for ValuatorStateIterator {
    type Item = ValuatorState;
    fn next(&mut self) -> Option<ValuatorState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_valuator_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_valuator_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub fn SendExtensionEventChecked<'r> (c : &'r Connection,
                                  destination : xproto::Window,
                                  device_id : u8,
                                  propagate : u8,
                                  events : &str,
                                  classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let events = (events).as_bytes();
    let events_len = events.len();
    let events_ptr = events.as_ptr();
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_send_extension_event_checked(c.get_raw_conn(),
        destination as ffi::xproto::xcb_window_t, //1
        device_id as u8, //2
        propagate as u8, //3
        classes_len as u16, //4
        events_len as u8, //5
        events_ptr as *mut c_char, //6
        classes_ptr as *mut xcb_input_event_class_t); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn SendExtensionEvent<'r> (c : &'r Connection,
                           destination : xproto::Window,
                           device_id : u8,
                           propagate : u8,
                           events : &str,
                           classes : &[EventClass]) -> base::VoidCookie<'r> {
  unsafe {
    let events = (events).as_bytes();
    let events_len = events.len();
    let events_ptr = events.as_ptr();
    let classes_len = classes.len();
    let classes_ptr = classes.as_ptr();
    let cookie = xcb_input_send_extension_event(c.get_raw_conn(),
        destination as ffi::xproto::xcb_window_t, //1
        device_id as u8, //2
        propagate as u8, //3
        classes_len as u16, //4
        events_len as u8, //5
        events_ptr as *mut c_char, //6
        classes_ptr as *mut xcb_input_event_class_t); //7
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn DeviceBellChecked<'r> (c : &'r Connection,
                          device_id : u8,
                          feedback_id : u8,
                          feedback_class : u8,
                          percent : i8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_device_bell_checked(c.get_raw_conn(),
        device_id as u8, //1
        feedback_id as u8, //2
        feedback_class as u8, //3
        percent as i8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:true}}
  }
}
pub fn DeviceBell<'r> (c : &'r Connection,
                   device_id : u8,
                   feedback_id : u8,
                   feedback_class : u8,
                   percent : i8) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_input_device_bell(c.get_raw_conn(),
        device_id as u8, //1
        feedback_id as u8, //2
        feedback_class as u8, //3
        percent as i8); //4
    base::VoidCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDeviceValuators<'r> (c : &'r Connection,
                           device_id : u8,
                           first_valuator : u8,
                           valuators : &[i32]) -> SetDeviceValuatorsCookie<'r> {
  unsafe {
    let valuators_len = valuators.len();
    let valuators_ptr = valuators.as_ptr();
    let cookie = xcb_input_set_device_valuators(c.get_raw_conn(),
        device_id as u8, //1
        first_valuator as u8, //2
        valuators_len as u8, //3
        valuators_ptr as *mut i32); //4
    SetDeviceValuatorsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn SetDeviceValuatorsUnchecked<'r> (c : &'r Connection,
                                    device_id : u8,
                                    first_valuator : u8,
                                    valuators : &[i32]) -> SetDeviceValuatorsCookie<'r> {
  unsafe {
    let valuators_len = valuators.len();
    let valuators_ptr = valuators.as_ptr();
    let cookie = xcb_input_set_device_valuators_unchecked(c.get_raw_conn(),
        device_id as u8, //1
        first_valuator as u8, //2
        valuators_len as u8, //3
        valuators_ptr as *mut i32); //4
    SetDeviceValuatorsCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl SetDeviceValuatorsReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(SetDeviceValuatorsCookie<'s>, mk_reply_xcb_input_set_device_valuators_reply_t, SetDeviceValuatorsReply, xcb_input_set_device_valuators_reply);

pub fn GetDeviceControl<'r> (c : &'r Connection,
                         control_id : u16,
                         device_id : u8) -> GetDeviceControlCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_control(c.get_raw_conn(),
        control_id as u16, //1
        device_id as u8); //2
    GetDeviceControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}
pub fn GetDeviceControlUnchecked<'r> (c : &'r Connection,
                                  control_id : u16,
                                  device_id : u8) -> GetDeviceControlCookie<'r> {
  unsafe {
    let cookie = xcb_input_get_device_control_unchecked(c.get_raw_conn(),
        control_id as u16, //1
        device_id as u8); //2
    GetDeviceControlCookie { base : Cookie {cookie:cookie,conn:c,checked:false}}
  }
}

impl GetDeviceControlReply {
  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, (*self.base.reply)) }
  }

}
impl_reply_cookie!(GetDeviceControlCookie<'s>, mk_reply_xcb_input_get_device_control_reply_t, GetDeviceControlReply, xcb_input_get_device_control_reply);

pub struct DeviceState {pub base : base::Struct<xcb_input_device_state_t> }


impl DeviceState {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

}

impl Iterator for DeviceStateIterator {
    type Item = DeviceState;
    fn next(&mut self) -> Option<DeviceState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceResolutionState {pub base : base::Struct<xcb_input_device_resolution_state_t> }


impl DeviceResolutionState {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn resolution_values(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_input_device_resolution_state_resolution_values_length, xcb_input_device_resolution_state_resolution_values, self.base.strct) }
  }

  pub fn resolution_min(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_input_device_resolution_state_resolution_min_length, xcb_input_device_resolution_state_resolution_min, self.base.strct) }
  }

  pub fn resolution_max(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_input_device_resolution_state_resolution_max_length, xcb_input_device_resolution_state_resolution_max, self.base.strct) }
  }

}

impl Iterator for DeviceResolutionStateIterator {
    type Item = DeviceResolutionState;
    fn next(&mut self) -> Option<DeviceResolutionState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_resolution_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_resolution_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceAbsCalibState {pub base : base::Struct<xcb_input_device_abs_calib_state_t> }


impl DeviceAbsCalibState {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn min_x(&mut self) -> i32 {
    unsafe { accessor!(min_x -> i32, self.base.strct) }
  }

  pub fn max_x(&mut self) -> i32 {
    unsafe { accessor!(max_x -> i32, self.base.strct) }
  }

  pub fn min_y(&mut self) -> i32 {
    unsafe { accessor!(min_y -> i32, self.base.strct) }
  }

  pub fn max_y(&mut self) -> i32 {
    unsafe { accessor!(max_y -> i32, self.base.strct) }
  }

  pub fn flip_x(&mut self) -> u32 {
    unsafe { accessor!(flip_x -> u32, self.base.strct) }
  }

  pub fn flip_y(&mut self) -> u32 {
    unsafe { accessor!(flip_y -> u32, self.base.strct) }
  }

  pub fn rotation(&mut self) -> u32 {
    unsafe { accessor!(rotation -> u32, self.base.strct) }
  }

  pub fn button_threshold(&mut self) -> u32 {
    unsafe { accessor!(button_threshold -> u32, self.base.strct) }
  }

}

impl Iterator for DeviceAbsCalibStateIterator {
    type Item = DeviceAbsCalibState;
    fn next(&mut self) -> Option<DeviceAbsCalibState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_abs_calib_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_abs_calib_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceAbsAreaState {pub base : base::Struct<xcb_input_device_abs_area_state_t> }


impl DeviceAbsAreaState {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn offset_x(&mut self) -> u32 {
    unsafe { accessor!(offset_x -> u32, self.base.strct) }
  }

  pub fn offset_y(&mut self) -> u32 {
    unsafe { accessor!(offset_y -> u32, self.base.strct) }
  }

  pub fn width(&mut self) -> u32 {
    unsafe { accessor!(width -> u32, self.base.strct) }
  }

  pub fn height(&mut self) -> u32 {
    unsafe { accessor!(height -> u32, self.base.strct) }
  }

  pub fn screen(&mut self) -> u32 {
    unsafe { accessor!(screen -> u32, self.base.strct) }
  }

  pub fn following(&mut self) -> u32 {
    unsafe { accessor!(following -> u32, self.base.strct) }
  }

}

impl Iterator for DeviceAbsAreaStateIterator {
    type Item = DeviceAbsAreaState;
    fn next(&mut self) -> Option<DeviceAbsAreaState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_abs_area_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_abs_area_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceCoreState {pub base : base::Struct<xcb_input_device_core_state_t> }


impl DeviceCoreState {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, self.base.strct) }
  }

  pub fn iscore(&mut self) -> u8 {
    unsafe { accessor!(iscore -> u8, self.base.strct) }
  }

}

impl Iterator for DeviceCoreStateIterator {
    type Item = DeviceCoreState;
    fn next(&mut self) -> Option<DeviceCoreState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_core_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_core_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceEnableState {pub base : base::Struct<xcb_input_device_enable_state_t> }


impl DeviceEnableState {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn enable(&mut self) -> u8 {
    unsafe { accessor!(enable -> u8, self.base.strct) }
  }

}

impl Iterator for DeviceEnableStateIterator {
    type Item = DeviceEnableState;
    fn next(&mut self) -> Option<DeviceEnableState> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_enable_state_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_enable_state_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceCtl {pub base : base::Struct<xcb_input_device_ctl_t> }


impl DeviceCtl {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

}

impl Iterator for DeviceCtlIterator {
    type Item = DeviceCtl;
    fn next(&mut self) -> Option<DeviceCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceResolutionCtl {pub base : base::Struct<xcb_input_device_resolution_ctl_t> }


impl DeviceResolutionCtl {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn first_valuator(&mut self) -> u8 {
    unsafe { accessor!(first_valuator -> u8, self.base.strct) }
  }

  pub fn resolution_values(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_input_device_resolution_ctl_resolution_values_length, xcb_input_device_resolution_ctl_resolution_values, self.base.strct) }
  }

}

impl Iterator for DeviceResolutionCtlIterator {
    type Item = DeviceResolutionCtl;
    fn next(&mut self) -> Option<DeviceResolutionCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_resolution_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_resolution_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceAbsCalibCtl {pub base : base::Struct<xcb_input_device_abs_calib_ctl_t> }


impl DeviceAbsCalibCtl {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn min_x(&mut self) -> i32 {
    unsafe { accessor!(min_x -> i32, self.base.strct) }
  }

  pub fn max_x(&mut self) -> i32 {
    unsafe { accessor!(max_x -> i32, self.base.strct) }
  }

  pub fn min_y(&mut self) -> i32 {
    unsafe { accessor!(min_y -> i32, self.base.strct) }
  }

  pub fn max_y(&mut self) -> i32 {
    unsafe { accessor!(max_y -> i32, self.base.strct) }
  }

  pub fn flip_x(&mut self) -> u32 {
    unsafe { accessor!(flip_x -> u32, self.base.strct) }
  }

  pub fn flip_y(&mut self) -> u32 {
    unsafe { accessor!(flip_y -> u32, self.base.strct) }
  }

  pub fn rotation(&mut self) -> u32 {
    unsafe { accessor!(rotation -> u32, self.base.strct) }
  }

  pub fn button_threshold(&mut self) -> u32 {
    unsafe { accessor!(button_threshold -> u32, self.base.strct) }
  }

}

impl Iterator for DeviceAbsCalibCtlIterator {
    type Item = DeviceAbsCalibCtl;
    fn next(&mut self) -> Option<DeviceAbsCalibCtl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_abs_calib_ctl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_abs_calib_ctl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceAbsAreaCtrl {pub base : base::Struct<xcb_input_device_abs_area_ctrl_t> }


impl DeviceAbsAreaCtrl {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn offset_x(&mut self) -> u32 {
    unsafe { accessor!(offset_x -> u32, self.base.strct) }
  }

  pub fn offset_y(&mut self) -> u32 {
    unsafe { accessor!(offset_y -> u32, self.base.strct) }
  }

  pub fn width(&mut self) -> i32 {
    unsafe { accessor!(width -> i32, self.base.strct) }
  }

  pub fn height(&mut self) -> i32 {
    unsafe { accessor!(height -> i32, self.base.strct) }
  }

  pub fn screen(&mut self) -> i32 {
    unsafe { accessor!(screen -> i32, self.base.strct) }
  }

  pub fn following(&mut self) -> u32 {
    unsafe { accessor!(following -> u32, self.base.strct) }
  }

}

impl Iterator for DeviceAbsAreaCtrlIterator {
    type Item = DeviceAbsAreaCtrl;
    fn next(&mut self) -> Option<DeviceAbsAreaCtrl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_abs_area_ctrl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_abs_area_ctrl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceCoreCtrl {pub base : base::Struct<xcb_input_device_core_ctrl_t> }


impl DeviceCoreCtrl {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn status(&mut self) -> u8 {
    unsafe { accessor!(status -> u8, self.base.strct) }
  }

}

impl Iterator for DeviceCoreCtrlIterator {
    type Item = DeviceCoreCtrl;
    fn next(&mut self) -> Option<DeviceCoreCtrl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_core_ctrl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_core_ctrl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}

pub struct DeviceEnableCtrl {pub base : base::Struct<xcb_input_device_enable_ctrl_t> }


impl DeviceEnableCtrl {
  pub fn control_id(&mut self) -> u16 {
    unsafe { accessor!(control_id -> u16, self.base.strct) }
  }

  pub fn len(&mut self) -> u16 {
    unsafe { accessor!(len -> u16, self.base.strct) }
  }

  pub fn enable(&mut self) -> u8 {
    unsafe { accessor!(enable -> u8, self.base.strct) }
  }

}

impl Iterator for DeviceEnableCtrlIterator {
    type Item = DeviceEnableCtrl;
    fn next(&mut self) -> Option<DeviceEnableCtrl> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter: *mut xcb_input_device_enable_ctrl_iterator_t = mem::transmute(self);
            let data = (*iter).data;
            xcb_input_device_enable_ctrl_next(iter);
            Some(mem::transmute(*data))
        }
    }
}


impl DeviceValuatorEvent {
  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn device_state(&mut self) -> u16 {
    unsafe { accessor!(device_state -> u16, (*self.base.event)) }
  }

  pub fn num_valuators(&mut self) -> u8 {
    unsafe { accessor!(num_valuators -> u8, (*self.base.event)) }
  }

  pub fn first_valuator(&mut self) -> u8 {
    unsafe { accessor!(first_valuator -> u8, (*self.base.event)) }
  }

  pub fn valuators(&self) -> Vec<i32> {
    unsafe { ((*self.base.event).valuators).to_vec() }
  }

  pub fn new(device_id : u8,
         device_state : u16,
         num_valuators : u8,
         first_valuator : u8,
         valuators : [i32; 6]) -> DeviceValuatorEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_device_valuator_event_t;
      (*raw).device_id = device_id;
      (*raw).device_state = device_state;
      (*raw).num_valuators = num_valuators;
      (*raw).first_valuator = first_valuator;
      (*raw).valuators = valuators;
      DeviceValuatorEvent { base : Event { event : raw as *mut xcb_input_device_valuator_event_t }}
    }
  }
}

impl DeviceKeyPressEvent {
  pub fn detail(&mut self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn root(&mut self) -> xproto::Window {
    unsafe { accessor!(root -> xproto::Window, (*self.base.event)) }
  }

  pub fn event(&mut self) -> xproto::Window {
    unsafe { accessor!(event -> xproto::Window, (*self.base.event)) }
  }

  pub fn child(&mut self) -> xproto::Window {
    unsafe { accessor!(child -> xproto::Window, (*self.base.event)) }
  }

  pub fn root_x(&mut self) -> i16 {
    unsafe { accessor!(root_x -> i16, (*self.base.event)) }
  }

  pub fn root_y(&mut self) -> i16 {
    unsafe { accessor!(root_y -> i16, (*self.base.event)) }
  }

  pub fn event_x(&mut self) -> i16 {
    unsafe { accessor!(event_x -> i16, (*self.base.event)) }
  }

  pub fn event_y(&mut self) -> i16 {
    unsafe { accessor!(event_y -> i16, (*self.base.event)) }
  }

  pub fn state(&mut self) -> u16 {
    unsafe { accessor!(state -> u16, (*self.base.event)) }
  }

  pub fn same_screen(&mut self) -> u8 {
    unsafe { accessor!(same_screen -> u8, (*self.base.event)) }
  }

  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn new(detail : u8,
         time : xproto::Timestamp,
         root : xproto::Window,
         event : xproto::Window,
         child : xproto::Window,
         root_x : i16,
         root_y : i16,
         event_x : i16,
         event_y : i16,
         state : u16,
         same_screen : u8,
         device_id : u8) -> DeviceKeyPressEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_device_key_press_event_t;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).root = root;
      (*raw).event = event;
      (*raw).child = child;
      (*raw).root_x = root_x;
      (*raw).root_y = root_y;
      (*raw).event_x = event_x;
      (*raw).event_y = event_y;
      (*raw).state = state;
      (*raw).same_screen = same_screen;
      (*raw).device_id = device_id;
      DeviceKeyPressEvent { base : Event { event : raw as *mut xcb_input_device_key_press_event_t }}
    }
  }
}

impl FocusInEvent {
  pub fn detail(&mut self) -> u8 {
    unsafe { accessor!(detail -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn window(&mut self) -> xproto::Window {
    unsafe { accessor!(window -> xproto::Window, (*self.base.event)) }
  }

  pub fn mode(&mut self) -> u8 {
    unsafe { accessor!(mode -> u8, (*self.base.event)) }
  }

  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn new(detail : u8,
         time : xproto::Timestamp,
         window : xproto::Window,
         mode : u8,
         device_id : u8) -> FocusInEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_focus_in_event_t;
      (*raw).detail = detail;
      (*raw).time = time;
      (*raw).window = window;
      (*raw).mode = mode;
      (*raw).device_id = device_id;
      FocusInEvent { base : Event { event : raw as *mut xcb_input_focus_in_event_t }}
    }
  }
}

impl DeviceStateNotifyEvent {
  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn num_keys(&mut self) -> u8 {
    unsafe { accessor!(num_keys -> u8, (*self.base.event)) }
  }

  pub fn num_buttons(&mut self) -> u8 {
    unsafe { accessor!(num_buttons -> u8, (*self.base.event)) }
  }

  pub fn num_valuators(&mut self) -> u8 {
    unsafe { accessor!(num_valuators -> u8, (*self.base.event)) }
  }

  pub fn classes_reported(&mut self) -> u8 {
    unsafe { accessor!(classes_reported -> u8, (*self.base.event)) }
  }

  pub fn buttons(&self) -> Vec<u8> {
    unsafe { ((*self.base.event).buttons).to_vec() }
  }

  pub fn keys(&self) -> Vec<u8> {
    unsafe { ((*self.base.event).keys).to_vec() }
  }

  pub fn valuators(&self) -> Vec<u32> {
    unsafe { ((*self.base.event).valuators).to_vec() }
  }

  pub fn new(device_id : u8,
         time : xproto::Timestamp,
         num_keys : u8,
         num_buttons : u8,
         num_valuators : u8,
         classes_reported : u8,
         buttons : [u8; 4],
         keys : [u8; 4],
         valuators : [u32; 3]) -> DeviceStateNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_device_state_notify_event_t;
      (*raw).device_id = device_id;
      (*raw).time = time;
      (*raw).num_keys = num_keys;
      (*raw).num_buttons = num_buttons;
      (*raw).num_valuators = num_valuators;
      (*raw).classes_reported = classes_reported;
      (*raw).buttons = buttons;
      (*raw).keys = keys;
      (*raw).valuators = valuators;
      DeviceStateNotifyEvent { base : Event { event : raw as *mut xcb_input_device_state_notify_event_t }}
    }
  }
}

impl DeviceMappingNotifyEvent {
  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn request(&mut self) -> u8 {
    unsafe { accessor!(request -> u8, (*self.base.event)) }
  }

  pub fn first_keycode(&mut self) -> KeyCode {
    unsafe { accessor!(first_keycode -> KeyCode, (*self.base.event)) }
  }

  pub fn count(&mut self) -> u8 {
    unsafe { accessor!(count -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn new(device_id : u8,
         request : u8,
         first_keycode : KeyCode,
         count : u8,
         time : xproto::Timestamp) -> DeviceMappingNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_device_mapping_notify_event_t;
      (*raw).device_id = device_id;
      (*raw).request = request;
      (*raw).first_keycode = first_keycode;
      (*raw).count = count;
      (*raw).time = time;
      DeviceMappingNotifyEvent { base : Event { event : raw as *mut xcb_input_device_mapping_notify_event_t }}
    }
  }
}

impl ChangeDeviceNotifyEvent {
  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn request(&mut self) -> u8 {
    unsafe { accessor!(request -> u8, (*self.base.event)) }
  }

  pub fn new(device_id : u8,
         time : xproto::Timestamp,
         request : u8) -> ChangeDeviceNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_change_device_notify_event_t;
      (*raw).device_id = device_id;
      (*raw).time = time;
      (*raw).request = request;
      ChangeDeviceNotifyEvent { base : Event { event : raw as *mut xcb_input_change_device_notify_event_t }}
    }
  }
}

impl DeviceKeyStateNotifyEvent {
  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn keys(&self) -> Vec<u8> {
    unsafe { ((*self.base.event).keys).to_vec() }
  }

  pub fn new(device_id : u8,
         keys : [u8; 28]) -> DeviceKeyStateNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_device_key_state_notify_event_t;
      (*raw).device_id = device_id;
      (*raw).keys = keys;
      DeviceKeyStateNotifyEvent { base : Event { event : raw as *mut xcb_input_device_key_state_notify_event_t }}
    }
  }
}

impl DeviceButtonStateNotifyEvent {
  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn buttons(&self) -> Vec<u8> {
    unsafe { ((*self.base.event).buttons).to_vec() }
  }

  pub fn new(device_id : u8,
         buttons : [u8; 28]) -> DeviceButtonStateNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_device_button_state_notify_event_t;
      (*raw).device_id = device_id;
      (*raw).buttons = buttons;
      DeviceButtonStateNotifyEvent { base : Event { event : raw as *mut xcb_input_device_button_state_notify_event_t }}
    }
  }
}

impl DevicePresenceNotifyEvent {
  pub fn time(&mut self) -> xproto::Timestamp {
    unsafe { accessor!(time -> xproto::Timestamp, (*self.base.event)) }
  }

  pub fn devchange(&mut self) -> u8 {
    unsafe { accessor!(devchange -> u8, (*self.base.event)) }
  }

  pub fn device_id(&mut self) -> u8 {
    unsafe { accessor!(device_id -> u8, (*self.base.event)) }
  }

  pub fn control(&mut self) -> u16 {
    unsafe { accessor!(control -> u16, (*self.base.event)) }
  }

  pub fn new(time : xproto::Timestamp,
         devchange : u8,
         device_id : u8,
         control : u16) -> DevicePresenceNotifyEvent {
    unsafe {
      let raw = malloc(32 as size_t) as *mut xcb_input_device_presence_notify_event_t;
      (*raw).time = time;
      (*raw).devchange = devchange;
      (*raw).device_id = device_id;
      (*raw).control = control;
      DevicePresenceNotifyEvent { base : Event { event : raw as *mut xcb_input_device_presence_notify_event_t }}
    }
  }
}
