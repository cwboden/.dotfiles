// Implement a MyQueue class which implements a queue using two stacks.

class stackQueue
{
    Stack<int> input;
    Stack<int> output;

    void push_back(int elt)
    {
        input.push_back(elt);
    }

    void _flipStack()
    {
        // 'Flip' the input stack onto the output
        while (!input.empty())
        {
            output.push_back(input.pop_back());
        }

        // If no elements present in entire queue
        if (output.empty())
        {
            exit(1);
        }
    }

    int top()
    {
        if (output.empty())
        {
            _flipStack();
        }

        return output.top();
    }

    int pop_back()
    {
        if (output.empty())
        {
            _flipStack();
        }

        return output.pop_back();
    }

    int size()
    {
        return input.size() + output.size();
    }

    bool empty()
    {
        return input.empty() && output.empty();
    }
}
