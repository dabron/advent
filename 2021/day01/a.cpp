#include <iostream>

using namespace std;

int main()
{
	int larger = 0;
	int current = 0;
	int previous = 0;

	cin >> previous;

	while (cin >> current)
	{
		if (current > previous)
			larger++;
		previous = current;
	}

	cout << "larger count: " << larger << endl;
}
