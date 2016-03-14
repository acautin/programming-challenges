#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

int parseDirection(string);

int main()
{
    int W; // width of the building.
    int H; // height of the building.
    cin >> W >> H; cin.ignore();
    int N; // maximum number of turns before game over. (Why do I care ??)
    cin >> N; cin.ignore();
    int X;
    int Y;
    cin >> X >> Y; cin.ignore(); // Batman's current location


    int W0 = 0; // Starting left limit for search
    int H0 = 0; // Starting up limit for search
    
    int direction;
    
    // Strategy: 2d binary search.
    while (1) {
        string bombDir; // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)
        cin >> bombDir; cin.ignore();

        // Write an action using cout. DON'T FORGET THE "<< endl"
        // To debug: cerr << "Debug messages..." << endl;
        direction = parseDirection(bombDir);
        
        switch (direction) {
        case 0:
            // Bomb is up so we search from here...
            H = Y-1;
            Y = (H + H0)/2;
            break;
        case 1:
            H = Y-1;
            Y = (H + H0)/2;
            W0 = X+1;
            X = (W + W0)/2;
            break;
        case 2:
            W0 = X+1;
            X = (W + W0)/2;
            break;
        case 3:
            H0 = Y+1;
            Y = (H + H0)/2;
            W0 = X+1;
            X = (W + W0)/2;
            break;
        case 4:
            H0 = Y+1;
            Y = (H + H0)/2;
            break;
        case 5:
            H0 = Y+1;
            Y = (H + H0)/2;
            W = X-1;
            X = (W + W0)/2;
            break;
        case 6:
            W = X-1;
            X = (W + W0)/2;
            break;
        case 7:
            H = Y-1;
            Y = (H + H0)/2;
            W = X-1;
            X = (W + W0)/2;
            break;
        }

        cout << X << " " << Y << endl; // the location of the next window Batman should jump to.
    }
}

int parseDirection(string s) {
    if (s == "U") {return 0;}
    if (s == "UR") {return 1;}
    if (s == "R") {return 2;}
    if (s == "DR") {return 3;}
    if (s == "D") {return 4;}
    if (s == "DL") {return 5;}
    if (s == "L") {return 6;}
    if (s == "UL") {return 7;}
}
