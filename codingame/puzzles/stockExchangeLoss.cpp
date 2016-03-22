#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using std::cin;
using std::cout;
using std::endl;
using std::vector;

int main()
{
    int n;
    cin >> n; cin.ignore();
	vector<int> prices;
    for (int i = 0; i < n; i++) {
        int v;
        cin >> v; cin.ignore();
		prices.push_back(v);
    }

	int min = 0;
	int acc = 0;
	for(int i = 1; i < n; ++i) {
		acc += prices[i] - prices[i-1];
		if(acc < min) {
			min = acc;
		}
		if(acc > 0) {
			acc = 0;
		}
	}
    cout << min << endl;
}
