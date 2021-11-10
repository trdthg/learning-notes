-- 第一种
-- findKey :: (Eq k) => k -> [(k, v)] -> v
-- findKey key xs = (snd . head . filter (\ (k, v) -> key == k)) xs

-- 第二种 Maybe非空校验
findKey :: (Eq k) => k -> [(k, v)] -> Maybe v
findKey _ [] = Nothing 
findKey key ((k, v):xs) = if k == key 
							then Just v
							else findKey key xs

-- 第三种
findKey :: (Eq k) => k -> [(k,v)] -> Maybe v
findKey key xs = foldr (\(k,v) acc -> if key == k then Just v else acc) Nothing xs

main :: IO ()
main = do 
	let phoneBook = 
		[("aaa","111")
		,("bbb","222")
		,("ccc","333")
		]
	print $ findKey "bbb" phoneBook

