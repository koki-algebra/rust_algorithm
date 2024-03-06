use algorithm::prime_factorize;

#[test]
fn test_prime_factorize() {
    // Test cases with expected prime factors and their exponents
    let test_cases = vec![
        (12, vec![(2, 2), (3, 1)]),
        (24, vec![(2, 3), (3, 1)]),
        (25, vec![(5, 2)]),
        (30, vec![(2, 1), (3, 1), (5, 1)]),
        (123456789, vec![(3, 2), (3607, 1), (3803, 1)]),
        (
            876543210,
            vec![(2, 1), (3, 2), (5, 1), (1997, 1), (4877, 1)],
        ),
        (987654321, vec![(3, 2), (17, 2), (379721, 1)]),
    ];

    for (input, expected_factors) in test_cases {
        let result = prime_factorize(input);

        // Check if the number of factors match
        assert_eq!(result.len(), expected_factors.len());

        // Check if each prime factor and its exponent are correct
        for (prime, exp) in expected_factors {
            assert!(result.contains_key(&prime));
            assert_eq!(result[&prime], exp);
        }
    }
}
