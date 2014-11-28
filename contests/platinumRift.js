/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

var inputs = readline().split(' ');
var playerCount = parseInt(inputs[0]); // the amount of players (2 to 4)
var myId = parseInt(inputs[1]); // my player ID (0, 1, 2 or 3)
var zoneCount = parseInt(inputs[2]); // the amount of zones on the map
var linkCount = parseInt(inputs[3]); // the amount of links between all zones
var zonesPlat = [];
var listLinks = [];
var firstTurn = false; // TODO: Disable this or improve for a sensible one.

for (var i = 0; i < zoneCount; i++) {
    var inputs = readline().split(' ');
    var zoneId = parseInt(inputs[0]); // this zone's ID (between 0 and zoneCount-1)
    var platinumSource = parseInt(inputs[1]); // the amount of Platinum this zone can provide per game turn
    zonesPlat[zoneId] = platinumSource;
}

for (var i = 0; i < linkCount; i++) {
    var inputs = readline().split(' ');
    var zone1 = parseInt(inputs[0]);
    var zone2 = parseInt(inputs[1]);
    if(!listLinks[zone1]) {
        listLinks[zone1] = [zone2];
    } else {
        listLinks[zone1].push(zone2);
    }
    if(!listLinks[zone2]) {
        listLinks[zone2] = [zone1];
    } else {
        listLinks[zone2].push(zone1);
    }
}

// game loop
while (true) {
    var platinum = parseInt(readline()); // my available Platinum
    var amount = Math.floor(platinum / 20);
    printErr('The amount: ' + amount);
    var out = 'WAIT';
    var move = 'WAIT';
    var zonesInfo = [];
    for(var i = 0; i < zoneCount; ++i) {
        var inputs = readline().split(' ');
        var zId = parseInt(inputs[0]); // this zone's ID
        var ownerId = parseInt(inputs[1]); // the player who owns this zone (-1 otherwise)
        var pods = [];
        pods[0] = parseInt(inputs[2]); // player 0's PODs on this zone
        pods[1] = parseInt(inputs[3]); // player 1's PODs on this zone
        pods[2] = parseInt(inputs[4]); // player 2's PODs on this zone
        pods[3] = parseInt(inputs[5]); // player 3's PODs on this zone
        var zoneInfo = {
            id: zId,
            owner: ownerId,
            pods: pods
        };
        zonesInfo.push(zoneInfo);
    }

    var myZone = {id: 0, plat: -1};
    for (var i = 0; i < zonesInfo.length; i++) {
        if(zonesInfo[i].owner == -1) {
            if(amount > 0 && (zonesPlat[zonesInfo[i].id] > 3 || Math.random() > 0.99 - i/1000)) {
                out = (out === 'WAIT')?'1 ' + zonesInfo[i].id:out + ' 1 ' + zonesInfo[i].id;
                amount--;
            }
        } else if (zonesInfo[i].owner == myId) {
            if(myZone.plat == -1) { // First owned zone.
                myZone = {id: zonesInfo[i].id, plat: zonesPlat[zonesInfo[i].id], pods: zonesInfo[i].pods[myId]};
            } else if(zonesInfo[i].pods[myId] < myZone.pods) {
                myZone = {id: zonesInfo[i].id, plat: zonesPlat[zonesInfo[i].id], pods: zonesInfo[i].pods[myId]};
            } else if(zonesInfo[i].pods[myId] == myZone.pods && zonesPlat[zonesInfo[i].id] > myZone.plat) {
                myZone = {id: zonesInfo[i].id, plat: zonesPlat[zonesInfo[i].id], pods: zonesInfo[i].pods[myId]};
            }
            //Search for empty links.
            var linksCount = listLinks[zonesInfo[i].id].length;
            var emptyLinks = [];
            for(var j = 0; j < linksCount; ++j) {
                var destId = listLinks[zonesInfo[i].id][j];
                if(zonesInfo[destId].owner != myId) {
                    emptyLinks.push({id: destId, plat: zonesPlat[destId]});
                }
            }
            //TODO: Improve prioritization by defense
            emptyLinks.sort(function(a, b) { return (a.plat > b.plat); });
            while(zonesInfo[i].pods[myId] >  0) {
                var destMove;
                if(emptyLinks.length > 0) {
                    //move to an empty position.
                    destMove = emptyLinks.pop().id;
                } else {
                    //move it anywhere :p.
                    // TODO!! If there is nothing to in range do not move anymore. 
                    destMove = listLinks[zonesInfo[i].id][Math.floor(Math.random() * listLinks[zonesInfo[i].id].length)];
                }
                var moveStr = '1 ' + zonesInfo[i].id + ' ' + destMove;
                move = (move === 'WAIT')?moveStr:move + ' ' + moveStr;
                zonesInfo[i].pods[myId]--;
            }
        }
    }
    //Reinforcements go to most profitable zone. // maybe in a list not all to the same place.
    if(amount > 0) {
        out = (out === 'WAIT')?amount + ' ' + myZone.id:out + ' ' + amount + ' ' + myZone.id;
    }

    // Write an action using print()
    // To debug: printErr('Debug messages...');

    if(firstTurn && playerCount > 2) {
        firstTurn = false;
        print('WAIT');
        print('WAIT');
    } else {
        print(move); // first line for movement commands, second line for POD purchase (see the protocol in the statement for details)
        print(out);
    }
}
