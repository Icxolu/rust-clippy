//@run-rustfix

#![allow(unused)]
#![warn(clippy::needless_into_iter)]

fn main() {
    let sample = vec![0, 1, 2];
    let sample2 = vec!['t', 'e', 's', 't'];

    vec![].extend(Vec::<i32>::new().into_iter());
    foo(Vec::new().into_iter());
    bar(sample.clone().into_iter(), sample.clone().into_iter());
    baz(sample, (), sample2.into_iter());
}

fn foo(_: impl IntoIterator<Item = usize>) {}
fn bar<I: IntoIterator<Item = usize>>(_: impl Iterator<Item = usize>, _: I) {}
fn baz<I: IntoIterator<Item = usize>>(_: I, _: (), _: impl IntoIterator<Item = char>) {}
