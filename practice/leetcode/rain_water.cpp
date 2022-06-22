/**
 * PROMPT:
 * Given n non-negative integers representing an elevation map where the width
 * of each bar is 1, compute how much water it is able to trap after raining.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <vector>

int rain_water(std::vector<int>& heights)
{
  int area = 0;

  std::vector<int> left_max(heights.size());
  std::vector<int> right_max(heights.size());

  left_max.at(0) = heights.at(0);

  for (unsigned int i = 1; i < heights.size(); ++i) {
    left_max.at(i) = std::max(heights.at(i), left_max.at(i - 1));
  }

  right_max.at(right_max.size() - 1) = heights.at(heights.size() - 1);
  for (int i = heights.size() - 2; i >= 0; i--) {
    right_max.at(i) = std::max(heights.at(i), right_max.at(i + 1));
  }

  for (unsigned int i = 1; i < heights.size() - 1; ++i) {
    area += std::min(left_max.at(i), right_max.at(i)) - heights.at(i);
  }

  return area;
}

int main(int argc, char* argv[])
{
  std::vector<int> heights = { 0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1 };

  std::cout << rain_water(heights) << std::endl;

  return 0;
}
