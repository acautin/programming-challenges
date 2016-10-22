package main
import "fmt"
func main() {
var LX, LY, TX, TY int
fmt.Scan(&LX, &LY, &TX, &TY)
Y := LY - TY
X := LX - TX
for {var E int
fmt.Scan(&E)        
S := ""
if Y > 0 {S += "S"
Y--
}
if Y < 0 {S += "N"
Y++
}
if X > 0 {S += "E"
X--
}
if X < 0 {
S += "W"
X++
}
fmt.Println(S)
}
}
