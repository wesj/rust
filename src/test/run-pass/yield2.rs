// -*- rust -*-
use std;

fn main() {
    let i: int = 0;
    while i < 100 { i = i + 1; log(error, i); task::yield(); }
}
