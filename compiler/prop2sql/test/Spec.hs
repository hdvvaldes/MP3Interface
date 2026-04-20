{-# LANGUAGE OverloadedStrings #-}

import Test.Tasty
import Test.Tasty.Golden
import System.FilePath ((</>), replaceExtension)
import qualified Data.ByteString.Lazy as BSL
import qualified Data.ByteString.Lazy.Char8 as BSL8

import UnitTests (unitTests)
-- import Compiler (compileToSql) -- Import your actual compilation function here

main :: IO ()
main = defaultMain tests

tests :: TestTree
tests = testGroup "prop2sql Test Suite"
  [ unitTests
  , goldenTests
  ]

goldenTests :: TestTree
goldenTests = testGroup "Golden Tests (Integration)"
  [ -- Add your test files here (without the extensions)
    mkGoldenTest "test/integration/simple_and"
  , mkGoldenTest "test/integration/complex_nested"
  ]

-- | Helper to wire up a golden test automatically
mkGoldenTest :: FilePath -> TestTree
mkGoldenTest baseName =
  let propFile   = baseName ++ ".prop"
      goldenFile = baseName ++ ".sql.golden"
      
      -- The action that executes your compiler
      runCompiler = do
        input <- readFile propFile
        -- REPLACE THIS with your actual compiler function:
        -- let output = compileToSql input 
        let output = "SELECT * FROM dummy;\n" 
        
        return $ BSL8.pack output
        
  in goldenVsString baseName goldenFile runCompiler
