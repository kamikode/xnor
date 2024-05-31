#[test]
pub fn pass() {
    tryexpand::expand(["tests/expand/pass/*.rs"]).expect_pass();
}
