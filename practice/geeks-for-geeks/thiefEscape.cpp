#include <stdio.h>
#include <iostream>
#include <vector>

using namespace std;

void thiefEscape(int jump, int slip, vector<int> walls) {

    int totalJumps = 0;
    for (int wall : walls) {
        totalJumps++;
        wall -= jump;

        while (wall > 0) {
            wall += slip;
            wall -= jump;
            totalJumps ++;
        }
    }

    cout << totalJumps << endl;
}

int main() {
    int numTests;
    cin >> numTests;

    int jump, slip, walls;
    for (int i = 0; i < numTests; ++i) {
        cin >> jump >> slip >> walls;

        vector<int> wallHeights;
        int wallHeight;
        for (int i = 0; i < walls; ++i) {
            cin >> wallHeight;
            wallHeights.push_back(wallHeight);
        }

        thiefEscape(jump, slip, wallHeights);
    }
}
