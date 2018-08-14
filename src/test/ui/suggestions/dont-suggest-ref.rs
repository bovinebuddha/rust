// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(nll)]

#[derive(Clone)]
enum Either {
    One(X),
    Two(X),
}

#[derive(Clone)]
struct X(Y);

#[derive(Clone)]
struct Y;

pub fn main() {
    let e = Either::One(X(Y));
    let mut em = Either::One(X(Y));

    let r = &e;
    let rm = &mut Either::One(X(Y));

    let x = X(Y);
    let mut xm = X(Y);

    let s = &x;
    let sm = &mut X(Y);

    let ve = vec![Either::One(X(Y))];

    let vr = &ve;
    let vrm = &mut vec![Either::One(X(Y))];

    let vx = vec![X(Y)];

    let vs = &vx;
    let vsm = &mut vec![X(Y)];

    // -------- move from Either/X place --------

    let X(_t) = *s;
    //~^ ERROR cannot move
    //~| HELP consider removing the `*`
    //~| SUGGESTION s
    if let Either::One(_t) = *r { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `*`
    //~| SUGGESTION r
    while let Either::One(_t) = *r { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `*`
    //~| SUGGESTION r
    match *r {
        //~^ ERROR cannot move
        //~| HELP consider removing the `*`
        //~| SUGGESTION r
        Either::One(_t)
        | Either::Two(_t) => (),
    }
    match *r {
        //~^ ERROR cannot move
        //~| HELP consider removing the `*`
        //~| SUGGESTION r
        Either::One(_t) => (),
        Either::Two(ref _t) => (),
        // FIXME: should suggest removing `ref` too
    }

    let X(_t) = *sm;
    //~^ ERROR cannot move
    //~| HELP consider removing the `*`
    //~| SUGGESTION sm
    if let Either::One(_t) = *rm { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `*`
    //~| SUGGESTION rm
    while let Either::One(_t) = *rm { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `*`
    //~| SUGGESTION rm
    match *rm {
        //~^ ERROR cannot move
        //~| HELP consider removing the `*`
        //~| SUGGESTION rm
        Either::One(_t)
        | Either::Two(_t) => (),
    }
    match *rm {
        //~^ ERROR cannot move
        //~| HELP consider removing the `*`
        //~| SUGGESTION rm
        Either::One(_t) => (),
        Either::Two(ref _t) => (),
        // FIXME: should suggest removing `ref` too
    }
    match *rm {
        //~^ ERROR cannot move
        //~| HELP consider removing the `*`
        //~| SUGGESTION rm
        Either::One(_t) => (),
        Either::Two(ref mut _t) => (),
        // FIXME: should suggest removing `ref` too
    }

    let X(_t) = vs[0];
    //~^ ERROR cannot move
    //~| HELP consider borrowing here
    //~| SUGGESTION &vs[0]
    if let Either::One(_t) = vr[0] { }
    //~^ ERROR cannot move
    //~| HELP consider borrowing here
    //~| SUGGESTION &vr[0]
    while let Either::One(_t) = vr[0] { }
    //~^ ERROR cannot move
    //~| HELP consider borrowing here
    //~| SUGGESTION &vr[0]
    match vr[0] {
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        //~| SUGGESTION &vr[0]
        Either::One(_t)
        | Either::Two(_t) => (),
    }
    match vr[0] {
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        //~| SUGGESTION &vr[0]
        Either::One(_t) => (),
        Either::Two(ref _t) => (),
        // FIXME: should suggest removing `ref` too
    }

    let X(_t) = vsm[0];
    //~^ ERROR cannot move
    //~| HELP consider borrowing here
    //~| SUGGESTION &vsm[0]
    if let Either::One(_t) = vrm[0] { }
    //~^ ERROR cannot move
    //~| HELP consider borrowing here
    //~| SUGGESTION &vrm[0]
    while let Either::One(_t) = vrm[0] { }
    //~^ ERROR cannot move
    //~| HELP consider borrowing here
    //~| SUGGESTION &vrm[0]
    match vrm[0] {
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        //~| SUGGESTION &vrm[0]
        Either::One(_t)
        | Either::Two(_t) => (),
    }
    match vrm[0] {
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        //~| SUGGESTION &vrm[0]
        Either::One(_t) => (),
        Either::Two(ref _t) => (),
        // FIXME: should suggest removing `ref` too
    }
    match vrm[0] {
        //~^ ERROR cannot move
        //~| HELP consider borrowing here
        //~| SUGGESTION &vrm[0]
        Either::One(_t) => (),
        Either::Two(ref mut _t) => (),
        // FIXME: should suggest removing `ref` too
    }

    // -------- move from &Either/&X place --------

    let &X(_t) = s;
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION X(_t)
    if let &Either::One(_t) = r { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION Either::One(_t)
    while let &Either::One(_t) = r { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION Either::One(_t)
    match r {
        //~^ ERROR cannot move
        &Either::One(_t)
        //~^ HELP consider removing the `&`
        //~| SUGGESTION Either::One(_t)
        | &Either::Two(_t) => (),
        // FIXME: would really like a suggestion here too
    }
    match r {
        //~^ ERROR cannot move
        &Either::One(_t) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION Either::One(_t)
        &Either::Two(ref _t) => (),
    }
    match r {
        //~^ ERROR cannot move
        &Either::One(_t) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION Either::One(_t)
        Either::Two(_t) => (),
    }
    fn f1(&X(_t): &X) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION X(_t)

    let &mut X(_t) = sm;
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION X(_t)
    if let &mut Either::One(_t) = rm { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION Either::One(_t)
    while let &mut Either::One(_t) = rm { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION Either::One(_t)
    match rm {
        //~^ ERROR cannot move
        &mut Either::One(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        &mut Either::Two(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::Two(_t)
    }
    match rm {
        //~^ ERROR cannot move
        &mut Either::One(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        &mut Either::Two(ref _t) => (),
    }
    match rm {
        //~^ ERROR cannot move
        &mut Either::One(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        &mut Either::Two(ref mut _t) => (),
    }
    match rm {
        //~^ ERROR cannot move
        &mut Either::One(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        Either::Two(_t) => (),
    }
    fn f2(&mut X(_t): &mut X) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION X(_t)

    // -------- move from tuple of &Either/&X --------

    // FIXME: These should have suggestions.

    let (&X(_t),) = (&x.clone(),);
    //~^ ERROR cannot move
    if let (&Either::One(_t),) = (&e.clone(),) { }
    //~^ ERROR cannot move
    while let (&Either::One(_t),) = (&e.clone(),) { }
    //~^ ERROR cannot move
    match (&e.clone(),) {
        //~^ ERROR cannot move
        (&Either::One(_t),)
        | (&Either::Two(_t),) => (),
    }
    fn f3((&X(_t),): (&X,)) { }
    //~^ ERROR cannot move

    let (&mut X(_t),) = (&mut xm.clone(),);
    //~^ ERROR cannot move
    if let (&mut Either::One(_t),) = (&mut em.clone(),) { }
    //~^ ERROR cannot move
    while let (&mut Either::One(_t),) = (&mut em.clone(),) { }
    //~^ ERROR cannot move
    match (&mut em.clone(),) {
        //~^ ERROR cannot move
        (&mut Either::One(_t),) => (),
        (&mut Either::Two(_t),) => (),
    }
    fn f4((&mut X(_t),): (&mut X,)) { }
    //~^ ERROR cannot move

    // -------- move from &Either/&X value --------

    let &X(_t) = &x;
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION X(_t)
    if let &Either::One(_t) = &e { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION Either::One(_t)
    while let &Either::One(_t) = &e { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION Either::One(_t)
    match &e {
        //~^ ERROR cannot move
        &Either::One(_t)
        //~^ HELP consider removing the `&`
        //~| SUGGESTION Either::One(_t)
        | &Either::Two(_t) => (),
        // FIXME: would really like a suggestion here too
    }
    match &e {
        //~^ ERROR cannot move
        &Either::One(_t) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION Either::One(_t)
        &Either::Two(ref _t) => (),
    }
    match &e {
        //~^ ERROR cannot move
        &Either::One(_t) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION Either::One(_t)
        Either::Two(_t) => (),
    }

    let &mut X(_t) = &mut xm;
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION X(_t)
    if let &mut Either::One(_t) = &mut em { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION Either::One(_t)
    while let &mut Either::One(_t) = &mut em { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION Either::One(_t)
    match &mut em {
        //~^ ERROR cannot move
        &mut Either::One(_t)
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        | &mut Either::Two(_t) => (),
        // FIXME: would really like a suggestion here too
    }
    match &mut em {
        //~^ ERROR cannot move
        &mut Either::One(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        &mut Either::Two(ref _t) => (),
    }
    match &mut em {
        //~^ ERROR cannot move
        &mut Either::One(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        &mut Either::Two(ref mut _t) => (),
    }
    match &mut em {
        //~^ ERROR cannot move
        &mut Either::One(_t) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION Either::One(_t)
        Either::Two(_t) => (),
    }

    // -------- test for duplicate suggestions --------

    let &(X(_t), X(_u)) = &(x.clone(), x.clone());
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION (X(_t), X(_u))
    if let &(Either::One(_t), Either::Two(_u)) = &(e.clone(), e.clone()) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION (Either::One(_t), Either::Two(_u))
    while let &(Either::One(_t), Either::Two(_u)) = &(e.clone(), e.clone()) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION (Either::One(_t), Either::Two(_u))
    match &(e.clone(), e.clone()) {
        //~^ ERROR cannot move
        &(Either::One(_t), Either::Two(_u)) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        &(Either::Two(_t), Either::One(_u)) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION (Either::Two(_t), Either::One(_u))
        _ => (),
    }
    match &(e.clone(), e.clone()) {
        //~^ ERROR cannot move
        &(Either::One(_t), Either::Two(_u))
        //~^ HELP consider removing the `&`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        | &(Either::Two(_t), Either::One(_u)) => (),
        // FIXME: would really like a suggestion here too
        _ => (),
    }
    match &(e.clone(), e.clone()) {
        //~^ ERROR cannot move
        &(Either::One(_t), Either::Two(_u)) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        &(Either::Two(ref _t), Either::One(ref _u)) => (),
        _ => (),
    }
    match &(e.clone(), e.clone()) {
        //~^ ERROR cannot move
        &(Either::One(_t), Either::Two(_u)) => (),
        //~^ HELP consider removing the `&`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        (Either::Two(_t), Either::One(_u)) => (),
        _ => (),
    }
    fn f5(&(X(_t), X(_u)): &(X, X)) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&`
    //~| SUGGESTION (X(_t), X(_u))

    let &mut (X(_t), X(_u)) = &mut (xm.clone(), xm.clone());
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION (X(_t), X(_u))
    if let &mut (Either::One(_t), Either::Two(_u)) = &mut (em.clone(), em.clone()) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION (Either::One(_t), Either::Two(_u))
    while let &mut (Either::One(_t), Either::Two(_u)) = &mut (em.clone(), em.clone()) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION (Either::One(_t), Either::Two(_u))
    match &mut (em.clone(), em.clone()) {
        //~^ ERROR cannot move
        &mut (Either::One(_t), Either::Two(_u)) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        &mut (Either::Two(_t), Either::One(_u)) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION (Either::Two(_t), Either::One(_u))
        _ => (),
    }
    match &mut (em.clone(), em.clone()) {
        //~^ ERROR cannot move
        &mut (Either::One(_t), Either::Two(_u))
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        | &mut (Either::Two(_t), Either::One(_u)) => (),
        // FIXME: would really like a suggestion here too
        _ => (),
    }
    match &mut (em.clone(), em.clone()) {
        //~^ ERROR cannot move
        &mut (Either::One(_t), Either::Two(_u)) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        &mut (Either::Two(ref _t), Either::One(ref _u)) => (),
        _ => (),
    }
    match &mut (em.clone(), em.clone()) {
        //~^ ERROR cannot move
        &mut (Either::One(_t), Either::Two(_u)) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        &mut (Either::Two(ref mut _t), Either::One(ref mut _u)) => (),
        _ => (),
    }
    match &mut (em.clone(), em.clone()) {
        //~^ ERROR cannot move
        &mut (Either::One(_t), Either::Two(_u)) => (),
        //~^ HELP consider removing the `&mut`
        //~| SUGGESTION (Either::One(_t), Either::Two(_u))
        (Either::Two(_t), Either::One(_u)) => (),
        _ => (),
    }
    fn f6(&mut (X(_t), X(_u)): &mut (X, X)) { }
    //~^ ERROR cannot move
    //~| HELP consider removing the `&mut`
    //~| SUGGESTION (X(_t), X(_u))
}
