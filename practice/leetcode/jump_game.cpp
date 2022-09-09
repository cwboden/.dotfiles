/**
 * PROMPT:
 * Given an array of non-negative integers, you are initially positioned at the
 * first index of the array.
 *
 * Each element in the array represents your maximum jump length at that
 * position.
 *
 * Determine if you are able to reach the last index.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <vector>

bool can_jump(std::vector<int> nums)
{
  int last_pos = nums.size() - 1;

  for (int i = last_pos; i >= 0; i--) {
    // If we can reach the last position from the current one
    if (i + nums[i] >= last_pos) {
      last_pos = i;
    }
  }

  return last_pos == 0;
}

int main(int argc, char* argv[])
{
  std::cout << (can_jump({ 3, 2, 1, 0, 4 }) ? "true" : "false") << std::endl;
  std::cout << (can_jump({ 3, 2, 1, 4 }) ? "true" : "false") << std::endl;
  std::cout << (can_jump({ 3, 2, 1, 2, 3, 4, 1, 0, 0, 1, 4 }) ? "true" : "false") << std::endl;
  std::cout << (can_jump({ 5, 3, 2, 1, 0, 1, 4 }) ? "true" : "false") << std::endl;

  return 0;
}
