/*!
 *  Low-level internal wrapper API around the raw bindings created by bindgen.
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


include!(concat!(env!("OUT_DIR"), "/libClearCore_bindings.rs"));