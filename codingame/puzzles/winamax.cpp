#include <iostream>
#include <list>

using namespace std;

char normalize(string);
void print(list<char> &l);

int main() {
  int n; // the number of cards for player
  list<char> cards_player1; 
  cin >> n; cin.ignore();
  for (int i = 0; i < n; i++) {
    string cardp1; // the n cards of player 1
    cin >> cardp1; cin.ignore();
    cards_player1.push_back(normalize(cardp1));
  }
  // the number of cards for player 2
  cin >> n; cin.ignore();
  list<char> cards_player2;
  for (int i = 0; i < n; i++) {
    string cardp2; // the m cards of player 2
    cin >> cardp2; cin.ignore();
    cards_player2.push_back(normalize(cardp2));
  }

  int rounds = 0;
  while (cards_player1.size() != 0 && cards_player2.size() != 0) {
    ++rounds;
    char p1 = cards_player1.front();
    char p2 = cards_player2.front();
    cards_player1.pop_front();
    cards_player2.pop_front();
    if (p1 > p2) {
      cards_player1.push_back(p1);
      cards_player1.push_back(p2);
    } else if (p2 > p1) {
      cards_player2.push_back(p1);
      cards_player2.push_back(p2);
    } else {
      bool war_ended = false;
      list<char> q1;
      list<char> q2;
      // War...      
      while (!war_ended) {
        q1.push_back(p1);
        q2.push_back(p2);
        if(cards_player1.size() < 4 || cards_player2.size() < 4) {
          cout << "PAT" << endl;
          return 0;
        }

        for(int i = 0; i < 3; ++i){
          q1.push_back(cards_player1.front());
          cards_player1.pop_front();
        }

        for(int i = 0; i < 3; ++i) {
          q2.push_back(cards_player2.front());
          cards_player2.pop_front();
        }

        p1 = cards_player1.front();
        p2 = cards_player2.front();
        cards_player1.pop_front();
        cards_player2.pop_front();
        if (p1 > p2) {
          while(!q1.empty()) {
            cards_player1.push_back(q1.front());
            q1.pop_front();
          }
          cards_player1.push_back(p1);
          while(!q2.empty()) {
            cards_player1.push_back(q2.front());
            q2.pop_front();
          }
          cards_player1.push_back(p2);
          war_ended = true;
        } else if (p2 > p1) {
          while(!q1.empty()) {
            cards_player2.push_back(q1.front());
            q1.pop_front();
          }
          cards_player2.push_back(p1);
          while(!q2.empty()) {
            cards_player2.push_back(q2.front());
            q2.pop_front();
          }
          cards_player2.push_back(p2);
          war_ended = true;
        }
      }
    }
    if (rounds > 8) {
      print(cards_player1);
      cerr << "-- cha --" << endl;
    }
  }

  int winner = 1;
  if (cards_player2.size() != 0) {
    winner = 2;
  }

  cout << winner << " " << rounds << endl;
}

char normalize(string s) {
  if (s[0] == 'K') {
    return 'Y';
  } else if (s[0] == 'A') {
    return 'Z';
  } else if (s[0] == '1') {
    return 'I';
  }
  return s[0];
}

void print(list<char> &l) {
  for (std::list<char>::iterator it = l.begin(); it != l.end(); ++it) {
    cerr << "p1: " << *it << endl;
  }
}