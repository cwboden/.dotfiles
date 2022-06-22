// Carson Boden / November 2016
// Prints out prime numbers between 1 and 100,000

int main()
{
	// Range from 1 to 100,000
	for (size_t i = 1; i <= 100000; ++i)
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
		if (isPrime)
		{
			cout << i << endl;
		} // if

	} // for

	return 0;
}
