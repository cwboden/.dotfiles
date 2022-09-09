// Implement an algorithm to delete a node in the middle of a single linked list, given
// only access to that node.

struct Node
{
    Node* next;
    int datum;
};

// Returns true if node deleted successfully
bool deleteNode(Node* node)
{
    if (node == nullptr || node->next == nullptr)
    {
        // Error - Can't delete last or null node
        return false;
    } // if

    Node* temp = node->next;
    node->datum = temp->datum;
    node->next = temp->next;
    delete temp;

    return true;
}
