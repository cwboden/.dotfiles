// Given a sequence, find the length of the longest increasing subsequence from a given sequence .
// The longest increasing subsequence means to find a subsequence of a given sequence in which the subsequence's elements are in sorted order, lowest
// to highest, and in which the subsequence is as long as possible. This subsequence is not necessarily contiguous, or unique.

#include <stdio.h>
#include <iostream>
#include <vector>

using namespace std;

void longestIncreasingSequence(vector<int> nums) {
    int maxLength = 0;
    int length = 0;
    int lastNum = 0;

    for (int num : nums) {
        if (num > lastNum) {
            length++;
        }
        else {
            if (length > maxLength) {
                maxLength = length;
            }
            length = 0;
        }
        lastNum = num;
    }

    cout << maxLength << endl;
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

	    longestIncreasingSequence(nums);
	}

	return 0;
}
