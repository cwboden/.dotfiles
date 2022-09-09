#include <stdio.h>
#include <iostream>
#include <vector>

using namespace std;

bool basicNoGCD(int x, int y) {
    return !((x % 2 == 0) && (y % 2 == 0));
}

void largestProperFraction(int N) {

    float currentMax = 0.0;
    int numerator, denominator;

    for (int n = 1, d = N-1; n < d; n++, d--) {

        float newValue = float(n) / float(d);

        if (newValue > currentMax && noGCD(n, d)) {
            currentMax = newValue;
            numerator = n;
            denominator = d;
        }
    }

    cout << numerator << ' ' << denominator << endl;

}

int main() {
    int numTests;
    cin >> numTests;

    int N;
    for (int i = 0; i < numTests; ++i) {
        cin >> N;

        largestProperFraction(N);
    }
}
