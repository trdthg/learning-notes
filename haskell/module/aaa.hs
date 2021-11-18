data Vector a = Vector a a a deriving (Show)

vplus :: (Functor t) => Vector t -> Vector t -> Vector t
(Vector i j k) `vplus` (Vector l m n) = Vector (i++l) (j++m) (k++n)

main = do
	putStrLn "start"
	putStrLn $ show $ (Vector "a" "b" "c") `vplus` (Vector "1" "2" "3")
