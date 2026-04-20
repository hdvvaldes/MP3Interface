module UnitTests (unitTests) where

import Test.Tasty
import Test.Tasty.HUnit
-- import Parser (parseProp) -- Import your parser or helpers

unitTests :: TestTree
unitTests = testGroup "Unit Tests"
  [ testCase "Basic Proposition parses correctly" $ do
      -- Replace with your actual parser logic:
      -- let result = parseProp "A"
      -- result @?= Right (Var "A")
      (1 + 1) @?= (2 :: Int)
  , testCase "Fails on empty input" $ do
      -- assertBool "Expected error" (isLeft (parseProp ""))
      True @?= True
  ]



