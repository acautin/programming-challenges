var inputs = readline().split(' ');
var nbFloors = parseInt(inputs[0]);
var width = parseInt(inputs[1]);
var nbRounds = parseInt(inputs[2]);
var exitFloor = parseInt(inputs[3]); 
var exitPos = parseInt(inputs[4]);
var nbTotalClones = parseInt(inputs[5]);
var nbAdditionalElevators = parseInt(inputs[6]);
var nbElevators = parseInt(inputs[7]);

printErr('number of elevators...', nbElevators);

var elevatorPos = [];
for (var i = 0; i < nbElevators; i++) {
    var inputs = readline().split(' ');
    var elevatorFloor = parseInt(inputs[0]);
    elevatorPos[elevatorFloor] = parseInt(inputs[1]);
}
var currentFloor = 0;
while (true) {
    var inputs = readline().split(' ');
    var cloneFloor = parseInt(inputs[0]);
    var clonePos = parseInt(inputs[1]);
    var direction = inputs[2];
    var cmd = "WAIT";
    if(cloneFloor == currentFloor) {
        var targetPos = (exitFloor == cloneFloor)? exitPos: elevatorPos[cloneFloor];
        if((direction == "RIGHT" && clonePos > targetPos) ||
            (direction == "LEFT" && clonePos < targetPos)) {
            cmd = "BLOCK";
        }
        currentFloor++;
    }
    print(cmd);
}
