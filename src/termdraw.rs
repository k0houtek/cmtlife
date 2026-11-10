pub fn set_position(x: usize, y: usize) {
    print!("\x1b[{};{}H", y+1, x*2+1);
}

pub fn fill(on: bool) {
    if on { print!("##"); }
    else { print!("  "); }
}