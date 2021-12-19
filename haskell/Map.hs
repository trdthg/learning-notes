-- 第一种
findKey :: (Eq k) => k -> [(k, v)] -> v
findKey key = snd . head . filter (\(k, v) -> key == k)

-- 第二种 Maybe非空校验
findKey2 :: (Eq k) => k -> [(k, v)] -> Maybe v
findKey2 _ [] = Nothing
findKey2 key ((k, v) : xs) =
  if k == key
    then Just v
    else findKey2 key xs

-- 第三种
findKey3 :: (Eq k) => k -> [(k, v)] -> Maybe v
findKey3 key = foldr (\(k, v) acc -> if key == k then Just v else acc) Nothing

main :: IO ()
main = do
  let phoneBook =
        [ ("aaa", "111"),
          ("bbb", "222"),
          ("ccc", "333")
        ]
  print $ findKey "bbb" phoneBook
  print $ findKey2 "bbb" phoneBook
  print $ findKey3 "bbb" phoneBook
