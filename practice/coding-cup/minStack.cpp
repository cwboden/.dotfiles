// How would you design a stack which, in addition to push and pop, also has a function
// min which returns the minimum element? Push, pop and min should all operate in
// O(1) time.

class MinStack
{
    Stack mins<int>();

    void push(int elt)
    {
        mins.push(min(elt, mins.top()));

        // ...
    }

    int pop()
    {
        mins.pop();

        // ...
    }

    int top(); // Implemented

    int min()
    {
        return mins.top();
    }
};
