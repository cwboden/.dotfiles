#include <stdio.h>
#include <iostream>
#include <string>
#include <map>

using namespace std;

int romanToInt(string input)
{
    static map<char, int> romans;

    romans['M'] = 1000;
    romans['D'] = 500;
    romans['C'] = 100;
    romans['L'] = 50;
    romans['X'] = 10;
    romans['V'] = 5;
    romans['I'] = 1;

    int sum = 0;

    for (int i = 0; i < input.size()-1; ++i)
    {
        if (romans[input[i]] < romans[input[i+1]])
        {
            sum -= romans[input[i]];
        }
        else
        {
            sum += romans[input[i]];
        }
    }

    sum += romans[input.back()];

    return sum;
}

int main()
{
	//code
	int numTests;
	cin >> numTests;

	for (int i = 0; i < numTests; i++)
	{
	    string stringIn;
	    cin >> stringIn;

	    cout << romanToInt(stringIn) << endl;
	}

	return 0;
}
