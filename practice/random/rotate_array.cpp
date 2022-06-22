/**
 * PROMPT:
 * Rotate the elements of an array by k places
 *
 * @author Carson Boden
 */

#include <iostream>
#include <vector>

std::vector<int> rotate(std::vector<int>& arr, int places)
{
  std::vector<int> temp;
  for (int i = 0; i < places; ++i) {
    temp.push_back(arr.at(i));
  }

  std::vector<int> result;
  for (int i = places; i < arr.size(); ++i) {
    result.push_back(arr.at(i));
  }

  result.insert(result.end(), temp.begin(), temp.end());
  return result;
}

void print(std::vector<int>& arr)
{
  for (int num : arr) {
    std::cout << num << ' ';
  }
  std::cout << std::endl;
}

int main(int argc, char* argv[])
{
  std::vector<int> test = { 0, 1, 2, 3, 4, 5, 6, 7 };

  auto result = rotate(test, 4);
  print(result);

  return 0;
}
