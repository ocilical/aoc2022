main = do
    input <- readFile "input"
    putStr "part 1: "
    print $ sum $ map toPoints1 $ lines input
    putStr "part 2: "
    print $ sum $ map toPoints2 $ lines input


toPoints1 :: Num a => String -> a
toPoints1 "A X" = 4
toPoints1 "A Y" = 8
toPoints1 "A Z" = 3
toPoints1 "B X" = 1
toPoints1 "B Y" = 5
toPoints1 "B Z" = 9
toPoints1 "C X" = 7
toPoints1 "C Y" = 2
toPoints1 "C Z" = 6
toPoints1 _ = 0

toPoints2 :: Num a => String -> a
toPoints2 "A X" = 3
toPoints2 "A Y" = 4
toPoints2 "A Z" = 8
toPoints2 "B X" = 1
toPoints2 "B Y" = 5
toPoints2 "B Z" = 9
toPoints2 "C X" = 2
toPoints2 "C Y" = 6
toPoints2 "C Z" = 7
toPoints2 _ = 0
