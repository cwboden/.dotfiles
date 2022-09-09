/**
 * PROMPT:
 * Given a binary tree, determine if it is a valid binary search tree (BST).
 *
 * Assume a BST is defined as follows:
 *  The left subtree of a node contains only nodes with keys less than the node's key.
 *  The right subtree of a node contains only nodes with keys greater than the node's key.
 *  Both the left and right subtrees must also be binary search trees.
 *
 * @author Carson Boden
 */

#include <climits>
#include <iostream>
#include <vector>

struct TreeNode
{
  int val;
  TreeNode* left;
  TreeNode* right;
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) { }
};

bool is_valid_binary_tree_h(TreeNode* head, int min, int max)
{
  if (head == nullptr) {
    return true;
  }

  if (head->val <= min || max <= head->val) {
    return false;
  }

  return is_valid_binary_tree_h(head->left, min, head->val) &&
    is_valid_binary_tree_h(head->right, head->val, max);
}

bool is_valid_binary_tree(TreeNode* head)
{
  return is_valid_binary_tree_h(head, INT_MIN, INT_MAX);
}

int main(int argc, char* argv[])
{
  TreeNode* head = new TreeNode(2);
  head->left = new TreeNode(1);
  head->right = new TreeNode(3);

  std::cout << (is_valid_binary_tree(head) ? "true" : "false") << std::endl;

  head->left->left = new TreeNode(0);
  head->left->right = new TreeNode(10);

  std::cout << (is_valid_binary_tree(head) ? "true" : "false") << std::endl;

  return 0;
}
