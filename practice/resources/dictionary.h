/**
 * @file: dictionary.h
 *
 * Returns a vector of words from the dictionary
 *
 * @author Carson Boden
 */

#include <fstream>
#include <string>
#include <vector>

std::vector<std::string> load_dictionary()
{
  std::vector<std::string> words;
  std::ifstream file_stream("../resources/enable1.txt");
  std::string line;

  while (getline(file_stream, line)) {
    words.push_back(line);
  }

  return words;
}
