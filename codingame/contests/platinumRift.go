package main

import "fmt"
import "os"
import "math/rand"

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

type zoneInfo struct {
	id, owner, pods, plat int
	locked bool
}

type dest struct {
	id, step int
}

func getMyPods(id int, pods ...int) int {
	return pods[id];
}

func searchDest(id int, listLinks [][]int, zonesInfo []zoneInfo, myId int) (bool, int, []bool) {
	added := make([]bool, len(zonesInfo))
	toVisit := make([]dest, 0)
	pos := 0
	added[id] = true
	for _, v := range listLinks[id] {
		if !added[v] {
			toVisit = append(toVisit, dest{v, v})
			added[v] = true
		}
	}
	for pos < len(toVisit) {
		id = toVisit[pos].id
		step := toVisit[pos].step
		if zonesInfo[id].owner != myId {
			return true, step, added
		} else {
			pos++
			for _, v := range listLinks[id] {
				if !added[v] {
					toVisit = append(toVisit, dest{v, step})
					added[v] = true;
				}
			}
		}
	}
	return false, -1, added
}

func main() {
    // playerCount: the amount of players (2 to 4)
    // myId: my player ID (0, 1, 2 or 3)
    // zoneCount: the amount of zones on the map
    // linkCount: the amount of links between all zones
    var playerCount, myId, zoneCount, linkCount int
    fmt.Scan(&playerCount, &myId, &zoneCount, &linkCount)
    
	var zonesPlat = make([]int, zoneCount)
	var listLinks = make([][]int, zoneCount)
	var linkInitialized = make([]bool, zoneCount)
	zonesInfo := make([]zoneInfo, zoneCount)

    for i := 0; i < zoneCount; i++ {
        // zoneId: this zone's ID (between 0 and zoneCount-1)
        // platinumSource: the amount of Platinum this zone can provide per game turn
        var zoneId, platinumSource int
        fmt.Scan(&zoneId, &platinumSource)
		zonesPlat[zoneId] = platinumSource
    }

    for i := 0; i < linkCount; i++ {
        var zone1, zone2 int
        fmt.Scan(&zone1, &zone2)

		if !linkInitialized[zone1] {
			linkInitialized[zone1] = true
			listLinks[zone1] = []int{zone2}
		} else {
			listLinks[zone1] = append(listLinks[zone1], zone2)
		}

		if !linkInitialized[zone2] {
			linkInitialized[zone2] = true
			listLinks[zone2] = []int{zone1}
		} else {
			listLinks[zone2] = append(listLinks[zone2], zone1)
		}
    }

    for {
        // platinum: my available Platinum
        var platinum int
        fmt.Scan(&platinum)
        amount := platinum / 20;
		fmt.Fprintf(os.Stderr, "The amount of reinforcements: %d\n", amount)

		newPods := "WAIT"
		movePods := "WAIT"
		
        for i := 0; i < zoneCount; i++ {
            // zId: this zone's ID
            // ownerId: the player who owns this zone (-1 otherwise)
            // podsP0: player 0's PODs on this zone
            // podsP1: player 1's PODs on this zone
            // podsP2: player 2's PODs on this zone (always 0 for a two player game)
            // podsP3: player 3's PODs on this zone (always 0 for a two or three player game)
            var zId, ownerId, podsP0, podsP1, podsP2, podsP3 int
            fmt.Scan(&zId, &ownerId, &podsP0, &podsP1, &podsP2, &podsP3)
			myPods := getMyPods(myId, podsP0, podsP1, podsP2, podsP3)
			zonesInfo[zId].id = zId
			zonesInfo[zId].owner = ownerId
			zonesInfo[zId].pods = myPods
			zonesInfo[zId].plat = zonesPlat[zId]
        }

		var myZone zoneInfo
		myZone.plat = -1
		for i, zone := range zonesInfo {
			if(zone.owner == -1) {
				if(amount > 0 && (zone.plat > 3 || rand.Intn(100) > 98 - (i + i*zone.plat)/10)) {
					if newPods == "WAIT" {
						newPods = fmt.Sprintf("1 %d", zone.id)
					} else {
						newPods += fmt.Sprintf(" 1 %d", zone.id)
					}
					amount--;
				}
			} else if (zone.owner == myId && !zone.locked) {
				if(myZone.plat == -1) { // First owned zone.
					myZone = zoneInfo{zone.id, myId, zone.pods, zone.plat, false}
				} else if(zone.pods < myZone.pods) {
					myZone = zoneInfo{zone.id, myId, zone.pods, zone.plat, false}
				} else if(zone.pods == myZone.pods && zone.plat > myZone.plat) {
					myZone = zoneInfo{zone.id, myId, zone.pods, zone.plat, false}
				}
				//Search for empty links.
				var emptyLinks = make([]zoneInfo, 0);
				for _, destId := range listLinks[zone.id] {
					if(zonesInfo[destId].owner != myId) {
						if len(emptyLinks) > 0 {
							if emptyLinks[len(emptyLinks)-1].plat < zonesInfo[destId].plat {
								emptyLinks = append(emptyLinks, zonesInfo[destId]);
							} else {
								temp := emptyLinks[len(emptyLinks)-1]
								emptyLinks[len(emptyLinks)-1] = zonesInfo[destId]
								emptyLinks = append(emptyLinks, temp);
							}
						} else {
							emptyLinks = append(emptyLinks, zonesInfo[destId]);
						}
					}
				}
				
				for zone.pods > 0 && (zone.plat < 4 || zone.pods > 1 || platinum < 201) {
					var destMove int
					var moveStr string
					var added []bool
					found := true
					if(len(emptyLinks) > 0) {
						//move to an empty position.
						destMove = emptyLinks[len(emptyLinks)-1].id
						emptyLinks = emptyLinks[:len(emptyLinks)-1];
						moveStr = fmt.Sprintf("1 %d %d", zone.id, destMove)
						zone.pods--;
					} else {
						//move it anywhere :p.
						// TODO!! If there is nothing in range (found false) do not search anymore. 
						found, destMove, added = searchDest(zone.id, listLinks, zonesInfo, myId);
						if zone.plat < 4 || platinum < 201 {
							moveStr = fmt.Sprintf("%d %d %d", zone.pods, zone.id, destMove)
							zone.pods = 0
						} else {
							moveStr = fmt.Sprintf("%d %d %d", zone.pods - 1, zone.id, destMove)
							zone.pods = 1
						}
					}
					if found {
						if movePods == "WAIT" {
							movePods = moveStr;
						} else {
							movePods += " " + moveStr
						}
					} else {
						for idx, isLocked := range added {
							zonesInfo[idx].locked = zonesInfo[idx].locked || isLocked
						} 
					}
				}
			}
		}

		//Reinforcements go to most profitable zone. // maybe in a list not all to the same place.
		if(amount > 0 && myZone.plat != -1) {
			if newPods == "WAIT" {
				newPods = fmt.Sprintf("%d %d", amount, myZone.id)
			} else {
				newPods += fmt.Sprintf(" %d %d", amount, myZone.id)
			}
		}

        fmt.Println(movePods)
        fmt.Println(newPods)
    }
}
