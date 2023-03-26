// fn main() {
//     let mut a = vec![];
//     let mut b = vec![];

//     let mut f = || {
//         a.push(1);
//         b.push(a.last().unwrap()); // error here coz of reborrow
//     };

//     f();

//     dbg!(&b); // none of these
//     dbg!(&a); // has been moved
// }

// fn main() {
//     let mut a = vec![];
//     let mut b = vec![];

//     a.push(1); // this gets rid of the error due to reborrow

//     let mut f = || {
//         b.push(a.last().unwrap());
//     };

//     f();

//     dbg!(&b); // none of these
//     dbg!(&a); // has been moved
// }

struct UnrelatedStruct {}

pub fn exec() {
    let mut a = vec![];
    let mut b = vec![];

    let _unrelated_var = UnrelatedStruct {};

    let f = || {
        let _unrelated_var = _unrelated_var; // this stops the error as the Closure no longer implements FnMut?
        a.push(1);
        b.push(a.last().unwrap());
    };

    f();

    dbg!(&b); // none of these
    dbg!(&a); // has been moved
}
