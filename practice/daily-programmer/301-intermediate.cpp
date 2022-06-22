/**
 * PROMPT:
 * You will be given a sequence of letters and you must match with a
 * dictionary. The sequence is a pattern of letters that you must find.
 *
 * Eg.
 * Pattern: XXYY
 * Means that you have a word that contains a sequence of 2 of the same letters
 * followed by a different set of two of the same letters
 *
 * @author Carson Boden
 */

#include "../resources/dictionary.h"

#include <algorithm>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <vector>

bool pattern_matches(const std::string& format, const std::string& word);
bool compare_part(const std::string& format, std::string& part);

int main(int argc, char* argv[])
{
  std::vector<std::string> words = load_dictionary();

  for (auto& word : words) {
    if (pattern_matches(argv[1], word)) {
      std::cout << word << std::endl;
    }
  }

  return 0;
}

bool pattern_matches(const std::string& format, const std::string& word)
{
  if (format.length() > word.length()) {
    return false;
  }

  for (int i = 0; i < word.length(); ++i) {
    std::string part = word.substr(i, format.length());

    if (part.length() < format.length()) {
      break;
    }
    if (compare_part(format, part)) {
      return true;
    }
  }

  return false;
}

bool compare_part(const std::string& format, std::string& part)
{
  std::set<char> found_chars;
  for (int i = 0; i < format.length(); ++i) {
    if (found_chars.find(format[i]) == found_chars.end()) {
      char to_replace = static_cast<char>(part[i]);
      char replace_with = static_cast<char>(format[i]);
      std::replace(part.begin(), part.end(), to_replace, replace_with);
      found_chars.insert(format[i]);
    }
  }

  return format == part;
}
