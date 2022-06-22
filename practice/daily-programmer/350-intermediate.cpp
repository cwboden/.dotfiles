/**
 * PROMPT:
 * You'll be given input over two lines. The first line tells you how many
 * distinct values to read in the following line. The next line is sequence of
 * integers showing credits and debits.
 *
 * Your program should emit the positions (0-indexed) where the sum of the sub-
 * sequences before and after the position are the same.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <vector>

int main(int argc, char* argv[])
{
  int num_elements;
  std::cin >> num_elements;

  int sum = 0;
  std::vector<int> transactions;
  for (int i = 0; i < num_elements; ++i) {
    int transaction;
    std::cin >> transaction;

    sum += transaction;
    transactions.push_back(transaction);
  }

  int left = 0;
  int right = sum;
  std::vector<int> equal_indices;
  for (int i = 0; i < transactions.size(); ++i) {
    right -= transactions.at(i);

    if (left == right) {
      equal_indices.push_back(i);
    }

    left += transactions.at(i);
  }

  for (auto& index : equal_indices) {
    std::cout << index << ' ';
  }
  std::cout << std::endl;

  return 0;
}
