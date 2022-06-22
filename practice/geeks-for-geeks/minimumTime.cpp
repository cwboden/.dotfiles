#include <stdio.h>
#include <iostream>
#include <vector>

using namespace std;

void printMinTime(int s1, int s2, int docs) {

    int timeTaken = 0;

    int timeS1 = 0, timeS2 = 0;

    while (docs > 0) {
        if (timeS1 + s1 < timeS2 + s2) {
            timeS1 += s1;
        } else {
            timeS2 += s2;
        }

        docs--;
    }

    if (timeS1 > timeS2) {
        timeTaken = timeS1;
    } else {
        timeTaken = timeS2;
    }

    cout << timeTaken << endl;
}

int main() {
    int numTests;
    cin >> numTests;

    int s1, s2, docs;
    for (int i = 0; i < numTests; ++i) {
        cin >> s1;
        cin >> s2;
        cin >> docs;

        // cout << "S1: " << s1 << "\tS2: " << s2 << "\tDocs: " << docs << endl;

        printMinTime(s1, s2, docs);
    }
}
