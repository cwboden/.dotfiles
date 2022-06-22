/**
 * PROMPT:
 * You must remotely send a sequence of orders to a robot to get it out of a
 * minefield. You win the game when the order sequence allows the robot to get
 * out of the minefield without touching any mine. Otherwise it returns the
 * position of the mine that destroyed it.
 *
 * The mines are represented by *, the robot by R, the exit by E, and walls by +
 *
 * The orders understandable by the robot are as follows:
 *  N: Move the robot one square north
 *  E: Move the robot one square east
 *  S: Move the robot one square south
 *  W: Move the robot one square west
 *  I: Start the engine of the robot
 *  -: Cuts the engine of the robot
 *
 * If the robot tries to move to a square occupied by a wall (+) then it stays
 * in place. If the robot is not started, then the commands are inoperative. It
 * is possible to start and stop it as many times as desired.
 *
 * When the robot reaches the exit, it is necessary to stop to win the game.
 *
 * @author Carson Boden
 */

#include <iostream>
#include <fstream>
#include <string>
#include <vector>

class MineGame
{
 public:
  MineGame()
  {
    _mine_field = std::vector<std::vector<char>>();
    _is_robot_on = false;
    _is_playing = true;
    _y_pos = 0;
    _x_pos = 0;
  }

  void ViewMenu()
  {
    std::cout << "Welcome to MineMover!" << std::endl;
    std::cout << "Please enter a minefield file: ";

    std::string file_path;
    std::cin >> file_path;

    std::fstream file_stream(file_path);

    int height, width;
    file_stream >> width >> height;

    char value;
    for (int i = 0; i < height; ++i) {
      _mine_field.push_back(std::vector<char>());
      for (int j = 0; j < width; ++j) {
        file_stream >> value;

        // Sets robot position
        if (value == 'R') {
          _y_pos = i;
          _x_pos = j;
          _mine_field.at(i).push_back('0');
        } else {
          _mine_field.at(i).push_back(value);
        }
      }
    }
  }

  void PlayGame()
  {
    char input;
    while (_is_playing) {
      PrintMap();
      std::cout << "Enter your move: ";
      std::cin >> input;

      HandleInput(input);
    }
  }

  void PrintMap()
  {
    for (int i = 0; i < _mine_field.size(); ++i) {
      for (int j = 0; j < _mine_field[i].size(); ++j) {
        if (_x_pos == j && _y_pos == i) {
          std::cout << 'R';
        } else {
          std::cout << _mine_field[i][j];
        }
      }
      std::cout << std::endl;
    }
  }

  void HandleInput(char input)
  {
    switch(input) {
      case 'N':
        Move(_x_pos, _y_pos - 1);
        break;
      case 'E':
        Move(_x_pos + 1, _y_pos);
        break;
      case 'S':
        Move(_x_pos, _y_pos + 1);
        break;
      case 'W':
        Move(_x_pos - 1, _y_pos);
        break;
      case 'I':
        _is_robot_on = true;
        break;
      case '-':
        _is_robot_on = false;
        if (_mine_field.at(_y_pos).at(_x_pos) == 'E') {
          std::cout << "You Win!" << std::endl;
          _is_playing = false;
        }
        break;
      default:
        std::cout << "Invalid input. Try again with one of <NESWI->." << std::endl;
        break;
    }
  }

  void Move(int x, int y)
  {
    // If bot should not move due to user error, don't move
    if (x < 0 || y < 0 ||
        _mine_field.size() < y || _mine_field[y].size() < x ||
        !_is_robot_on) {
      return;
    }

    switch (_mine_field[y][x]) {
      // Check for wall, don't move if wall
      case '+':
        return;

      // Check for mine, end game if so
      case '*':
        std::cout << "Mine Hit at (" << x << ", " << y << ")!" << std::endl;
        _is_playing = false;
        return;

      // Otherwise, set new position of robot
      default:
        _y_pos = y;
        _x_pos = x;
        break;
    }
  }

 private:
  std::vector<std::vector<char>> _mine_field;
  int _x_pos, _y_pos;
  bool _is_robot_on;
  bool _is_playing;
};

int main(int argc, char* argv[])
{
  MineGame game;
  game.ViewMenu();
  game.PlayGame();

  return 0;
}
