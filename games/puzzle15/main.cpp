// Carson Boden / December 2016
// The 15 Number Puzzle Game

#include <algorithm>
#include <vector>
#include <iostream>
#include <random>

class Puzzle
{
public:
	Puzzle()
	{
		for (int i = 1; i < 10; ++i)
		{

			board.push_back(i + '0');
		} // for

		for (int i = 0; i < 6; ++i)
		{
			board.push_back(i + 'A');
		} // for

		board.push_back(' ');

		shuffleBoard();
	} // Puzzle()

	void playGame()
	{
		while (!isSolved())
		{
			std::cout << "Enter move: ";
			char move;

			std::cin >> move;

			switch (move)
			{
			case 'u':
				if (blankPos > 3)
				{
					board[blankPos] = board[blankPos - 4];
					blankPos -= 4;
					board[blankPos] = ' ';
				}
				break;
			case 'd':
				if (blankPos < 12)
				{
					board[blankPos] = board[blankPos + 4];
					blankPos += 4;
					board[blankPos] = ' ';
				}
				break;
			case 'l':
				if (blankPos % 4 != 0)
				{
					board[blankPos] = board[blankPos - 1];
					blankPos -= 1;
					board[blankPos] = ' ';
				}
				break;
			case 'r':
				if (blankPos % 4 != 3)
				{
					board[blankPos] = board[blankPos + 1];
					blankPos += 1;
					board[blankPos] = ' ';
				}
				break;
			case 'q':
				return;
			default:
				std::cout << "Invalid move " << move << std::endl;
				break;
			}

			printBoard();

		}
	} // playGame()

	void printBoard()
	{
		for (int r = 0; r < 4; ++r)
		{
			std::cout << "+-----+-----+-----+-----+\n";
			for (int c = 0; c < 4; ++c)
			{
				std::cout << "|  " << board[(4 * r) + c] << "  ";
			} // for
			std::cout << "|\n";
		} // for

		std::cout << "+-----+-----+-----+-----+\n";
	} // printBoard()

private:
	int blankPos;
	std::vector<char> board;

	bool nextInSequence(char a, char b)
	{
		return (b - a == 1) || (a == '9' && b == 'A');
	}

	bool isSolved()
	{
		for (int i = 1; i < board.size(); ++i)
		{
			if (!nextInSequence(board[i - 1], board[i]))
			{
				return false;
			}
		}
		return true;
	}

	void shuffleBoard()
	{
		for (int i = 0; i < 16; ++i)
		{
			char temp = board[i];
			board[i] = board[i*i % 16];
			board[i*i % 16] = temp;
		} // for

		blankPos = std::find(board.begin(), board.end(), ' ') - board.begin();
	}

}; // Puzzle

int main()
{
	Puzzle puzzle;
	puzzle.printBoard();
	puzzle.playGame();

	return 0;
} // main()
