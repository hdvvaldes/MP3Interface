{-# LANGUAGE OverloadedStrings #-}

module Compiler(
  prop2sql
)where

import qualified Data.Text as T
import Parser (parseInput)
import Domain (Prop(..), SQLTable, SQLQuery)
import Text.Megaparsec (parse, errorBundlePretty)

prop2sql :: T.Text -> Either String SQLQuery
prop2sql input = 
  case parse parseInput "prop2sql: " input of 
    Left err -> Left $ errorBundlePretty err
    Right val -> Right $ uncurry genSQL val

genSQL :: SQLTable -> Prop -> SQLQuery
genSQL table prop =
  "SELECT * FROM " <> table <> " WHERE " <> genProp prop <> ";" 

genProp :: Prop -> T.Text
genProp (VAR name)   = name
genProp (DISY p1 p2) = "(" <> genProp p1 <> " OR " <> genProp p2 <> ")"
genProp (CONJ p1 p2) = "(" <> genProp p1 <> " AND " <> genProp p2 <> ")"
genProp (NEG p)      = "NOT " <> genProp p
