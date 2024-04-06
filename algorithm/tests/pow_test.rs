use algorithm::pow;


#[test]
fn test_pow() {
    const MOD: usize = 1000000007;

    // Test cases: base, exp, modulus, expected_result
    let test_cases = vec![
        (2, 10, MOD, 1024),
    ];

    for (base, exp, m, expected_result) in test_cases {
        let result = pow(base, exp, m);
        assert_eq!(
            result, expected_result,
            "Failed for base: {}, exp: {}, modulus: {}. Expected: {}, Actual: {}",
            base, exp, m, expected_result, result
        );
    }
}
