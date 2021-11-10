module Module1
( sum2
, add
, getStr
) where

sum2 :: Num a => [a] -> a
sum2 [] = 0
sum2 (x:xs) = x + (sum2 xs)

add ::  a -> a -> a
add a b = a `add` b

getStr :: [Char]
getStr = "this is a string"
