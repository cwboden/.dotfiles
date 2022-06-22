/**
 * PROMPT:
 * A Ducci sequence is a sequence of n-tuples of integers, sometimes known as
 * "the Diffy game", because it is based on sequences. Given an n-tuple of
 * integers (a_1, a_2, ... a_n) the next n-tuple in the sequence is formed by
 * taking the absolute differences of neighboring integers. Ducci sequences are
 * named after Enrico Ducci (1864-1940), the Italian mathematician credited
 * with their discovery.
 *
 * Some Ducci sequences descend to all zeroes or a repeating sequence. An
 * example is (1,2,1,2,1,0) -> (1,1,1,1,1,1) -> (0,0,0,0,0,0).
 *
 * You'll be given an n-tuple, one per line. Your program should emit the
 * number of steps taken to get to either an all 0 tuple or when it enters a
 * stable repeating pattern.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <cmath>
#include <set>
#include <vector>

void find_ducci_sequence(std::vector<int> ducci);
std::vector<int> calculate_next_ducci(std::vector<int> current_ducci);

void print_ducci(std::vector<int> ducci);

int main(int argc, char* argv[])
{
  std::vector<int> standard_input = { 0, 653, 1854, 4063 };
  std::vector<int> challenge_input_1 = { 1, 5, 7, 9, 9 };
  std::vector<int> challenge_input_2 = { 1, 2, 1, 2, 1, 0 };
  std::vector<int> challenge_input_3 = { 10, 12, 41, 62, 31, 50 };
  std::vector<int> challenge_input_4 = { 10, 12, 41, 62, 31 };

  find_ducci_sequence(standard_input);

  return 0;
}

void find_ducci_sequence(std::vector<int> ducci)
{
  std::set<std::vector<int>> seen_duccis;

  // Add zero ducci to set
  std::vector<int> zero_ducci;
  for (int i = 0; i < ducci.size(); ++i) {
    zero_ducci.push_back(0);
  }
  seen_duccis.insert(zero_ducci);

  print_ducci(ducci);

  while (seen_duccis.find(ducci) == seen_duccis.end()) {
    seen_duccis.insert(ducci);
    ducci = calculate_next_ducci(ducci);
    print_ducci(ducci);
  }
}

void print_ducci(std::vector<int> ducci)
{
  for (auto& number : ducci) {
    std::cout << number << '\t';
  }
  std::cout << std::endl;
}

std::vector<int> calculate_next_ducci(std::vector<int> current_ducci)
{
  // Check for zero-element ducci
  if (current_ducci.size() < 1) {
    return current_ducci;
  }

  std::vector<int> next_ducci;

  // Calculate elements
  for (int i = 0; i < current_ducci.size() - 1; ++i) {
    int diff = abs(current_ducci[i] - current_ducci[i + 1]);
    next_ducci.push_back(diff);
  }

  // Calculate last element
  int diff = abs(current_ducci.back() - current_ducci.front());
  next_ducci.push_back(diff);

  return next_ducci;
}
