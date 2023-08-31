#include <iostream>

using namespace std;

int main()
{
	int larger = 0;
	int current;
	int previous;
	int x, y, z;

	cin >> x;
	cin >> y;
	cin >> z;

	previous = x + y + z;
	x = y;
	y = z;

	while (cin >> z)
	{
		current = x + y + z;
		if (current > previous)
			larger++;
		previous = current;
		x = y;
		y = z;
	}

	cout << "larger count: " << larger << endl;
}
