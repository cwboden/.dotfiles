#include <stdio.h>
#include <iostream>
#include <vector>

using namespace std;

void equilibriumPoint(vector<int> nums)
{
    if (nums.size() == 1)
    {
        cout << 1 << endl;
        return;
    }
    if (nums.size() == 2)
    {
        cout << -1 << endl;
        return;
    }

    bool foundPoint = false;
    vector<int> cumSum(nums.size());
    cumSum[0] = nums[0];

    for (int i = 1; i < nums.size(); ++i)
    {
        cumSum[i] = cumSum[i-1] + nums[i];
    }

    for (int i = 1; i < cumSum.size()-1; ++i)
    {
        if (cumSum[i-1] == cumSum.back() - cumSum[i])
        {
            cout << i+1 << endl;
            foundPoint = true;
        }
    }

    if (!foundPoint)
    {
        cout << -1 << endl;
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
