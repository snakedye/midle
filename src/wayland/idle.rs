use std::os::raw::{c_char, c_void};
const NULLPTR: *const c_void = 0 as *const c_void;
static mut types_null: [*const sys::common::wl_interface; 0] = [];
#[doc = "User idle time manager\n\nThis interface allows to monitor user idle time on a given seat. The interface\nallows to register timers which trigger after no user activity was registered\non the seat for a given interval. It notifies when user activity resumes.\n\nThis is useful for applications wanting to perform actions when the user is not\ninteracting with the system, e.g. chat applications setting the user as away, power\nmanagement features to dim screen, etc.."]
pub mod org_kde_kwin_idle {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = ""]
        GetIdleTimeout {
            seat: super::wl_seat::WlSeat,
            timeout: u32,
        },
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[super::MessageDesc {
            name: "get_idle_timeout",
            since: 1,
            signature: &[
                super::ArgumentType::NewId,
                super::ArgumentType::Object,
                super::ArgumentType::Uint,
            ],
            destructor: false,
        }];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::GetIdleTimeout { .. } => 0,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::GetIdleTimeout { .. } => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                0 => Some(Object::from_interface::<
                    super::org_kde_kwin_idle_timeout::OrgKdeKwinIdleTimeout,
                >(version, meta.child())),
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::GetIdleTimeout { seat, timeout } => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![
                        Argument::NewId(0),
                        Argument::Object(seat.as_ref().id()),
                        Argument::Uint(timeout),
                    ],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::GetIdleTimeout { seat, timeout } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = ::std::ptr::null_mut() as *mut _;
                    _args_array[1].o = seat.as_ref().c_ptr() as *mut _;
                    _args_array[2].u = timeout;
                    f(0, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {}
        }
        fn opcode(&self) -> u16 {
            match *self {}
        }
        fn since(&self) -> u32 {
            match *self {}
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct OrgKdeKwinIdle(Proxy<OrgKdeKwinIdle>);
    impl AsRef<Proxy<OrgKdeKwinIdle>> for OrgKdeKwinIdle {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<OrgKdeKwinIdle>> for OrgKdeKwinIdle {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            OrgKdeKwinIdle(value)
        }
    }
    impl From<OrgKdeKwinIdle> for Proxy<OrgKdeKwinIdle> {
        #[inline]
        fn from(value: OrgKdeKwinIdle) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for OrgKdeKwinIdle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for OrgKdeKwinIdle {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "org_kde_kwin_idle";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &org_kde_kwin_idle_interface }
        }
    }
    impl OrgKdeKwinIdle {
        #[doc = ""]
        pub fn get_idle_timeout(
            &self,
            seat: &super::wl_seat::WlSeat,
            timeout: u32,
        ) -> Main<super::org_kde_kwin_idle_timeout::OrgKdeKwinIdleTimeout> {
            let msg = Request::GetIdleTimeout {
                seat: seat.clone(),
                timeout: timeout,
            };
            self.0.send(msg, None).unwrap()
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_IDLE_TIMEOUT_SINCE: u32 = 1u32;
    static mut org_kde_kwin_idle_requests_get_idle_timeout_types: [*const wl_interface; 3] = [
        unsafe {
            &super::org_kde_kwin_idle_timeout::org_kde_kwin_idle_timeout_interface
                as *const wl_interface
        },
        unsafe { &super::wl_seat::wl_seat_interface as *const wl_interface },
        NULLPTR as *const wl_interface,
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut org_kde_kwin_idle_requests: [wl_message; 1] = [wl_message {
        name: b"get_idle_timeout\0" as *const u8 as *const c_char,
        signature: b"nou\0" as *const u8 as *const c_char,
        types: unsafe { &org_kde_kwin_idle_requests_get_idle_timeout_types as *const _ },
    }];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut org_kde_kwin_idle_interface: wl_interface = wl_interface {
        name: b"org_kde_kwin_idle\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 1,
        requests: unsafe { &org_kde_kwin_idle_requests as *const _ },
        event_count: 0,
        events: NULLPTR as *const wl_message,
    };
}
pub mod org_kde_kwin_idle_timeout {
    use super::sys::client::*;
    use super::sys::common::{wl_argument, wl_array, wl_interface, wl_message};
    use super::{
        smallvec, types_null, AnonymousObject, Argument, ArgumentType, Interface, Main, Message,
        MessageDesc, MessageGroup, Object, ObjectMetadata, Proxy, NULLPTR,
    };
    use std::os::raw::c_char;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "release the timeout object\n\n\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Release,
        #[doc = "Simulates user activity for this timeout, behaves just like real user activity on the seat"]
        SimulateUserActivity,
    }
    impl super::MessageGroup for Request {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "release",
                since: 1,
                signature: &[],
                destructor: true,
            },
            super::MessageDesc {
                name: "simulate_user_activity",
                since: 1,
                signature: &[],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Request::Release => 0,
                Request::SimulateUserActivity => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Request::Release => 1,
                Request::SimulateUserActivity => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            panic!("Request::from_raw can not be used Client-side.")
        }
        fn into_raw(self, sender_id: u32) -> Message {
            match self {
                Request::Release => Message {
                    sender_id: sender_id,
                    opcode: 0,
                    args: smallvec![],
                },
                Request::SimulateUserActivity => Message {
                    sender_id: sender_id,
                    opcode: 1,
                    args: smallvec![],
                },
            }
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Request, ()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            match self {
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                }
                Request::SimulateUserActivity => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                }
            }
        }
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "Triggered when there has not been any user activity in the requested idle time interval"]
        Idle,
        #[doc = "Triggered on the first user activity after an idle event"]
        Resumed,
    }
    impl super::MessageGroup for Event {
        const MESSAGES: &'static [super::MessageDesc] = &[
            super::MessageDesc {
                name: "idle",
                since: 1,
                signature: &[],
                destructor: false,
            },
            super::MessageDesc {
                name: "resumed",
                since: 1,
                signature: &[],
                destructor: false,
            },
        ];
        type Map = super::ProxyMap;
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false,
            }
        }
        fn opcode(&self) -> u16 {
            match *self {
                Event::Idle => 0,
                Event::Resumed => 1,
            }
        }
        fn since(&self) -> u32 {
            match *self {
                Event::Idle => 1,
                Event::Resumed => 1,
            }
        }
        fn child<Meta: ObjectMetadata>(
            opcode: u16,
            version: u32,
            meta: &Meta,
        ) -> Option<Object<Meta>> {
            match opcode {
                _ => None,
            }
        }
        fn from_raw(msg: Message, map: &mut Self::Map) -> Result<Self, ()> {
            match msg.opcode {
                0 => Ok(Event::Idle),
                1 => Ok(Event::Resumed),
                _ => Err(()),
            }
        }
        fn into_raw(self, sender_id: u32) -> Message {
            panic!("Event::into_raw can not be used Client-side.")
        }
        unsafe fn from_raw_c(
            obj: *mut ::std::os::raw::c_void,
            opcode: u32,
            args: *const wl_argument,
        ) -> Result<Event, ()> {
            match opcode {
                0 => Ok(Event::Idle),
                1 => Ok(Event::Resumed),
                _ => return Err(()),
            }
        }
        fn as_raw_c_in<F, T>(self, f: F) -> T
        where
            F: FnOnce(u32, &mut [wl_argument]) -> T,
        {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }
    #[derive(Clone, Eq, PartialEq)]
    pub struct OrgKdeKwinIdleTimeout(Proxy<OrgKdeKwinIdleTimeout>);
    impl AsRef<Proxy<OrgKdeKwinIdleTimeout>> for OrgKdeKwinIdleTimeout {
        #[inline]
        fn as_ref(&self) -> &Proxy<Self> {
            &self.0
        }
    }
    impl From<Proxy<OrgKdeKwinIdleTimeout>> for OrgKdeKwinIdleTimeout {
        #[inline]
        fn from(value: Proxy<Self>) -> Self {
            OrgKdeKwinIdleTimeout(value)
        }
    }
    impl From<OrgKdeKwinIdleTimeout> for Proxy<OrgKdeKwinIdleTimeout> {
        #[inline]
        fn from(value: OrgKdeKwinIdleTimeout) -> Self {
            value.0
        }
    }
    impl std::fmt::Debug for OrgKdeKwinIdleTimeout {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.0))
        }
    }
    impl Interface for OrgKdeKwinIdleTimeout {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "org_kde_kwin_idle_timeout";
        const VERSION: u32 = 1;
        fn c_interface() -> *const wl_interface {
            unsafe { &org_kde_kwin_idle_timeout_interface }
        }
    }
    impl OrgKdeKwinIdleTimeout {
        #[doc = "release the timeout object\n\n\n\nThis is a destructor, you cannot send requests to this object any longer once this method is called."]
        pub fn release(&self) -> () {
            let msg = Request::Release;
            self.0.send::<AnonymousObject>(msg, None);
        }
        #[doc = "Simulates user activity for this timeout, behaves just like real user activity on the seat"]
        pub fn simulate_user_activity(&self) -> () {
            let msg = Request::SimulateUserActivity;
            self.0.send::<AnonymousObject>(msg, None);
        }
    }
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_RELEASE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_SIMULATE_USER_ACTIVITY_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_IDLE_SINCE: u32 = 1u32;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_RESUMED_SINCE: u32 = 1u32;
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut org_kde_kwin_idle_timeout_requests: [wl_message; 2] = [
        wl_message {
            name: b"release\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"simulate_user_activity\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C-representation of the messages of this interface, for interop"]
    pub static mut org_kde_kwin_idle_timeout_events: [wl_message; 2] = [
        wl_message {
            name: b"idle\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
        wl_message {
            name: b"resumed\0" as *const u8 as *const c_char,
            signature: b"\0" as *const u8 as *const c_char,
            types: unsafe { &types_null as *const _ },
        },
    ];
    #[doc = r" C representation of this interface, for interop"]
    pub static mut org_kde_kwin_idle_timeout_interface: wl_interface = wl_interface {
        name: b"org_kde_kwin_idle_timeout\0" as *const u8 as *const c_char,
        version: 1,
        request_count: 2,
        requests: unsafe { &org_kde_kwin_idle_timeout_requests as *const _ },
        event_count: 2,
        events: unsafe { &org_kde_kwin_idle_timeout_events as *const _ },
    };
}
