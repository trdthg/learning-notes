main = do print (maximum' [1, 2, 3])

maximum' :: (Ord a) => [a] -> a
maximum' [] = error "maximum of empty list"
maximum' [x] = x
maximum' (x : xs)
  | x > maximum' xs = x
  | otherwise = maximum' xs