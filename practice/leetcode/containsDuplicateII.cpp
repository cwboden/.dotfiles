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

bool containsDuplicate(vector<int>& nums, int k) {
    map<int, vector<int> > numMap;

    for (int i = 0; i < nums.size(); ++i) {
        int num = nums[i];
        auto it = numMap.find(num);

        if (it != numMap.end()) {
            for (int index : it->second) {
                if ((i - index) > k) {
                    return true;
                }
            }
        }

        numMap[num].push_back(i);
    }

    return false;
}
