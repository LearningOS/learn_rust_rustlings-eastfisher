// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)


fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x; // NLL起作用了, 即使y没有离开作用域, 也不会与z冲突
    *z += 1000;
    assert_eq!(x, 1200);
}
