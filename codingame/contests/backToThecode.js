var opponentCount = parseInt(readline());
var complete = false;
var target, last, complete;

while (true) {
    var gameRound = parseInt(readline());
    var inputs = readline().split(' ');
    var x = parseInt(inputs[0]); // Your x position
    var y = parseInt(inputs[1]); // Your y position
    var backInTimeLeft = parseInt(inputs[2]); // Remaining back in time
    for (var i = 0; i < opponentCount; ++i) {
        var inputs = readline().split(' ');
        var opponentX = parseInt(inputs[0]);
        var opponentY = parseInt(inputs[1]);
        var opponentBackInTimeLeft = parseInt(inputs[2]);
    }
    var map = [];
    for (var i = 0; i < 20; ++i) {
        map.push(readline());
        /*for(var j = 0; j < line.length; ++j) {
            
        }*/ // One line of the map ('.' = free, '0' = you, otherwise the id of the opponent)
    }

    // Write an action using print()
    // To debug: printErr('Debug messages...');

    if(gameRound == 1) {
        target = {};
        target.x = (x < 17)? 0 : 34;
        target.y = (y < 10)? 0 : 19;
        last = calculateLast(target, {x: x, y: y});
    } else if(target.x === x && target.y === y) {
        if(last.x === x && last.y === y) {
            var maxfree = -1;
            for(var i = 0; i < gameRound * 2 + 50; ++i) {
                var x1 = Math.floor(Math.random() * 35);
                var y1 = Math.floor(Math.random() * 20);
                var dist = Math.abs(x1 - x) + Math.abs(y1 - y);
                var newfree = calculateFree(map, x1, y1, x, y) / dist;
                if(newfree > maxfree) {
                    target.x = x1;
                    target.y = y1;
                    maxfree = newfree;
                }
            }
        } else {
            if(shouldComplete(map, x, y, last.x, last.y)) {
                complete = true;
                target.x = last.x;
                target.y = last.y;
            } else {
                complete = false;
                var maxfree = -1;
                for(var i = 0; i < gameRound * 2 + 50; ++i) {
                    var x1 = Math.floor(Math.random() * 35);
                    var y1 = Math.floor(Math.random() * 20);
                    var dist = Math.abs(x1 - x) + Math.abs(y1 - y);
                    var newfree = calculateFree(map, x1, y1, x, y) / dist;
                    if(newfree > maxfree) {
                        target.x = x1;
                        target.y = y1;
                        maxfree = newfree;
                    }
                }
                last = calculateLast(target, {x: x, y: y});
            }
        }
    }

    if(gameRound == 100 && backInTimeLeft > 0 && countPoints(map) < 50) {
        print("BACK 25");
    } else {
        if(x === target.x) {
            print(target.x + ' ' + target.y + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
        } else {
            if(target.x > x && map[y][x+1] !== '.') {
                if(y !== 19 && map[y+1][x] === '.') {
                    print(target.x + ' ' + (y+1) + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
                } else if(y !== 0 && map[y-1][x] === '.') {
                    print(target.x + ' ' + (y-1) + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
                } else {
                    print(target.x + ' ' + target.y + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
                }
            } else if(map[y][x-1] !== '.') {
                if(y !== 19 && map[y+1][x] === '.') {
                    print(target.x + ' ' + (y+1) + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
                } else if(y !== 0 && map[y-1][x] === '.') {
                    print(target.x + ' ' + (y-1) + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
                } else {
                    print(target.x + ' ' + target.y + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
                }
            } else {
                print(target.x + ' ' + target.y + ' :' + x + ' ' + y + ' ' + last.x + ' ' + last.y + ' ' + complete);
            }
        }
    }
}

function calculateFree(map, x1, y1, x, y) {
    var freeCount = 0;
    var i, j;
    if(y > y1) {
        for(i = y; i >= y1; --i) {
            if(map[i][x] == '.') {
                freeCount++;
            }
        }
    } else {
        for(i = y; i <= y1; ++i) {
            if(map[i][x] == '.') {
                freeCount++;
            }
        }
    }
    if(x > x1) {
        for(i = x; i >= x1; --i) {
            if(map[y1][i] == '.') {
                freeCount++;
            }
        }
    } else {
        for(i = x; i <= x1; ++i) {
            if(map[y1][i] == '.') {
                freeCount++
            }
        }
    }

    return freeCount;
}

function shouldComplete(map, x1, y1, x2, y2) {
    var p1 = {x: Math.min(x1, x2), y: Math.min(y1, y2)};
    var p2 = {x: Math.max(x1, x2), y: Math.max(y1, y2)};
    var empty = 0;
    for(var i = p1.x; i < p2.x; ++i) {
        for(var j = p1.y; j < p2.y; ++j) {
            if(map[j][i] === '.') {
                ++empty;
            } else if(map[j][i] !== '0') {
                return false;
            }
        }
    }
    return empty > 4;
}

function countPoints(map) {
    var points = 0;
    for(var i = 0; i < 20; ++i) {
        for(var j = 0; j < 35; ++j) {
            if(map[i][j] === '0') {
                ++points;
            }
        }
    }
    return points;
}

function calculateLast(target, current) {
    if(target.y > current.y) {
        current.y += 1;
    } else if(target.y < current.y) {
        current.y -= 1;
    }
    return current;
}
