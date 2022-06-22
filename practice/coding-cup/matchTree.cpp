// You have two very large binary trees: T1, with millions of nodes, and T2, with hun-
// dreds of nodes. Create an algorithm to decide if T2 is a subtree of T1.

struct Node
{
    Node* left = nullptr;
    Node* right = nullptr;
    int datum;
};

bool containsTree(Node* bigTree, Node* littleTree)
{
    if (littleTree == nullptr)
    {
        // Null tree will always be a subtree
        return true;
    }

    return subTree(bigTree, littleTree);
}

bool subTree(Node* bigTree, Node* littleTree)
{
    if (bigTree == nullptr)
    {
        // No non-null tree can be a subtree of a null tree
        return false;
    }

    if (bigTree->datum == littleTree->datum)
    {
        // Possible subtree found, call search
        if (matchTree(bigTree, littleTree))
        {
            return true;
        }
    }

    // Continue looking for possible matches to root of littleTree
    return (subTree(bigTree->left, littleTree) ||
            subTree(bitTree->right, littleTree));

}

bool matchTree(Node* bigTree, Node* littleTree)
{
    if (bigTree == nullptr && littleTree == nullptr)
    {
        // Both trees exhausted at the same time, match found!
        return true;
    }
    if (bigTree == nullptr || littleTree == nullptr)
    {
        // Only one tree exhausted, trees do not match
        return false;
    }
    if (bigTree->datum == littleTree->datum)
    {
        // Trees continue to match, check on children
        return (matchTree(bigTree->right, littleTree->right) &&
                matchTree(bigTree->left, littleTree->left));
    }

    // Data does not match, so the trees don't either
    return false;
}
