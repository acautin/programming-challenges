#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using std::cin;
using std::cout;
using std::endl;
using std::string;
using std::vector;
using std::to_string;

string sequence(int, int);
vector<int> next(vector<int>);
void examples();

int main()
{
    int seed;
    cin >> seed; cin.ignore();
    int lineNumber;
    cin >> lineNumber; cin.ignore();
    cout << sequence(seed, lineNumber) << endl;
}

string sequence(int seed, int lineNumber) {
	vector<int> line;
	line.push_back(seed);
	
	for(int i = 1; i < lineNumber; ++i) {
		line = next(line);
	}

	string result;
	for(int i = 0; i < line.size(); ++i) {
		if(i != 0) {
			result += " ";
		}
		result += to_string(line[i]);
	}
	
	return result;
}

vector<int> next(vector<int> current) {
	vector<int> result;
	if(!current.empty()) {
		int first = current.front();
		int i = 1;
		while(current[i] == first && i < current.size()) {
			++i;
		}
		result.push_back(i);
		result.push_back(first);
		vector<int> rest;
		while(i < current.size()) {
			rest.push_back(current[i]);
			++i;
		}
		vector<int> restResult = next(rest);
		result.insert(result.end(), restResult.begin(), restResult.end());
	}
	return result;
}

void examples() {
	if(sequence(1, 6) != "3 1 2 2 1 1") {
		cout << "failed" << endl;
	} else if(sequence(25, 10) != "3 1 1 3 1 1 2 2 2 1 1 3 1 1 1 2 3 1 1 3 3 2 2 1 1 25") {
		cout << "failed" << endl;
	} else {
		cout << "Everything green! :D" << endl;
	}
}

	
