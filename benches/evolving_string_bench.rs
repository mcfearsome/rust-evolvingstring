#![feature(test)]

extern crate test;
use test::Bencher;
use evolvingstring::EvolvingString;

#[bench]
fn bench_predict_10_thousand_times(b: &mut Bencher) {
    let initial_string = "test_string".to_string();
    let secret = "secret".to_string();
    let interval_seconds = 60;
    let evolving_string = EvolvingString::new(initial_string, secret, interval_seconds);
    let mut last = 0;

    b.iter(|| {
        let n = test::black_box(10000);
        for i in 0..n {
          test::black_box(evolving_string.predict(10 * i + last));
          last += 10 * i
        }
    });
}
