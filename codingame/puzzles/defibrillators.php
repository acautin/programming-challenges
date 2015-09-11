<?php
/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

fscanf(STDIN, "%s", $long);
fscanf(STDIN, "%s", $lat);
fscanf(STDIN, "%d", $n);

$long = str_replace(",", ".", $long);
$lat = str_replace(",", ".", $lat);

$result = "";
$minDist = INF;

for ($i = 0; $i < $n; $i++) {
    $defLine = stream_get_line(STDIN, 256, "\n");
	$defarr = explode(";", $defLine);
	error_log(var_export($defarr, true));
	$defLong = str_replace(",", ".", $defarr[4]);
	$defLat = str_replace(",", ".", $defarr[5]);
	$x = ($defLong - $long) * cos(($lat + $defLat)/2);
	$y = ($defLat -$lat);
	$dist = sqrt(pow($x, 2) + pow($y, 2))*6371;
	if ($dist < $minDist) {
		$result = $defarr[1];
		$minDist = $dist;
	}
}

/*
x = (lb - la) * cos((la + lb)/2)
	y = (lb -la)
	d = sqrt(pow(x, 2) + pow(y, 2))*6371
*/

// Write an action using echo(). DON'T FORGET THE TRAILING \n
// To debug (equivalent to var_dump): error_log(var_export($var, true));

echo("$result\n");
?>