// Carson Boden / November 2016
// Prints out fibonacci and prime numbers
// as well as the intersection of them
// for two, positive whole numbers

#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <cmath>

using namespace std;

// Helper function to print out the results
void printVec(vector<int> vec, string name)
{
	cout << name << ":\n";
	for (size_t i = 0; i < vec.size(); ++i)
	{
		cout << vec[i] << " ";
	} // for
	cout << endl;
} // printVec()

int main()
{
	int low, high;
	double first, second;

	// Prompt and input for the numbers
	cout << "Please enter (2) whole, positive numbers: " << endl;
	cin >> first >> second;

	// Reprompts on invalid input
	while (first < 0 || floor(first) != first)
	{
		cout << "First value invalid. Try again." << endl;
		cin >> first;
	} // while
	while (second < 0 || floor(second) != second)
	{
		cout << "Second value invalid. Try again." << endl;
		cin >> second;
	} // while

	// Swaps low and high if they are inverted
	high = static_cast<int>(second);
	low = static_cast<int>(first);
	if (high < low) swap(low, high);

	// Vectors for found values
	vector<int> primes;
	vector<int> fibs;

	// Calculates prime numbers between values
	for (int i = low + 1; i < high; ++i)
	{
		// Flag to confirm if number is prime
		bool isPrime = true;

		// Checks factors up to half of the number (Since 2 is the smallest divisor)
		for (size_t j = 2; j <= i / 2; ++j)
		{
			// If the number i is cleanly divisible
			if (i % j == 0)
			{
				isPrime = false;
				break;
			} // if
		} // for

		  // Print the number if it's prime
		if (isPrime && i != 1)
		{
			primes.push_back(i);
		} // if

	} // for

	// Calculates fibonacci numbers between values
	int num = 0, next = 1, temp;
	while (num < high)
	{
		// Pushes back a fibonacci number if it falls within the range
		if (low < num) fibs.push_back(num);

		// Calculates the next fibonacci number
		// F(n) = F(n - 1) + F(n -2)
		temp = num + next;
		num = next;
		next = temp;
	} // while

	// Calculates the intersection of the two vectors
	size_t p = 0, f = 0;
	vector<int> intersection;
	while (p < primes.size() && f < fibs.size())
	{
		if (primes[p] < fibs[f]) p++;
		else if (fibs[f] < primes[p]) f++;
		else
		{
			intersection.push_back(primes[p++]);
			f++;
		} // else
	} // while

	printVec(primes, "Primes");
	printVec(fibs, "Fibonaccis");
	printVec(intersection, "Intersection");

	return 0;
} // main()
