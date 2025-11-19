mod common;
use common::case::TestCase;

#[macro_export]
macro_rules! assert_eq_dbg {
    ($x:expr, $y:expr $(,)?     ) =>(assert_eq!(format!("{:?}", $x), format!("{:?}", $y)));
    ($x:expr, $y:expr, $($z:tt)+) =>(assert_eq!(format!("{:?}", $x), format!("{:?}", $y), $($z)*));
}

#[test]
fn test_one() {
    for test in TestCase::all() {
        assert_eq!(test.expect.is_empty(),      test.ansi.is_empty(),   "{:?}.is_empty()",   test.ansi);
        assert_eq!(test.expect.is_reset(),      test.ansi.is_reset(),   "{:?}.is_reset()",   test.ansi);
        assert_eq!(test.expect.is_no_ansi(),    test.ansi.is_no_ansi(), "{:?}.is_no_ansi()", test.ansi);
        assert_eq_dbg!(test.expect,             test.ansi);
        assert_eq_dbg!(test.expect.not(),       test.ansi.not());
        assert_eq_dbg!(test.expect.not().not(), test.ansi.not().not());
        assert_eq_dbg!(test.expect.important(), test.ansi.important());
    }
}

#[test]
fn test_combine() {
    // 1-attr + 1-attr
    for a in TestCase::all() {
        for b in TestCase::all() {
            check_all(a, b);
        }
    }

    // 2-attrs + 1-attr
    for a in TestCase::all() {
        for b in TestCase::all() {
            for c in TestCase::all() {
                check_all(a.add(c),       b);
                check_all(a.add(c),       b.not());
                check_all(a.not().add(c), b);
                check_all(a.not().add(c), b.not());
                check_all(a,              b.add(c));
                check_all(a.not(),        b.add(c));
                check_all(a,              b.not().add(c));
                check_all(a.not(),        b.not().add(c));
            }
        }
    }
}

fn check_all(a: TestCase, b: TestCase) {
    check_operations(a, b);
    check_operations(a, b.not());
    check_operations(a.not(), b);
    check_operations(a.not(), b.not());
}

fn check_operations(a: TestCase, b: TestCase) {
    check_add (a, b);
    check_then(a, b);
    check_transition(a, b);
}

fn check_add(a: TestCase, b: TestCase) {
    assert_eq_dbg!(
        a.expect.add(b.expect),
        a.ansi.add(b.ansi),
        "{:?}.add({:?})", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.add(b.expect.important()),
        a.ansi.add(b.ansi.important()),
        "{:?}.add({:?}.important())", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.important().add(b.expect),
        a.ansi.important().add(b.ansi),
        "{:?}.important().add({:?})", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.important().add(b.expect.important()),
        a.ansi.important().add(b.ansi.important()),
        "{:?}.important().add({:?}.important())", a.ansi, b.ansi,
    );
}

fn check_then(a: TestCase, b: TestCase) {
    assert_eq_dbg!(
        a.expect.then(b.expect),
        a.ansi.then(b.ansi),
        "{:?}.then({:?})", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.then(b.expect.important()),
        a.ansi.then(b.ansi.important()),
        "{:?}.then({:?}.important())", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.important().then(b.expect),
        a.ansi.important().then(b.ansi),
        "{:?}.important().then({:?})", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.important().then(b.expect.important()),
        a.ansi.important().then(b.ansi.important()),
        "{:?}.important().then({:?}.important())", a.ansi, b.ansi,
    );
}

fn check_transition(a: TestCase, b: TestCase) {
    assert_eq_dbg!(
        a.expect.transition(b.expect),
        a.ansi.transition(b.ansi),
        "{:?}.transition({:?})", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.transition(b.expect.important()),
        a.ansi.transition(b.ansi.important()),
        "{:?}.transition({:?}.important())", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.important().transition(b.expect),
        a.ansi.important().transition(b.ansi),
        "{:?}.important().transition({:?})", a.ansi, b.ansi,
    );

    assert_eq_dbg!(
        a.expect.important().transition(b.expect.important()),
        a.ansi.important().transition(b.ansi.important()),
        "{:?}.important().transition({:?}.important())", a.ansi, b.ansi,
    );
}
