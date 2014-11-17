/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

var roadSize = parseInt(readline()); // the length of the road before the gap.
var gapSize = parseInt(readline()) + 1; // the length of the gap.
var landingSize = parseInt(readline()); // the length of the landing platform.
printErr('The road size: ' + roadSize);
printErr('The gap size: ' + gapSize);
printErr('The landing size: ' + landingSize);

var jump = false;

// game loop
while (true) {
    var speed = parseInt(readline()); // the motorbike's speed.
    var position = parseInt(readline()); // the position on the road of the motorbike.

    if(jump) {
        print('SLOW');
    } else if(position + speed > roadSize) {
        jump = true;
        print('JUMP')
    } else if(speed < gapSize) {
        print('SPEED');
    } else if(speed > gapSize) {
        print('SLOW');
    } else {
        print('WAIT');
    }
        // Write an action using print()
        // To debug: printErr('Debug messages...');
}