const TEXT: &str = "Hello✨, 🎈 this 🎉 is a text.";

mod short_text {
    use substring::*;

    use super::TEXT;

    const START: usize = 1;
    const END: usize = 20;
    const EXPECTED: &str = "ello✨, 🎈 this 🎉 is ";

    #[divan::bench]
    fn bench_substring_v1() {
        assert_eq!(substring_v1(TEXT, START, END), EXPECTED);
    }
    #[divan::bench]
    fn bench_substring_v4() {
        assert_eq!(substring_v4(TEXT, START, END), EXPECTED);
    }
    #[divan::bench]
    fn bench_substring_v5() {
        assert_eq!(substring_v5(TEXT, START, END), EXPECTED);
    }
}

mod large_text {
    use substring::*;

    use super::TEXT;

    const REPEAT: usize = 1000;
    const START: usize = 1;
    const END: usize = 20000;

    #[divan::bench]
    fn bench_substring_v1(bencher: divan::Bencher) {
        let input = TEXT.repeat(REPEAT);

        bencher.bench(|| substring_v1(&input, START, END))
    }
    #[divan::bench]
    fn bench_substring_v4(bencher: divan::Bencher) {
        let input = TEXT.repeat(REPEAT);

        bencher.bench(|| substring_v4(&input, START, END))
    }
    #[divan::bench]
    fn bench_substring_v5(bencher: divan::Bencher) {
        let input = TEXT.repeat(REPEAT);

        bencher.bench(|| substring_v5(&input, START, END))
    }
}

fn main() {
    divan::main();
}
