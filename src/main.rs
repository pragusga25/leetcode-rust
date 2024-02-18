mod easy;
mod medium;
fn main() {
    easy::valid_anagram::run_tests();
    easy::contains_duplicate::run_tests();
    easy::two_sum::run_tests();
    easy::valid_parentheses::run_tests();

    medium::group_anagrams::run_tests();
    medium::top_k_frequent::run_tests();
    medium::product_except_self::run_tests();
    medium::is_valid_sudoku::run_tests();
    medium::longest_consecutive::run_tests();
    medium::min_stack::run_tests();
    medium::eval_rpn::run_tests();
    medium::generate_parenthesis::run_tests();
}
