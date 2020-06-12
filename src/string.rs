// <string.h> style string library.
use core::mem;
use core::slice;

// c char is signed 8 bit integer.
#[allow(non_camel_case_types)]
pub type c_char = i8;

#[no_mangle]
pub extern "C" fn memset<T>(dst: *const T) {}

#[no_mangle]
pub extern "C" fn memcpy<T>(dst: *const T) {}

#[no_mangle]
pub extern "C" fn memcmp<T>(dst: *const T) {}

#[no_mangle]
pub extern "C" fn memmove<T>(dst: *const T) {}


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

    let (ss, ts) = {
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
