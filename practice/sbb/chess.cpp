// Carson Boden / November 2016
// Chess classes


enum pieceType { Blank, Pawn, Bishop, Knight, Rook, Queen, King };
enum side { Blank, Black, White };

class Piece
{
public:
	// Default piece (A blank square)
	Piece()
	{
		_type = Blank;
		_side = Blank;
	} // Piece()

	// Constructor for a piece with a side
	Piece(pieceType typeIn, side sideIn)
	{
		_type = typeIn;
		_side = sideIn;
	} // Piece()

	// Returns the type
	pieceType getType()
	{
		return _type;
	} // getType()

private:
	pieceType _type;
	side _side;
};

class Board
{
public:
	// Default board with 16 pieces on each side
	Board()
	{
		for (size_t r = 0; r < 8; ++r)
		{
			for (size_t c = 0; c < 8; ++c)
			{
				// TODO: Initialize pieces on the board
			}
		}
	} // Board()

	void movePiece(pair<int, int> pieceIndex, pair<int, int> target)
	{
		// If the piece is moved to a valid target and is not blocked by other pieces, then it is moved
		if (_isValidMove(pieceIndex, target) && _isNotBlocked(piece, target))
		{
			_board[target.first][target.second] = _board[pieceIndex.first][pieceIndex.second];
			_board[pieceIndex.first][pieceIndex.second] = Piece();
		}
	} // movePiece()

private:
	vector<vector<Piece> > _board;

	// Checks that a piece is actually moving to a valid location for its type
	// E.g, a rook only moves in the cardinal directions
	bool _isValidMove(pair<int, int> pieceIndex, pair<int, int> target)
	{
		switch (_board[pieceIndex.first][pieceIndex.second].getType())
		{
		case (Blank):
			return false;
		case (Pawn):
			// TODO: Check that target is forward, cardinally or diagonally
			return false;
		case (Bishop):
			// TODO: Check target is diagonal
			return false;
		case (Knight):
			// TODO: Check target is an L shape away
			return false;
		case (Rook):
			// TODO: Check target is cardinal
			return false;
		case (Queen):
			// TODO: Check target is diagonal or cardinal
			return false;
		case (King):
			// TODO: Check target is only one square away
			return false;
		} // switch
	} // _isValidMove()

	// Checks that a piece can move to a specific square without interfering with other pieces
	// Requires: Piece should already be moving to a valid location
	bool _isNotBlocked(pair<int, int> pieceIndex, pair<int, int> target)
	{
		switch (_board[pieceIndex.first][pieceIndex.second].getType())
		{
		case (Blank):
			return false;
		case (Pawn):
			// Pawns can attack diagonally, but not straight forward
			return (pieceIndex.first != target.first - 1) &&
				(pieceIndex.first != target.first + 1);
		case (Bishop):
			// TODO: Check diagonal squares
			return false;
		case (Knight):
			// Knights are never blocked
			return true;
		case (Rook):
			// TODO: Check cardinal squares
			return false;
		case (Queen):
			// TODO: Check diagonal and cardinal squares
			return false;
		case (King):
			// TODO: Check that piece does not move into Check
			return false;
		} // switch
	} // _isNotBlocked()

	// Checks if the tile is blank
	bool _isEmptySquare(pair<int, int> square)
	{
		return _board[square.first][square.second].getType() == Blank;
	} // _isEmptySquare()
};
