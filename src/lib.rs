#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn smoke_test() {
            let mut major: u32 = 0;
            let mut minor: u32 = 0;
            DAQmxGetSysNIDAQMajorVersion(&mut major as *mut u32 );
            DAQmxGetSysNIDAQMinorVersion(&mut minor as *mut u32 );
            println!("Detected Version {}.{}", major, minor);

    }
}
