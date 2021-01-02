use shortscale::shortscale;

#[test]
fn call_with_a_bunch_of_different_values() {
    for (num, expected) in [
    (0, "zero"),
    (1, "one"),
    (10, "ten"),
    (11, "eleven"),
    (18, "eighteen"),
    (20, "twenty"),
    (22, "twenty two"),
    (30, "thirty"),
    (33, "thirty three"),
    (111, "one hundred and eleven"),
    (120, "one hundred and twenty"),
    (121, "one hundred and twenty one"),
    (300, "three hundred"),
    (999, "nine hundred and ninety nine"),
    (1_000, "one thousand"),
    (2_000, "two thousand"),
    (2_004, "two thousand and four"),
    (2_011, "two thousand and eleven"),
    (2_020, "two thousand and twenty"),
    (2_050, "two thousand and fifty"),
    (2_300, "two thousand three hundred"),
    (2_301, "two thousand three hundred and one"),
    (30_020, "thirty thousand and twenty"),
    (430_020, "four hundred and thirty thousand and twenty"),
    (430_920, "four hundred and thirty thousand nine hundred and twenty"),
    (999_001, "nine hundred and ninety nine thousand and one"),
    (999_120, "nine hundred and ninety nine thousand one hundred and twenty"),
    (1_000_000, "one million"),
    (1_001_000, "one million one thousand"),
    (1_002_000, "one million two thousand"),
    (1_002_004, "one million two thousand and four"),
    (1_002_011, "one million two thousand and eleven"),
    (1_002_020, "one million two thousand and twenty"),
    (1_002_050, "one million two thousand and fifty"),
    (1_002_300, "one million two thousand three hundred"),
    (1_002_301, "one million two thousand three hundred and one"),
    (1_030_020, "one million thirty thousand and twenty"),
    (1_430_020, "one million four hundred and thirty thousand and twenty"),
    (1_430_920, "one million four hundred and thirty thousand nine hundred and twenty"),
    (1_999_001, "one million nine hundred and ninety nine thousand and one"),
    (100_999_120, "one hundred million nine hundred and ninety nine thousand one hundred and twenty"),
    (999_999_120, "nine hundred and ninety nine million nine hundred and ninety nine thousand one hundred and twenty"),
    (420_000_999_015, "four hundred and twenty billion nine hundred and ninety nine thousand and fifteen"),
    (9_007_199_254_740_999, "nine quadrillion seven trillion one hundred and ninety nine billion \
        two hundred and fifty four million seven hundred and fourty thousand nine hundred and ninety nine"),
    (999_999_999_999_999_999, "nine hundred and ninety nine quadrillion nine hundred and ninety nine trillion \
        nine hundred and ninety nine billion nine hundred and ninety nine million nine hundred and ninety nine thousand \
        nine hundred and ninety nine"),
    (1_999_999_999_999_999_999, "(big number)")
    ].iter() {
        println!("test {}", num);
        assert_eq!(shortscale(*num), String::from(*expected));
    }
}