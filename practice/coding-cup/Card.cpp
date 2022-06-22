class Card
{
private:
    int value;
    Suit suit;

public:
    enum Suit
    {
        CLUBS, SPADES, HEARTS, DIAMONDS
    };

    Card(int v, Suit s) : value(v), suit(s) {}

    int value()
    {
        return value;
    }

    Card::Suit suit()
    {
        return suit;
    }
}
