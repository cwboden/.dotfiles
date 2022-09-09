// Given a circular linked list, implement an algorithm which returns node at the beginning
// of the loop.

struct Node
{
    Node* next;
    int datum;
};

Node* findLinkedLoop(Node* head)
{
    Node* slow = head;
    Node* fast = head;

    // Determines that a loop exists
    while(fast->next != nullptr)
    {
        if (slow == fast)
        {
            break;
        }

        slow = slow->next;
        fast = fast->next->next;
    }

    // If loop not actually present
    if (fast->next == nullptr)
    {
        return nullptr;
    }

    // Resets slow pointer
    slow = head;
    while (slow != fast)
    {
        slow = slow->next;
        fast = fast->next;
    }

    return fast;
}
