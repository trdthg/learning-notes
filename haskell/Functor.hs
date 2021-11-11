
-- fmap'd define
-- class Functor f where
--     fmap :: (a -> b) -> f a -> f b
-- map's define
-- map :: (a -> b) -> [a] -> [b]

-- anything like a box that can hold something can impl Functor, lsuch as Maybe, [], Tree = Empty | Tree a left rght, Either

-- instance Functor (Either a) where
--     fmap f (Right x) = Right (f x)
--     fmap f (Left x) = Left x
-- (b -> c) -> Either a b -> Ether a c
-- same as next
-- (b -> c) -> (Either a) b -> (Either a) c
-- in this case, we only mapped the right value constructor,
-- well, if we look at the define of Either:
-- data Either a b = Left a | Right b
-- we can't make sure f can handle both type a and type b
-- Another example is Map.Map, where fmap just map a function v -> v' over a Map k v, and return Map k v'

class Tofu t where
    tofu :: j a -> t a j

main :: IO ()
main = do

    putStrLn "start..."
