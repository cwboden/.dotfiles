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

inline LL Power(int b, int p) { LL ret = 1; for ( int i = 1; i <= p; i++ ) ret *= b; return ret; }
const int dr [] = {-1, -1, 0, 1, 1, 1, 0, -1};
const int dc [] = {0, 1, 1, 1, 0, -1, -1, -1};

#define f(i, a) for( int i = (0); i < (a); i++ )
#define fs(i, sz) for( size_t i = 0; i < sz.size (); i++ )
#define fe(i, x) for(typeof (x.begin()) i = x.begin(); i != x.end (); i++)
#define set(a, s) memset(a, s, sizeof (a))
#define max(a, b)  (a < b ? b : a)
#define min(a, b)  (a > b ? b : a)

using namespace std;

struct Node {
    int value;
    struct Node * next;
    struct Node * prev;
};

void deleteNthToLastNode(struct Node * head, int k) {

    if (head == nullptr) return;

    // Move ahead k spaces forward
    int count = 0;
    struct Node * ahead = head;
    while (count < k) {
        if (ahead == nullptr) return;

        ahead = ahead->next;
        count++;
    }

    // Move ahead to the end, head is now kth-to-last
    while (ahead != nullptr) {
        ahead = ahead->next;
        head = head->next;
    }

    // Delete node
    head->prev->next = head->next;
    head->next->prev = head->prev;
    delete head;
}

int main(int argc, char * argv[])
{
    struct Node * head = new struct Node;
    struct Node * current = head;
    head->value = 0;

    for (int i = 1; i < 5; ++i) {
        struct Node * newNode = new struct Node;
        newNode->value = i;

        current->next = newNode;
        current = current->next;
    }
    current->next = nullptr;

    deleteNthToLastNode(head, 4);

    while (head->next != nullptr) {
        cout << head->value << ' ';
        head = head->next;
    }

    return 0;
}
