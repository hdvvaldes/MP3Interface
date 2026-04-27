module Controller (
    runApp
) where

import Compiler (prop2sql)
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Data.Either (partitionEithers)
import System.Exit (exitWith, ExitCode (ExitFailure), exitSuccess)

runApp :: [String] -> IO ()
runApp arg = do 
  let (errors, successes) = partitionEithers $ map (prop2sql . T.pack) arg 
  mapM_ putStrLn errors
  mapM_ TIO.putStrLn successes
  if null errors 
    then exitSuccess
    else exitWith $ ExitFailure 2
