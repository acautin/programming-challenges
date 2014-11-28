package main

import "fmt"
//import "os"

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

type pos struct {
	x, y int
}

func main() {
	fired := false;
	target := 0;
    for {
        var shipPos pos
        fmt.Scan(&shipPos.x, &shipPos.y)
        mountainHeights := make([]int, 8)

		if shipPos.x == 0 || shipPos.x == 7 {
			fired = false;
		}

        for i := 0; i < 8; i++ {
            // Height of one mountain, from 9 to 0.
            fmt.Scan(&mountainHeights[i])
			if mountainHeights[i] > mountainHeights[target] {
				target = i
			}
        }
        
        if shipPos.x == target && !fired {
			fmt.Println("FIRE")
			fired = true;
		} else {
			fmt.Println("HOLD")
		}
    }
}
