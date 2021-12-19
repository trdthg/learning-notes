import Data.List

solveRPN :: (Num a, Read a) => String -> a
solveRPN = head . foldl foldingFunction [] . words
    where
        foldingFunction (x:y:ys) "*" = (x * y):ys
        foldingFunction (x:y:ys) "+" = (x + y):ys
        foldingFunction (x:y:ys) "-" = (x - y):ys
        foldingFunction xs numString = read numString:xs

solveRPN2 :: (Num a, Read a, Floating a) => String -> a
solveRPN2 = head . foldl foldingFunction [] . words
    where
        foldingFunction (x:y:ys) "*" = (x * y):ys
        foldingFunction (x:y:ys) "+" = (x + y):ys
        foldingFunction (x:y:ys) "-" = (x - y):ys
        foldingFunction (x:y:ys) "/" = (y / x):ys
        foldingFunction (x:y:ys) "^" = (y ** x):ys
        foldingFunction (x:xs) "ln" = log x:xs
        foldingFunction xs "sum" = [sum xs]
        foldingFunction xs numString = read numString:xs


main :: IO ()
main = do
    putStrLn "start..."
    print $ solveRPN "90 34 12 33 55 66 + * - + -"
    putStrLn "end..."
