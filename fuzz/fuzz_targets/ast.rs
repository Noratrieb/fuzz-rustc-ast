#![no_main]
use libfuzzer_sys::fuzz_target;

use fuzz_rustc_ast::RustCode;

fuzz_target!(|_data: RustCode| {
    // fuzzed code goes here
});
