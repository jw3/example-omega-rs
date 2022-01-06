#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_void, CString};
use std::fmt::{Display, Formatter};
use std::ptr;
use std::rc::Rc;

include!("bindings.rs");

type ViewportCallback = Box<dyn Fn(&Viewport)>;

pub struct Session {
    p: *mut omega_session_t,
    views: Vec<Rc<Viewport>>,
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            omega_edit_destroy_session(self.p);
        }
    }
}

impl Session {
    pub fn new() -> Self {
        let p = unsafe { omega_edit_create_session(ptr::null(), None, ptr::null_mut()) };
        Self { p, views: vec![] }
    }

    pub fn from_file(file_path: &str) -> Self {
        let file_path = CString::new(file_path).unwrap();
        let p = unsafe { omega_edit_create_session(file_path.as_ptr(), None, ptr::null_mut()) };
        Self { p, views: vec![] }
    }

    pub fn view(&mut self, offset: i64, size: i64) -> ViewportPtr {
        let p = unsafe { omega_edit_create_viewport(self.p, offset, size, None, ptr::null_mut()) };
        let rc = Rc::new(Viewport { p, f: None });
        self.views.push(rc.clone());
        rc
    }

    pub fn view_cb(&mut self, offset: i64, size: i64, cb: ViewportCallback) -> ViewportPtr {
        let p = unsafe {
            omega_edit_create_viewport(self.p, offset, size, Some(vpt_change_cbk), ptr::null_mut())
        };
        let rc = Rc::new(Viewport { p, f: Some(cb) });
        unsafe {
            // todo;; flip flop to eliminate a leak (is there a better way...)
            (*p).user_data_ptr = Rc::into_raw(rc.clone()) as *mut c_void;
            self.views
                .push(Rc::from_raw((*p).user_data_ptr as *const Viewport));
        }
        rc
    }

    pub fn push(&mut self, s: &str) {
        let s = CString::new(s).unwrap();
        unsafe {
            omega_edit_insert(self.p, 0, s.as_c_str().as_ptr(), 0);
        }
    }

    pub fn insert(&mut self, s: &str, offset: i64) {
        let s = CString::new(s).unwrap();
        unsafe {
            omega_edit_insert(self.p, offset, s.as_c_str().as_ptr(), 0);
        }
    }

    pub fn overwrite(&mut self, s: &str, offset: i64) {
        let s = CString::new(s).unwrap();
        unsafe {
            omega_edit_overwrite(self.p, offset, s.as_c_str().as_ptr(), 0);
        }
    }

    pub fn delete(&mut self, offset: i64, len: i64) {
        unsafe {
            omega_edit_delete(self.p, offset, len);
        }
    }
}

pub type ViewportPtr = Rc<Viewport>;

pub struct Viewport {
    p: *mut omega_viewport_t,
    f: Option<ViewportCallback>,
}

impl Viewport {
    pub fn update(&self, offset: i64, size: i64) {
        unsafe {
            omega_viewport_update(self.p, offset, size);
        }
    }

    pub fn len(&self) -> i64 {
        unsafe { omega_viewport_get_length(self.p) }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            let len = omega_viewport_get_length(self.p);
            let dat = omega_viewport_get_data(self.p);
            std::slice::from_raw_parts(dat, len as usize)
        }
    }
}

impl Display for Viewport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8(self.data().into()).unwrap())
    }
}

extern "C" fn vpt_change_cbk(v: *const omega_viewport_t, _: *const omega_change_t) {
    unsafe {
        let x = omega_viewport_get_user_data(v);
        if !x.is_null() {
            let vp = &*(x as *mut Viewport);
            if vp.f.is_some() {
                ((*vp).f.as_ref().unwrap())(&*vp);
            }
        }
    }
}
