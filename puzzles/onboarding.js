while(true) {
    var enemies = [];
    var input = readline();
    var enemiLine = "";
    printErr('The input ' + input);
    // The number of current enemy ships within range
    var count = parseInt(input);
    for (var i = 0; i < count; i++) {
        enemyLine = readline();
        printErr('Enemy ' + enemyLine)
        var enemyNameDist = enemyLine.split(' ');
        var name = enemyNameDist[0];
        var distance = parseInt(enemyNameDist[1]);
        enemies.push({name: name, distance: distance});
    }
    enemies.sort(function(a, b) {return a.distance > b.distance; });
    print(enemies[0]);
}
 