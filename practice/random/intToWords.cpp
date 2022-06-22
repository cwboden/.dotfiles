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

string convertHundred(int num) {

    static string dict1s[20] = {"", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen" };
    static string dict10s[10] = {"", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"};

    string result = "";

    // Get digits
    int ones = num % 10;
    int tens = num % 100;
    int hundreds = num / 100;

    // Check for teens
    if (tens < 20) {
        result = dict1s[tens];
    }
    // Else add ones and tens
    else {
        result = dict10s[tens / 10];

        if (ones > 0) {
            result = result + '-' + dict1s[ones];
        }
    }

    // Tack on the hundreds
    if (hundreds > 0) {
        result = dict1s[hundreds] + " hundred " + result;
    }

    return result;
}

string convertInt(int num) {

    static string dictBigs[4] = {"", "thousand", "million", "billion"};

    string result = "";
    int size = 0;

    while (num > 0) {
        int partial = num % 1000;
        num /= 1000;

        result = convertHundred(partial) + ' ' + dictBigs[size] + ' ' + result;
        size ++;
    }

    return result;
}

int main(int argc, char * argv[])
{
    int num;

    do {
        cout << "Please enter a number: ";
        cin >> num;

        cout << convertInt(num) << endl;
    } while (num != 0);

    return 0;
}
