/**
 * PROMPT:
 * Given a string, find the length of the longest substring without repeating
 * characters.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <map>
#include <string>

int longest_substring(std::string str)
{
  int result = 0;
  std::map<char, int> char_to_index;

  for (int i = 0, j = 0; j < str.size(); ++j) {
    if (char_to_index.find(str[j]) != char_to_index.end()) {
      i = std::max(i, char_to_index.at(str[j]));
    }

    result = std::max(result, j - i + 1);
    char_to_index[str[j]] = j + 1;
  }

  return result;
}

int main(int argc, char* argv[])
{
  std::cout << longest_substring("abcabcbb") << std::endl;
  std::cout << longest_substring("abcdefghijklmnopqrstuvwxyza") << std::endl;
  std::cout << longest_substring("bbbbbbbb") << std::endl;
  std::cout << longest_substring("pwwkew") << std::endl;
  std::cout << longest_substring("okeydokey") << std::endl;

  return 0;
}
