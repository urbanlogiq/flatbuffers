extern crate alloc;

use alloc::string::ToString;
use alloc::vec::Vec;

#[allow(dead_code, unused_imports)]
#[path = "../../apache_arrow/mod.rs"]
mod arrow;
use self::more_defaults_generated::*;

#[test]
fn arrow() {
    // this test introduces types that overlap with reserved words and due to #7617 will fail to
    // compile
}

