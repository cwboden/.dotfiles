/**
 * PROMPT:
 * Given an array of integers, return indices of the two numbers such that they
 *  add up to a specific target.
 * You may assume that each input would have exactly one solution, and you may
 *  not use the same element twice.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <map>
#include <vector>

std::vector<int> two_sum(std::vector<int>& nums, int target)
{
  std::map<int, int> inverse_to_index;

  for (int i = 0; i < nums.size(); ++i) {
    if (inverse_to_index.find(nums[i]) != inverse_to_index.end()) {
      return { inverse_to_index[nums[i]], i };
    }

    inverse_to_index[target - nums[i]] = i;
  }

  return std::vector<int>();
}

int main(int argc, char* argv[])
{
  std::vector<int> nums = { 2, 3, 5, 7, 11, 15 };

  std::vector<int> result = two_sum(nums, 9);

  std::cout << "Result: ";
  for (int num : result) {
    std::cout << num << " ";
  }
  std::cout << std::endl;

  return 0;
}
