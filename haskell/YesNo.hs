class YesNo a where
    yesno :: a -> Bool

instance YesNo Int where
    yesno 0 = False
    yesno _ = True

instance YesNo [a] where
    yesno [] = False
    yesno _ = True

-- Bool 也不要忘了
instance YesNo Bool where
    yesno = id

-- 因为我们并不关心Maybe 包含什么类型，只要他有东西就行了，所以没有加类型限定
instance YesNo (Maybe a) where
    yesno (Just _) = True
    yesno Nothing = False

-- 仿照着写一个 if
yesnoIf :: (YesNo a) => a -> b -> b -> b
yesnoIf a onyes onno = if yesno a then onyes else onno

main :: IO ()
main = do
    putStrLn "start..."
    print $ length []
    print $ yesno [0]
    print $ yesno Nothing
    print $ yesno True
    putStrLn $ yesnoIf (Just 1) "yes !" "No -_-"
