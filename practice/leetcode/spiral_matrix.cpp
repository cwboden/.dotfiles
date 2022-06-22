/**
 * PROMPT:
 * Given a matrix of m x n elements (m rows, n columns), return all elements of
 * the matrix in spiral order.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <vector>

std::vector<int> spiral_matrix(std::vector<std::vector<int>>& matrix)
{
  std::vector<int> results;

  unsigned int x_front = 0, y_front = 0;
  unsigned int x_back = matrix.at(0).size() - 1, y_back = matrix.size() - 1;

  while (x_front <= x_back && y_front <= y_back) {

    for (unsigned int i = x_front; i < x_back; ++i) {
      results.push_back(matrix.at(y_front).at(i));
    }
    for (unsigned int i = y_front; i < y_back; ++i) {
      results.push_back(matrix.at(i).at(x_back));
    }
    for (unsigned int i = x_back; i > x_front; i--) {
      results.push_back(matrix.at(y_back).at(i));
    }
    for (unsigned int i = y_back; i > y_front; i--) {
      results.push_back(matrix.at(i).at(x_front));
    }

    x_front++;
    y_front++;
    x_back--;
    y_back--;
  }

  return results;
}

void print_vector(std::vector<int>& nums) {
  for (int num : nums) {
    std::cout << num << ' ';
  }
  std::cout << std::endl;
}

int main(int argc, char* argv[])
{
  std::vector<std::vector<int>> matrix = {
    { 1, 2, 3, 4 },
    { 5, 6, 7, 8 },
    { 9, 10, 11, 12 },
    { 13, 14, 15, 16 }
  };

  auto result = spiral_matrix(matrix);
  print_vector(result);

  matrix = {
    { 1, 2, 3, 4, 5, 6, 7, 8 },
    { 9, 10, 11, 12, 13, 14, 15, 16 }
  };

  result = spiral_matrix(matrix);
  print_vector(result);

  return 0;
}
