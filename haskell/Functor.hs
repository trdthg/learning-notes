-- 看一眼fmap的定义
-- class Functor f where
--     fmap :: (a -> b) -> f a -> f b
-- 这个是map的定义
-- map :: (a -> b) -> [a] -> [b]
-- map就是特殊化的fmap,只能用于[],不能用于Maybe,Ehther等类型
-- anything like a box that can hold something can impl Functor, lsuch as Maybe, [], Tree = Empty | Tree a left rght, Either
-- 所有能够承载其他类型的东西都能实现Functor,比如Maybe, [], Tree, Either

-- 下面是关于Either的Functor实现
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
-- 在这个例子里,我们只map了Either的右值,因为不能保证Either a b的类型相同, Map.Map也是,只有value改变

class Tofu t where
  tofu :: j a -> t a j

newtype Frank a b = Frank {frankField :: b a} deriving (Show)

instance Tofu Frank where
  tofu x = Frank x

data Barry t k p = Barry {yabba :: p, dabba :: t k}

instance Functor (Barry a b) where
  fmap f Barry {yabba = x, dabba = y} = Barry {yabba = f x, dabba = y}

main :: IO ()
main = do
  let a = Frank {frankField = Just 'c'}
  print a
  let b = tofu (Just 'a') :: Frank Char Maybe
  print b
  let c = tofu ["Hello"] :: Frank [Char] []
  print c
  putStrLn "start..."
