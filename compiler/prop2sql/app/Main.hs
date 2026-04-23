module Main where

import Controller (runApp, UserInterface (TUI))

main :: IO ()
main = do
    runApp TUI
