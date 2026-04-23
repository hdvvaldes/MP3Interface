{-# LANGUAGE OverloadedStrings #-}

import Test.Tasty
import Test.Tasty.HUnit
import Compiler (prop2sql)

main :: IO ()
main = defaultMain unitTests

unitTests :: TestTree
unitTests = testGroup "Unit Tests" 
  [andTest]

andTest :: TestTree 
andTest = testCase "Basic And Proposition" $ 
  prop2sql "BONES: A AND B" @?= "SELECT * FROM BONES WHERE (A AND B) OR (B AND A);"

