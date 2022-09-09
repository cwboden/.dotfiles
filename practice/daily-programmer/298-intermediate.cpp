/**
 * PROMPT:
 * Find the index in string where the first extra ) or ( parentheses
 * is, or return the length of string if parentheses are balanced.
 *
 * @author Carson Boden
 */

#include <deque>
#include <iostream>
#include <string>

int main(int argc, char* argv[])
{
  std::string str = argv[1];
  std::deque<int> checker;

  for (int i = 0; i < str.length(); ++i) {

    if (str[i] == '(') {
      checker.push_back(i);
    }
    else if (str[i] == ')') {
      if (checker.empty()) {
        std::cout << i << std::endl;
        return 0;
      }

      checker.pop_back();
    }
  }

  if (!checker.empty()) {
    std::cout << checker.front() << std::endl;
  }
  else {
    std::cout << str.length() << std::endl;
  }

  return 0;
}
