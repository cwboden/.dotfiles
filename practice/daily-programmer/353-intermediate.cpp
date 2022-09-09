/**
 * PROMPT:
 * I work as a waiter at a local breakfast establishment. The chef at the
 * pancake house is sloppier than I like, and when I deliver the pancakes I
 * want them to be sorted biggest on bottom and smallest on top. Problem is,
 * all I have is a spatula. I can grab substacks of pancakes and flip them over
 * to sort them, but I can't otherwise move them from the middle to the top.
 *
 * You'll be given a pair of lines per input. The first line tells you how many
 * numbers to read in the next line. The second line tells you the pancake sizes
 * as unsigned integers. Read them in order and imagine them describing pancakes
 * of given sizens from the top of the plate to the bottom.
 *
 * Your program should emit the number of spatula flips it took to sort the
 * pancakes from smallest to largest. Optionally show the intermediate steps.
 * Remember, all you have is a spatula that can grab the pancakes from the 0th
 * to the nth position and flip them.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <iterator>
#include <vector>

template<typename Iter>
int pancake_sort(Iter begin, Iter end);
template<typename Iter>
int move_to_end(Iter begin, Iter largest_pancake, Iter end);

int main(int argc, char* argv[])
{
  int num_pancakes;
  std::vector<int> pancakes;

  std::cin >> num_pancakes;
  for (int i = 0; i < num_pancakes; ++i) {
    int pancake;
    std::cin >> pancake;
    pancakes.push_back(pancake);
  }

  int num_flips;
  num_flips = pancake_sort(pancakes.begin(), pancakes.end());

  for (auto& pancake : pancakes) {
    std::cout << pancake << ' ';
  }
  std::cout << std::endl;

  std::cout << "Took " << num_flips << " total flip(s)." << std::endl;

  return 0;
}

template<typename Iter>
int pancake_sort(Iter begin, Iter end)
{
  // Nothing left to flip
  if (std::distance(begin, end) < 2) {
    return 0;
  }

  // Find pancake that needs to be flipped to the bottom
  auto largest_pancake = std::max_element(begin, end);
  int num_flips = 0;

  // Flip largest pancake to the bottom if it isn't there already
  if (largest_pancake != end - 1) {
    num_flips = move_to_end(begin, largest_pancake, end);
  }

  return num_flips + pancake_sort(begin, end - 1);
}

template<typename Iter>
int move_to_end(Iter begin, Iter largest_pancake, Iter end)
{
  // Performing at least one flip
  int num_flips = 1;

  // Flip largest pancake to top, if it isn't there already
  if (largest_pancake != begin) {
    std::reverse(begin, largest_pancake + 1);
    num_flips++;
  }

  // Flip largest pancake to bottom
  std::reverse(begin, end);
  return num_flips;
}
