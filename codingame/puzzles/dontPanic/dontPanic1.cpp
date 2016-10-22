#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int main()
{
    int nbFloors; // number of floors
    int width; // width of the area
    int nbRounds; // maximum number of rounds
    int exitFloor; // floor on which the exit is found
    int exitPos; // position of the exit on its floor
    int nbTotalClones; // number of generated clones
    int nbAdditionalElevators; // number of additional elevators that you can build
    int nbElevators; // number of elevators
    cin >> nbFloors >> width >> nbRounds >> exitFloor >> exitPos >> nbTotalClones >> nbAdditionalElevators >> nbElevators; cin.ignore();

	int elevatorPos[15]; // Position of the elevator on each floor (indexed by floor max 15 floors).
	cerr << nbElevators << endl;
    for (int i = 0; i < nbElevators; i++) {
        int floor; // floor on which this elevator is found
        int pos; // position of the elevator on its floor
        cin >> floor >> pos; cin.ignore();
		elevatorPos[floor] = pos;
    }

    // game loop
	int currentFloor = 0;
    while (1) {
        int cloneFloor; // floor of the leading clone
        int clonePos; // position of the leading clone on its floor
        string direction; // direction of the leading clone: LEFT or RIGHT
        cin >> cloneFloor >> clonePos >> direction; cin.ignore();

		string command = "WAIT";
		if(cloneFloor == currentFloor) {
			int targetPos = (exitFloor == cloneFloor)? exitPos: elevatorPos[cloneFloor];
			if((direction == "RIGHT" && clonePos > targetPos) ||
			   (direction == "LEFT" && clonePos < targetPos)) {
				command = "BLOCK";
			}
			currentFloor++;
		}
		cout << command << endl;
    }
}
