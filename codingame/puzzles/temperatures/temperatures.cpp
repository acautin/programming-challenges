#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
int main()
{
    int n; // the number of temperatures to analyse
    cin >> n; cin.ignore();
	int temp;
	int result = 0;
	for(int i = 0; i < n; i++) {
		cin >> temp;
		if(i == 0 || (abs(temp) < abs(result)) || (abs(temp) == abs(result) && temp > result)) {
			result = temp;
		}
	}
   	
    // Write an action using cout. DON'T FORGET THE "<< endl"
    // To debug: cerr << "Debug messages..." << endl;

    cout << result << endl;
}
