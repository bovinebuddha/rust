// Check that we do not allow coercions to object
// unsafe trait objects in match arms

#![feature(object_safe_for_dispatch)]

trait Trait: Sized {}

struct S;

impl Trait for S {}

struct R;

impl Trait for R {}

fn opt() -> Option<()> {
    Some(())
}

fn main() {
    match opt() { //~ ERROR E0308
        Some(()) => &S,
        None => &R,
    }
    let t: &dyn Trait = match opt() { //~ ERROR E0308
        Some(()) => &S,
        None => &R,
    };
}
