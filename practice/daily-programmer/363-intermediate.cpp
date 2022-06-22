/**
 * PROMPT:
 * You've been taking some classes at a local university. Unfortunately, your
 * theory-of-under-water-basket-weaving professor is really boring. He's also
 * very nosy. In order to pass the time during class, you like sharing notes
 * with your best friend sitting across the aisle. Just in case your professor
 * intercepts any of your notes, you've decided to encrypt them.
 *
 * Specifically, we will be implementing a type of route cipher today. In a
 * route cipher the text you want to encrypt is written out in a grid, and then
 * arranged in a given pattern. The pattern can be as simple or complex as you'd
 * like to make it.
 *
 *
 *
 * @author Carson Boden
 */

#include <locale>
#include <iostream>
#include <set>
#include <sstream>
#include <string>
#include <vector>

std::vector<std::vector<char>> generate_matrix(const std::string& message, int width, int height);
std::string read_encoding_from_matrix(const std::vector<std::vector<char>>& matrix, bool is_clockwise);
void print_matrix(std::vector<std::vector<char>> matrix);

int main(int argc, char* argv[])
{
  std::string message = "We are discovered! Flee at once.";
  std::cout << "Encoding: " << message << std::endl;

  auto matrix = generate_matrix(message, 9, 3);

  print_matrix(matrix);

  std::string output = read_encoding_from_matrix(matrix, true);
  std::cout << output << std::endl;

  return 0;
}

std::vector<std::vector<char>> generate_matrix(const std::string& message, int width, int height)
{
  static const std::set<char> kInvalidChars = { ' ', '.', ',', '!', '?' };
  std::vector<std::vector<char>> matrix;

  int message_index = 0;
  for (int c = 0; c < height; ++c) {
    matrix.push_back(std::vector<char>());
    for (int r = 0; r < width; ++r) {
      // Skip invalid characters
      while (kInvalidChars.find(message[message_index]) != kInvalidChars.end()) {
        message_index++;
      }

      // Add character to matrix
      if (message_index == message.size()) {
        matrix[c].push_back('X');
      }
      else {
        char character = toupper(message[message_index]);
        matrix[c].push_back(character);
        message_index++;
      }
    }
  }

  return matrix;
}

void print_matrix(std::vector<std::vector<char>> matrix)
{
  for (int i = 0; i < matrix.size(); ++i) {
    for (int j = 0; j < matrix[i].size(); ++j) {
      std::cout << matrix[i][j] << ' ';
    }
    std::cout << std::endl;
  }
}

std::string read_encoding_from_matrix(const std::vector<std::vector<char>>& matrix, bool is_clockwise)
{
  enum Direction { kUp, kRight, kDown, kLeft };

  std::stringstream string_builder;
  Direction current_direction;
  int x_pos = matrix.front().size() - 1;
  int y_pos = 0;

  int top = 0;
  int bottom = matrix.size() - 1;
  int left = 0;
  int right = matrix.front().size() - 1;

  if (is_clockwise) {
    current_direction = kDown;
  } else {
    current_direction = kLeft;
  }

  while(top <= bottom && left <= right) {

    string_builder << matrix[y_pos][x_pos];

    switch (current_direction) {
    case kUp:
      y_pos--;
      if (y_pos == top) {
        if (is_clockwise) {
          current_direction = kRight;
          left++;
        } else {
          current_direction = kLeft;
          right--;
        }
      }
      break;

    case kRight:
      x_pos++;
      if (x_pos == right) {
        if (is_clockwise) {
          current_direction = kDown;
          top++;
        } else {
          current_direction = kUp;
          bottom--;
        }
      }
      break;

    case kDown:
      y_pos++;
      if (y_pos == bottom) {
        if (is_clockwise) {
          current_direction = kLeft;
          right--;
        } else {
          current_direction = kRight;
          left++;
        }
      }
      break;

    case kLeft:
      x_pos--;
      if (x_pos == left) {
        if (is_clockwise) {
          current_direction = kUp;
          bottom--;
        } else {
          current_direction = kDown;
          top++;
        }
      }
      break;
    }
  }

  string_builder << matrix[y_pos][x_pos];
  return string_builder.str();
}
