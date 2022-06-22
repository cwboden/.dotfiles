/**
 * PROMPT:
 * Given an array of strings, group anagrams together.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <map>
#include <string>
#include <vector>

std::vector<std::vector<std::string>> group_anagrams(std::vector<std::string>& strings)
{
  if (strings.size() == 0) return std::vector<std::vector<std::string>>();

  std::map<std::string, std::vector<std::string>> word_to_list_map;

  for (std::string str : strings) {
    std::string sorted_str = str;
    std::sort(sorted_str.begin(), sorted_str.end());
    word_to_list_map[sorted_str].push_back(str);
  }

  std::vector<std::vector<std::string>> results;
  for (auto it = word_to_list_map.begin(); it != word_to_list_map.end(); it++) {
    results.push_back(it->second);
  }

  return results;
}

int main(int argc, char* argv[])
{
  std::vector<std::string> input = {
    "eat", "tea", "tan", "ate", "nat", "bat"
  };

  auto results = group_anagrams(input);

  for (int i = 0; i < results.size(); ++i) {
    for (int j = 0; j < results.at(i).size(); ++j) {
      std::cout << results.at(i).at(j) << " ";
    }
    std::cout << std::endl;
  }

  return 0;
}
