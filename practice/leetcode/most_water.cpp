/**
 * PROMPT:
 * Given n non-negative integers a1, a2, ..., an , where each represents a
 * point at coordinate (i, ai). n vertical lines are drawn such that the two
 * endpoints of line i is at (i, ai) and (i, 0). Find two lines, which together
 * with x-axis forms a container, such that the container contains the most
 * water.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <vector>

int max_area(std::vector<int>& heights)
{
  int max = 0;

  int close_idx = 0;
  int far_idx = heights.size() - 1;

  while (close_idx < far_idx) {
    int area = (far_idx - close_idx) * std::min(heights.at(close_idx), heights.at(far_idx));

    if (max < area) {
      max = area;
    }

    if (heights.at(close_idx) < heights.at(far_idx)) {
      close_idx++;
    }
    else {
      far_idx--;
    }
  }

  return max;
}

int main(int argc, char* argv[])
{
  std::vector<int> heights = { 1, 8, 6, 2, 5, 4, 8, 3, 7 };
  std::cout << max_area(heights) << std::endl;

  return 0;
}
