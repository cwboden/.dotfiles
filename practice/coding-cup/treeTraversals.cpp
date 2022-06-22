#include <iostream>
#include <cstdio>
#include <algorithm>
#include <cstring>
#include <string>
#include <cctype>
#include <stack>
#include <queue>
#include <list>
#include <vector>
#include <map>
#include <sstream>
#include <cmath>
#include <bitset>
#include <utility>
#include <set>
#include <numeric>
#include <ctime>

#define Inf 2147483647
#define Pi acos(-1.0)
#define N 1000000
#define LL long long

inline LL power(int b, int p) { LL ret = 1; for ( int i = 1; i <= p; i++ ) ret *= b; return ret; }
const int dr [] = {-1, -1, 0, 1, 1, 1, 0, -1};
const int dc [] = {0, 1, 1, 1, 0, -1, -1, -1};

#define f(i, a) for( int i = (0); i < (a); i++ )
#define fs(i, sz) for( size_t i = 0; i < sz.size (); i++ )
#define fe(i, x) for(typeof (x.begin()) i = x.begin(); i != x.end (); i++)
#define set(a, s) memset(a, s, sizeof (a))
#define max(a, b)  (a < b ? b : a)
#define min(a, b)  (a > b ? b : a)

using namespace std;

struct node
{
  int value;
  node *left;
  node *right;
};

void prefixTraversal(node * head) {
    if (head == nullptr) return;

    cout << head->value << ' ';

    prefixTraversal(head->left);
    prefixTraversal(head->right);
}

void postfixTraversal(node * head) {
    if (head == nullptr) return;

    postfixTraversal(head->left);
    postfixTraversal(head->right);

    cout << head->value << ' ';
}

void inorderTraversal(node * head) {
    if (head == nullptr) return;

    inorderTraversal(head->left);
    cout << head->value << ' ';
    inorderTraversal(head->right);
}

void levelTraversal(node * head) {
    if (head == nullptr) return;

    queue<node *> nodes;
    nodes.push(head);
    nodes.push(nullptr);

    while (nodes.size() > 1) {
        node * top = nodes.front();

        if (top == nullptr) {
            nodes.pop();
            cout << endl;
            nodes.push(nullptr);
            continue;
        }

        cout << top->value << ' ';

        if (top->left != nullptr) nodes.push(top->left);
        if (top->right != nullptr) nodes.push(top->right);

        nodes.pop();
    }
}

int main(int argc, char * argv[])
{

    node * head = new node;
    head->value = 3;

    head->left = new node;
    node * left = head->left;
    left->value = 1;

    head->right = new node;
    node * right = head->right;
    right->value = 5;

    left->left = new node;
    left->left->value = 0;
    left->left->left = nullptr;
    left->left->right = nullptr;

    left->right = new node;
    left->right->value = 2;
    left->right->left = nullptr;
    left->right->right = nullptr;

    right->left = new node;
    right->left->value = 4;
    right->left->left = nullptr;
    right->left->right = nullptr;

    right->right = new node;
    right->right->value = 6;
    right->right->left = nullptr;
    right->right->right = nullptr;

    cout << "Prefix: \t";
    prefixTraversal(head);
    cout << endl << "Postfix:\t";
    postfixTraversal(head);
    cout << endl << "In Order:\t";
    inorderTraversal(head);
    cout << endl << "Level:\t\t";
    levelTraversal(head);
    cout << endl;

    return 0;
}
