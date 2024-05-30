#[test]
pub fn pass() {
    use tryexpand::expand;
    expand(["tests/expand/pass/*.rs"]).expect_pass();
}
