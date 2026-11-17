// Integration test for SigmaOS-buck2
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use buck2 as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
