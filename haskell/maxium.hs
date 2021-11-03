main = do maximum' :: (Ord a) => [a] -> a
		  maximum' [] = error "maximum of empty list"
		  maximum' [x] = x
		  maximum' (x:xs)
			  | x > maximum' xs = x
			  | otherwise = maximum' xs
	
		  putStrLn (show (maximum' [1, 2, 3]))
