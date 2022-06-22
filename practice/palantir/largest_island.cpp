/**
 * PROMPT:
 * Given a 2d matrix of positive integer values or zero, find the area of the
 * largest island where the area of an island is the sum of adjacent integers
 * that are not zero
 *
 * Challenge: Design for a k-dimensional matrix
 *
 * @author Carson Boden
 */

#include <iostream>
#include <queue>
#include <vector>

struct Coordinate
{
  int row;
  int col;

  Coordinate(int r, int c) : row(r), col(c) { }
};

int find_largest_island(std::vector<std::vector<int>>& matrix)
{
  // Fill search matrix with false
  std::vector<std::vector<bool>> is_searched;
  for (int i = 0; i < matrix.size(); ++i) {
    is_searched.push_back(std::vector<bool>());
    for (int j = 0; j < matrix.at(i).size(); ++j) {
      if (matrix.at(i).at(j) == 0) {
        is_searched.at(i).push_back(true);
      } else {
        is_searched.at(i).push_back(false);
      }
    }
  }

  int max_area = 0;
  std::queue<Coordinate> frontier;
  for (int i = 0; i < matrix.size(); ++i) {
    for (int j = 0; j < matrix.at(i).size(); ++j) {
      if (!is_searched.at(i).at(j)) {
        frontier.push(Coordinate(i, j));

        int new_area = 0;
        while (!frontier.empty()) {
          Coordinate front = frontier.front();
          frontier.pop();

          std::cout << "Examining Matrix[" << front.col << "][" << front.row <<
              "]: " << matrix.at(front.row).at(front.col) << std::endl;

          new_area += matrix.at(front.row).at(front.col);
          is_searched.at(front.row).at(front.col) = true;

          // Push new values into frontier
          for (int i = front.row - 1; i <= front.row + 1; i += 2) {
            if (i >= 0 && i < matrix.size() && !is_searched.at(i).at(front.col)) {
              frontier.push(Coordinate(i, front.col));
            }
          }
          for (int i = front.col - 1; i <= front.col + 1; i += 2) {
            if (i >= 0 && i < matrix.at(front.row).size() && !is_searched.at(front.row).at(i)) {
              frontier.push(Coordinate(front.row, i));
            }
          }
        }

        std::cout << "Calculated area: " << new_area << std::endl;
        if (new_area > max_area) {
          max_area = new_area;
        }
      }
    }
  }

  return max_area;
}

int main(int argc, char* argv[])
{
  std::vector<std::vector<int>> matrix = {
    { 0, 1, 0, 1, 2 },
    { 0, 1, 1, 1, 0 },
    { 3, 0, 0, 0, 1 },
    { 1, 2, 0, 0, 3 },
    { 0, 0, 2, 1, 1 }
  };

  int max_area = find_largest_island(matrix);
  std::cout << "Max Area: " << max_area << std::endl;

  return 0;
}
