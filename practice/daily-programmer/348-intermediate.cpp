/**
 * PROMPT:
 * You will be given a list of integers that represent the number of pins
 * knocked down on each roll. Not that this list is not a fixed size, as bowling
 * a perfect game requires only 12 rolls, while most games would use more rolls.
 *
 * Your program should output the bowling frames including strikes and spares.
 * The total score is not necessary.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <map>
#include <string>
#include <vector>

int main(int argc, char* argv[])
{
  const std::map<int, char> strike_char = {
    { 0, '-' }, { 1, '1' }, { 2, '2' }, { 3, '3' }, { 4, '4' }, { 5, '5' },
    { 6, '6' }, { 7, '7' }, { 8, '8' }, { 9, '9' }, { 10, 'X' }
  };
  int current_frame = 0;

  while (current_frame < 10) {
    int first_roll;
    std::cin >> first_roll;

    bool strike_bonus = false;
    bool spare_bonus = false;

    // On a strike
    if (first_roll == 10) {
      std::cout << 'X';

      if (current_frame == 9) {
        strike_bonus = true;
      } else {
        std::cout << "  ";
      }
    // Otherwise
    } else {
      if (first_roll == 0) {
        std::cout << '-';
      } else {
        std::cout << first_roll;
      }

      int second_roll;
      std::cin >> second_roll;

      // On a spare
      if (first_roll + second_roll == 10) {
        std::cout << '/';

        if (current_frame == 9) {
          spare_bonus = true;
        } else {
          std::cout << ' ';
        }
      // On a miss
      } else if (second_roll == 0) {
        std::cout << "- ";
      // Otherwise
      } else {
        std::cout << second_roll << ' ';
      }
    }

    // If bonus roll for last frame
    if (strike_bonus) {
      int bonus_first;
      std::cin >> bonus_first;

      std::cout << strike_char.at(bonus_first);
      if (bonus_first == 10) {
        std::cin >> bonus_first;
        std::cout << strike_char.at(bonus_first);
      } else {
        int bonus_second;
        std::cin >> bonus_second;
        if (bonus_first + bonus_second == 10) {
          std::cout << '/';
        } else {
          std::cout << bonus_second;
        }
      }

    } else if (spare_bonus) {
      int last_roll;
      std::cin >> last_roll;

      std::cout << strike_char.at(last_roll);
    }

    current_frame++;
  }

  return 0;
}
