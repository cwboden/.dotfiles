/**
 * PROMPT:
 * Given a m x n matrix, if an element is 0, set its entire row and column to 0.
 * Do it in-place.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <vector>

void set_column_row_zeroes(std::vector<std::vector<int>>& matrix, int row_idx, int col_idx)
{
  for (unsigned int i = 0; i < matrix.size(); ++i) {
    matrix.at(row_idx).at(i) = 0;
  }

  for (unsigned int j = 0; j < matrix.at(col_idx).size(); ++j) {
    matrix.at(j).at(col_idx) = 0;
  }
}

void set_matrix_zeroes(std::vector<std::vector<int>>& matrix)
{
  std::vector<int> row_idxs;
  std::vector<int> col_idxs;

  for (unsigned int i = 0; i < matrix.size(); ++i) {
    for (unsigned int j = 0; j < matrix.at(i).size(); ++j) {
      if (matrix.at(i).at(j) == 0) {
        row_idxs.push_back(i);
        col_idxs.push_back(j);
      }
    }
  }

  for (unsigned int i = 0; i < row_idxs.size(); ++i) {
    set_column_row_zeroes(matrix, row_idxs.at(i), col_idxs.at(i));
  }
}

int main(int argc, char* argv[])
{
  std::vector<std::vector<int>> matrix = {
    { 1, 1, 1, 0, 1, 1, 1, 1 },
    { 0, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 0, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 0, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
    { 1, 1, 1, 1, 1, 0, 1, 1 },
    { 1, 1, 1, 1, 1, 1, 1, 1 },
  };

  set_matrix_zeroes(matrix);

  for (unsigned int i = 0; i < matrix.size(); ++i) {
    for (unsigned int j = 0; j < matrix.at(i).size(); ++j) {
      std::cout << matrix.at(i).at(j) << ' ';
    }
    std::cout << std::endl;
  }
  std::cout << std::endl;

  return 0;
}
