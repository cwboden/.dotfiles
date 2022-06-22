/**
 * PROMPT:
 * Given a non-empty array of digits representing a non-negative integer, plus
 * one to the integer.
 *
 * The digits are stored such that the most significant digit is at the head of
 * the list, and each element in the array contain a single digit.
 *
 * You may assume the integer does not contain any leading zero, except the
 * number 0 itself.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <vector>

std::vector<int> plus_one(std::vector<int> num)
{
  std::reverse(num.begin(), num.end());

  for (unsigned int i = 0; i < num.size(); ++i) {
    if (num.at(i) != 9) {
      num.at(i) += 1;
      break;
    }

    num.at(i) = 0;
  }

  if (num.back() == 0) {
    num.push_back(1);
  }

  std::reverse(num.begin(), num.end());
  return num;
}

void print_vector(std::vector<int>& num)
{
  for (int n : num) {
    std::cout << n;
  }

  std::cout << std::endl;
}

int main(int argc, char* argv[])
{
  auto result = plus_one({ 1, 2, 3 });
  print_vector(result);

  result = plus_one({ 4, 3, 2, 1 });
  print_vector(result);

  result = plus_one({ 1, 9 });
  print_vector(result);

  result = plus_one({ 4, 9, 9 });
  print_vector(result);

  result = plus_one({ 9, 9, 9, 9, 9 });
  print_vector(result);

  return 0;
}
