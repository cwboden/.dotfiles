#include <stdio.h>
#include <iostream>
#include <vector>

using namespace std;

void minimumJumps(vector<int> nums) {

    int currentNum;
    int currentIndex = 0;
    int numJumps = 0;

    while (currentIndex < nums.size()) {
        currentNum = nums[currentIndex];

        int max = 0;
        for (int i = currentIndex; i < currentNum && i < nums.size(); ++i) {
            if (max < nums[i]) {
                max = nums[i];
            }
        }

        if (max == 0) {
            cout << -1 << endl;
            return;
        }

        currentIndex += max;
        numJumps++;
    }
}

int main() {
	int numTests;
	cin >> numTests;

	for (int i = 0; i < numTests; i++)
	{
        int size;
        cin >> size;

        vector<int> nums(size);
        for (int i = 0; i < size; ++i)
        {
            int num;
            cin >> num;
            nums[i] = num;
        }

	    equilibriumPoint(nums);
	}

	return 0;
}
