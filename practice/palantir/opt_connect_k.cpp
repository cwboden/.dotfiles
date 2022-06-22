/**
 * PROMPT:
 * Optimize the 'win_found' function of a m-by-n-dimensional connect k game for
 * time complexity with no constraints on memory complexity (beyond n^2)
 *
 * @author Carson Boden
 */

#include <iostream>
#include <map>
#include <tuple>
#include <vector>

class ConnectK
{
 public:
  static const int kNumPlayers = 2;

  ConnectK(int width, int height, int connections_to_win) :
    width_(width), height_(height), connections_to_win_(connections_to_win)
  {
    for (int i = 0; i < kNumPlayers; ++i) {
      row_lengths.push_back(std::map<std::tuple<int, int>, int>());
      diag_lengths.push_back(std::map<std::tuple<int, int>, int>());

      player_col_usage.push_back(std::vector<int>(width, 0));
    }

    col_heights = std::vector<int>(width, 0);
  }

  bool WinFound(int player, int col)
  {
    // Check if column has a winner
    player_col_usage.at(player).at(col)++;
    if (player_col_usage.at(player).at(col) == connections_to_win_) {
      std::cout << "Vertical Victory for Player " << player << std::endl;
      return true;
    }

    // Reset column of another player
    int other_player = (player + 1) % 2;
    if (player_col_usage.at(other_player).at(col) != 0) {
      player_col_usage.at(other_player).at(col) = 0;
    }

    // Insert piece into row
    int row = col_heights.at(col);
    col_heights.at(col)++;

    // Check adjacent squares in row
    auto map = &row_lengths.at(player);
    int right_extension = 0;
    int left_extension = 0;

    std::tuple<int, int> right_coordinate(row + 1, col);
    std::tuple<int, int> left_coordinate(row - 1, col);

    // Find if segments on left or right
    if ((row + 1 < width_) &&
        map->find(right_coordinate) != map->end()) {
      right_extension = map->at(right_coordinate);
    }
    if ((row - 1 > 0) &&
        map->find(left_coordinate) != map->end()) {
      left_extension = map->at(left_coordinate);
    }

    // Check if new segment results in a win
    int new_length = left_extension + right_extension + 1;
    if (new_length > connections_to_win_) {
      std::cout << "Horizontal Victory for Player " << player << std::endl;
      return true;
    }

    // Insert piece into map
    map->operator[](std::tuple<int, int>(row, col)) = new_length;

    // Update edges
    if (right_extension != 0) {
      map->at(std::tuple<int, int>(row + right_extension, col)) = new_length;
    }
    if (left_extension != 0) {
      map->at(std::tuple<int, int>(row - left_extension, col)) = new_length;
    }

    /**
     * Also do the same for diagonal edges
     */

    std::cout << "No winner found yet" << std::endl;
    return false;
  }

 private:
  std::vector<std::map<std::tuple<int, int>, int>> row_lengths;
  std::vector<std::map<std::tuple<int, int>, int>> diag_lengths;

  std::vector<std::vector<int>> player_col_usage;
  std::vector<int> col_heights;

  int width_;
  int height_;
  int connections_to_win_;
};

int main(int argc, char* argv[])
{
  ConnectK board(7, 6, 4);

  board.WinFound(1, 5);
  board.WinFound(1, 5);
  board.WinFound(1, 5);
  board.WinFound(1, 5);

  board.WinFound(0, 0);
  board.WinFound(0, 3);
  board.WinFound(0, 2);
  board.WinFound(0, 1);


  return 0;
}
