// <string.h> style string library. can be called from c code directly.
use std::ffi::c_void;
use std::slice;

// c char is signed 8 bit integer.
#[allow(non_camel_case_types)]
pub type c_char = i8;

#[no_mangle]
pub extern "C" fn memset(dst: *mut c_void, c: c_char, n: usize) -> *mut c_void {
    let cdst = dst as *mut c_char;
    let n = n as isize;
    (0..n).map(|i| unsafe {
        *cdst.offset(i) = c;
    });
    dst
}

#[no_mangle]
pub extern "C" fn memcmp(v1: *const c_void, v2: *const c_void, n: usize) -> i32 {
    let s1 = unsafe { slice::from_raw_parts(v1 as *const c_char, n) };
    let s2 = unsafe { slice::from_raw_parts(v2 as *const c_char, n) };
    for (a, b) in s1.iter().zip(s2.iter()) {
        let diff = *a - *b;
        if diff != 0 {
            return diff as i32;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    let s1 = unsafe { slice::from_raw_parts_mut(dst as *mut c_char, n) };
    let s2 = unsafe { slice::from_raw_parts_mut(src as *mut c_char, n) };
    for (a, b) in s1.iter_mut().zip(s2.iter()) {
        *a = *b;
    }
    dst
}

#[no_mangle]
pub extern "C" fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    memcpy(dst, src, n)
}

#[no_mangle]
pub extern "C" fn strlen(s: *const c_char) -> isize {
    unsafe {
        let mut n = 0;
        while *s.offset(n) != 0 {
            n += 1;
        }
        n
    }
}

#[no_mangle]
pub extern "C" fn strncpy(s: *mut c_char, t: *const c_char, n: i32) -> *const c_char {
    if n <= 0 {
        return s;
    }

    let (ss, ts) = unsafe {
        let ss = slice::from_raw_parts_mut(s, n as usize);
        let ts = slice::from_raw_parts(t, n as usize);
        (ss, ts)
    };

    ss.iter_mut().zip(ts).map(|(e, ch)| {
        *e = *ch;
    });
    s
}

// guaranteed to NUL terminated.
#[no_mangle]
pub extern "C" fn safestrncpy(s: *mut c_char, t: *const c_char, n: i32) -> *const c_char {
    if n < 0 {
        return s;
    }

    let (ss, ts) = unsafe {
        let ss = slice::from_raw_parts_mut(s, n as usize);
        let ts = slice::from_raw_parts(t, n as usize);
        (ss, ts)
    };
    ss.iter_mut()
        .zip(ts.iter().chain([0].iter()))
        .map(|(e, ch)| {
            *e = *ch;
        });
    s
}

#[no_mangle]
pub extern "C" fn strncmp(p: *const c_char, q: *const c_char, n: u32) -> isize {
    let (ps, qs) = unsafe {
        let ps = slice::from_raw_parts(p, n as usize);
        let qs = slice::from_raw_parts(q, n as usize);
        (ps, qs)
    };

    for (ps, qs) in ps.iter().take_while(|&&x| x != 0).chain([0].iter()).zip(qs) {
        if *ps != *qs {
            return (*ps - *qs) as isize;
        }
    }
    0
}
