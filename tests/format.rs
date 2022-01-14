use ::fstrings::format_f;

#[test]
fn format_f_single_line() {
    let x = 3;
    let y = 40;
    let z = -7;

    assert_eq!(&format_f!("{x} {y} {z}"), "3 40 -7");
}

#[test]
fn format_f_multi_line() {
    let x = 3;
    let y = 40;
    let z = -7;

    assert_eq!(
        &format_f!(
            "{x}
{y}
{z}"
        ),
        "3
40
-7"
    );
}
