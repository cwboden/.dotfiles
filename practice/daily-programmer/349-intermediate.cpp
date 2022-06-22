/**
 * PROMPT:
 * You run a moving truck business, and you can pack the most in your truck when
 * you have stacks of equal size - no slack space. So, you're an enterprising
 * person, and you want to write some code to help you along.
 *
 * You'll be given two numbers per line. The first number is the number of
 * stacks of boxes to yield. The second is a list of boxes, one integer per
 * size, to pack.
 *
 * Your program should emit the stack of boxes as a series of integers, one
 * stack per line.
 *
 * @author Carson Boden
 */

#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

void print_boxes(std::vector<int> boxes);

int main(int argc, char* argv[])
{
  int num_stacks;
  std::cin >> num_stacks;

  std::string raw_boxes;
  std::cin >> raw_boxes;

  int sum = 0;
  std::vector<int> boxes;
  for (char& box_char : raw_boxes) {
    int box_int = box_char - '0';
    sum += box_int;
    boxes.push_back(box_int);
  }

  // Check that boxes can be stacked based on height
  if (sum % num_stacks != 0) {
    std::cout << "Boxes cannot be stacked evenly" << std::endl;
    return 0;
  }
  int stack_height = sum / num_stacks;

  // Improves speed with sort and reversal
  std::sort(boxes.begin(), boxes.end());
  std::reverse(boxes.begin(), boxes.end());

  std::vector<int> box_to_stack_index = {0};
  while (true) {
    // Create stacks of boxes
    std::vector<std::vector<int>> stacks;
    for (int i = 0; i < num_stacks; ++i) {
      stacks.push_back(std::vector<int>());
    }

    // Push boxes into the stack
    for (int i = 0; i < box_to_stack_index.size(); ++i) {
      int stack_index = box_to_stack_index.at(i);
      stacks.at(stack_index).push_back(boxes.at(i));
    }

    // Sum most recently pushed box in the stack
    int recent_index = box_to_stack_index.back();
    sum = 0;
    for (auto& box : stacks.at(recent_index)) {
      sum += box;
    }

    // If stack is too large
    if (sum > stack_height) {
      while (box_to_stack_index.back() == num_stacks - 1) {
        box_to_stack_index.pop_back();
      }
      if (box_to_stack_index.back() == box_to_stack_index.size() - 1) {
        std::cout << "Boxes are impossible to stack evenly" << std::endl;
        break;
      }
      box_to_stack_index.back() += 1;
    }

    // Solution found
    else if (box_to_stack_index.size() == boxes.size()) {
      for (auto& stack : stacks) {
        print_boxes(stack);
      }
      break;
    }

    // Solution requires more searching
    else {
      box_to_stack_index.push_back(0);
    }
  }

  return 0;
}

void print_boxes(std::vector<int> boxes)
{
  for (int& box : boxes) {
    std::cout << box << ' ';
  }
  std::cout << std::endl;
}
