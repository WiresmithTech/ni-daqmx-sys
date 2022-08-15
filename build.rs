// Credit to aimraeva on Github who provided a starting point for this file under the MIT license at https://github.com/amiraeva/nidaqmx-sys/blob/master/build.rs

macro_rules! windows_ni_path {
    () => {
        r"C:\Program Files (x86)\National Instruments\Shared\ExternalCompilerSupport\C\"
    };
}

#[cfg(all(windows, target_pointer_width = "32"))]
const LIB_DIR: &str = concat!(windows_ni_path!(), r"lib32\msvc\");

#[cfg(all(windows, target_pointer_width = "64"))]
const LIB_DIR: &str = concat!(windows_ni_path!(), r"lib64\msvc\");

#[cfg(not(windows))]
const LIB_DIR: &str = "/usr/lib/x86_64-linux-gnu/";

#[cfg(all(not(windows), target_pointer_width = "32"))]
compile_error!("nidaqmx-sys only compiles for 64 bit Unix OSes");

fn main() {
    // Tell cargo to link to nidaqmx
    println!(r"cargo:rustc-link-search={}", LIB_DIR);
    println!(r"cargo:rustc-link-lib=nidaqmx");
    //println!(r"cargo:rustc-link-lib=user32");
}
