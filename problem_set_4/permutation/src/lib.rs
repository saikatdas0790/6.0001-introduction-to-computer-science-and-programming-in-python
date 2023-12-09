#[allow(dead_code)]
pub fn get_permutations(sequence: String) -> Vec<String> {
    // Enumerate all permutations of a given string

    // sequence (string): an arbitrary string to permute. Assume that it is a
    // non-empty string.

    // You MUST use recursion for this part. Non-recursive solutions will not be
    // accepted.

    // Returns: a list of all permutations of sequence

    // Example:
    // >>> get_permutations('abc')
    // ['abc', 'acb', 'bac', 'bca', 'cab', 'cba']

    // Note: depending on your implementation, you may return the permutations in
    // a different order than what is listed here.
    let mut permutations: Vec<String> = Vec::new();
    if sequence.len() == 1 {
        permutations.push(sequence);
        return permutations;
    }

    let first_char = sequence.chars().next().unwrap();
    let remaining_chars = sequence.chars().skip(1).collect::<String>();
    let remaining_permutations = get_permutations(remaining_chars);
    for permutation in remaining_permutations {
        for i in 0..permutation.len() + 1 {
            let mut new_permutation = permutation.clone();
            new_permutation.insert(i, first_char);
            permutations.push(new_permutation);
        }
    }
    permutations
}

#[cfg(test)]
#[test]
fn test_get_permutations() {
    let mut permutations = get_permutations("abc".to_string());
    permutations.sort();
    assert_eq!(permutations, vec!["abc", "acb", "bac", "bca", "cab", "cba"]);
    permutations = get_permutations("a".to_string());
    permutations.sort();
    assert_eq!(permutations, vec!["a"]);

    permutations = get_permutations("ab".to_string());
    permutations.sort();
    assert_eq!(permutations, vec!["ab", "ba"]);

    permutations = get_permutations("abcde".to_string());
    permutations.sort();
    assert_eq!(
        permutations,
        vec![
            "abcde", "abced", "abdce", "abdec", "abecd", "abedc", "acbde", "acbed", "acdbe",
            "acdeb", "acebd", "acedb", "adbce", "adbec", "adcbe", "adceb", "adebc", "adecb",
            "aebcd", "aebdc", "aecbd", "aecdb", "aedbc", "aedcb", "bacde", "baced", "badce",
            "badec", "baecd", "baedc", "bcade", "bcaed", "bcdae", "bcdea", "bcead", "bceda",
            "bdace", "bdaec", "bdcae", "bdcea", "bdeac", "bdeca", "beacd", "beadc", "becad",
            "becda", "bedac", "bedca", "cabde", "cabed", "cadbe", "cadeb", "caebd", "caedb",
            "cbade", "cbaed", "cbdae", "cbdea", "cbead", "cbeda", "cdabe", "cdaeb", "cdbae",
            "cdbea", "cdeab", "cdeba", "ceabd", "ceadb", "cebad", "cebda", "cedab", "cedba",
            "dabce", "dabec", "dacbe", "daceb", "daebc", "daecb", "dbace", "dbaec", "dbcae",
            "dbcea", "dbeac", "dbeca", "dcabe", "dcaeb", "dcbae", "dcbea", "dceab", "dceba",
            "deabc", "deacb", "debac", "debca", "decab", "decba", "eabcd", "eabdc", "eacbd",
            "eacdb", "eadbc", "eadcb", "ebacd", "ebadc", "ebcad", "ebcda", "ebdac", "ebdca",
            "ecabd", "ecadb", "ecbad", "ecbda", "ecdab", "ecdba", "edabc", "edacb", "edbac",
            "edbca", "edcab", "edcba"
        ]
    );
}
