module Controller (
    runApp
) where

import Compiler (prop2sql)
import qualified Data.Text as T

runApp :: [String] -> IO()
runApp arg = print $ 
  map (prop2sql . T.pack) arg

