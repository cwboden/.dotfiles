// You are given a binary tree in which each node contains a value. Design an algorithm
// to print all paths which sum up to that value. Note that it can be any path in the tree
// - it does not have to start at the root.

using namespace std;

struct Node
{
    Node* left = nullptr;
    Node* right = nullptr;
    int datum;
};

void findTreeSum(Node* root, int target)
{
    vector<int> path;
    int value = 0;

    recursiveSum(root, target, path, value)
}

void recursiveSum(Node* root, int target, vector<int> path, int value)
{
    if (root == nullptr)
    {
        return;
    }

    // Add current path value and compare to target
    value += root->datum;
    if (value == target)
    {
        printPath(path);
    }

    // Start new trails to calculate possible sum
    findTreeSum(root->left, target);
    findTreeSum(root->right, target);

    // Continue down current path
    recursiveSum(root->left, target, path, value);
    recursiveSum(root->right, target, path, value);

}

void printPath(vector<int> path)
{
    for (int i : path)
    {
        cout << i << " < ";
    }
    cout << endl;
}
