use testing::{splish, sploosh};

#[test]
fn test_sploosh_splish() {
    let want = 4;
    let got = sploosh(splish(-1, 0), splish(1, 1), splish(3, 2));
    assert_eq!(got, want);
}
