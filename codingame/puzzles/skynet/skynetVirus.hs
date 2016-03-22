import System.IO
import Control.Monad
import Data.Graph

main :: IO()
main = do
    hSetBuffering stdout NoBuffering -- DO NOT REMOVE
    
    -- Auto-generated code below aims at helping you parse
    -- the standard input according to the problem statement.
    
    input_line <- getLine
    let input = words input_line
    let n = read (input!!0) :: Int -- the total number of nodes in the level, including the gateways
    let l = read (input!!1) :: Int -- the number of links
    let e = read (input!!2) :: Int -- the number of exit gateways

    links <- replicateM l $ do
        input_line <- getLine
        let input = words input_line
        let n1 = read (input!!0) :: Int -- N1 and N2 defines a link between these nodes
        let n2 = read (input!!1) :: Int
        return ((n1, n2))

    gateways <- replicateM e $ do
        input_line <- getLine
        let ei = read input_line :: Int -- the index of a gateway node
        return (ei)
    
    loop links gateways

loop :: [(Int, Int)] -> [Int] -> IO()
loop links gateways = do
    input_line <- getLine
    let pos = read input_line :: Int -- The index of the node on which the Skynet agent is positioned this turn

    --print links
    --print gateways
    -- hPutStrLn stderr "Debug messages..."
    case (cut links links gateways pos) of
        (rest, (-1, -1)) -> do
            let (newrest, (c, d)) = sever links pos
            putStrLn (concat [show c, " ", show d])
            loop newrest gateways 
        (rest, (a, b)) -> do
            putStrLn (concat [show a, " ", show b])
            loop rest gateways 


sever :: [(Int, Int)] -> Int -> ([(Int, Int)], (Int, Int))
sever ((a, b):rest) c
    | a == c = (rest, (a, b))
    | b == c = (rest, (a, b))
sever (l:rest) pos = do
    let (newrest, k) = sever rest pos
    (l:newrest, k)

cut :: [(Int, Int)] -> [(Int, Int)] -> [Int] -> Int -> ([(Int, Int)], (Int, Int))
cut ((a, b):rest) links gateways c
    | a == c && elem b gateways = (rest, (a, b))
    | b == c && elem a gateways = (rest, (a, b))
cut (l:[]) links gateways c = ([l], (-1, -1))
cut (l:rest) links gateways pos = do
    let (newrest, k) = cut rest links gateways pos
    (l:newrest, k)

getLinks :: Int -> [(Int, Int)] -> IO [(Int, Int)]
getLinks 0 result = return result
getLinks n partial = do
    input_line <- getLine
    let input = words input_line
    let n1 = read (input!!0) :: Int -- N1 and N2 defines a link between these nodes
    let n2 = read (input!!1) :: Int
    return ((n1, n2):partial)
