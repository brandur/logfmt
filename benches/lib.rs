extern crate logfmt;
extern crate test;

use test::Bencher;

#[bench]
fn parse(b: &mut Bencher) {
    b.iter(|| {
        logfmt::parse("measure.test=1 measure.foo=bar measure.time=2h measure=\"foo\"")
    })
}
