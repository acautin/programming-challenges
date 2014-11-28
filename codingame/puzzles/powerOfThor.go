package main

import "fmt"
//import "os"

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

func main() {
    // LX: the X position of the light of power
    // LY: the Y position of the light of power
    // TX: Thor's starting X position
    // TY: Thor's starting Y position
    var LX, LY, TX, TY int
    fmt.Scan(&LX, &LY, &TX, &TY)
    
    destY := LY - TY
    destX := LX - TX
    
    for {
        // E: The level of Thor's remaining energy, representing the number of moves he can still make.
        var E int
        fmt.Scan(&E)
        
        moveStr := ""
        
        // fmt.Fprintln(os.Stderr, "Debug messages...")
        if destY > 0 {
            moveStr += "S"
            destY--
        } else if destY < 0 {
            moveStr += "N"
            destY++
        }
        
        if destX > 0 {
            moveStr += "E"
            destX--
        } else if destX < 0 {
            moveStr += "W"
            destX++
        }
        fmt.Println(moveStr) // A single line providing the move to be made: N NE E SE S SW W or NW
    }
}
