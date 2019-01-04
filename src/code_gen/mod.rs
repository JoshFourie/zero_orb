pub mod comparator;

// DO NOT USE COMPARATOR EXCEPT FOR TESTING, POTENTIAL VULNERABILITY W.R.T. TWO'S COMPLEMENT BEING INVESTIGATED,
// FOR COMPARISON, ASSERT 0 < A < B AS A SUBSTITUTE TO AVOID A BEING > BUT BEING PARSED AS < BECAUSE OF TWO'S COMPLEMENT.

// todo: improve testing to check that certain lines exist in the generated files.
#[test]
fn test_comparator_code_gen() {
    use std::path::Path;
    comparator::new("64 COMP", Path::new("src/tests/files/code_gen/comparator_64.zk"));
    comparator::new("32 COMP", Path::new("src/tests/files/code_gen/comparator_32.zk"));
    comparator::new("16 COMP", Path::new("src/tests/files/code_gen/comparator_16.zk"));
    comparator::new("8 COMP", Path::new("src/tests/files/code_gen/comparator_8.zk"));
    comparator::new("64 RANGE", Path::new("src/tests/files/code_gen/range_64.zk"));
    comparator::new("32 RANGE", Path::new("src/tests/files/code_gen/range_32.zk"));
    comparator::new("16 RANGE", Path::new("src/tests/files/code_gen/range_16.zk"));
    comparator::new("8 RANGE", Path::new("src/tests/files/code_gen/range_8.zk"));
}
