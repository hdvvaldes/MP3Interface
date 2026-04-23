module Main where

import System.Environment (getArgs)
import Controller (runApp)

main :: IO ()
main = getArgs >>= runApp
