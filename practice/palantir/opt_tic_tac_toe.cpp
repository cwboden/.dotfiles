/**
 * PROMPT:
 * Optimize the 'win_found' function of a k-dimensional tic-tac-toe game to maximize runtime
 * complexity with no constrains on memory complexity (beyond n^2)
 *
 * @author Carson Boden
 */

#include <iostream>
#include <vector>

class TicTacToe
{
 public:
  static const int kNumPlayers = 2;

  TicTacToe(int size) : size_(size)
  {
    for (int i = 0; i < kNumPlayers; ++i) {
      player_row_usage.push_back(std::vector<int>());
      player_col_usage.push_back(std::vector<int>());
      player_diag_usage.push_back(std::vector<int>());

      for (int j = 0; j < size_; ++j) {
        player_row_usage.at(i).push_back(0);
        player_col_usage.at(i).push_back(0);
      }

      // Two diagonals possible
      for (int j = 0; j < 2; ++j) {
        player_diag_usage.at(i).push_back(0);
      }
    }

  }

  bool WinFound(int player, int row, int col)
  {
    player_row_usage.at(player).at(row)++;
    player_col_usage.at(player).at(col)++;

    // Check for diagonals
    if (row == col) {
      player_diag_usage.at(player).at(0)++;
    }
    if (row == (size_ - col - 1)) {
      player_diag_usage.at(player).at(1)++;
    }

    if (player_row_usage.at(player).at(row) == size_ ||
        player_col_usage.at(player).at(col) == size_ ||
        player_diag_usage.at(player).at(0) == size_ ||
        player_diag_usage.at(player).at(1) == size_) {

      std::cout << "Winner found" << std::endl;
      return true;
    }

    std::cout << "Winner not found" << std::endl;
    return false;
  }

 private:
  std::vector<std::vector<int>> player_row_usage;
  std::vector<std::vector<int>> player_col_usage;
  std::vector<std::vector<int>> player_diag_usage;
  int size_;
};

int main(int argc, char* argv[])
{
  TicTacToe board(3);
  board.WinFound(0, 1, 1);
  board.WinFound(1, 0, 1);
  board.WinFound(0, 1, 2);
  board.WinFound(1, 0, 2);
  board.WinFound(0, 2, 0);
  board.WinFound(1, 2, 1);
  board.WinFound(0, 1, 0);

  std::cout << std::endl << "NEW BOARD" << std::endl;
  // Test diagonals
  TicTacToe board2(5);
  board2.WinFound(1, 0, 0);
  board2.WinFound(1, 1, 1);
  board2.WinFound(1, 2, 2);
  board2.WinFound(1, 3, 3);
  board2.WinFound(1, 4, 4);
  board2.WinFound(0, 0, 4);
  board2.WinFound(0, 1, 3);
  board2.WinFound(0, 2, 2);
  board2.WinFound(0, 3, 1);
  board2.WinFound(0, 4, 0);

  return 0;
}
