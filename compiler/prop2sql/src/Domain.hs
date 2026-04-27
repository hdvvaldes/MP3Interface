{-# LANGUAGE OverloadedStrings #-}

module Domain(
  Prop(..),
  SQLTable,
  SQLQuery
)where

import Data.Text as Text
type SQLTable = Text
type SQLQuery= Text

data Prop
  = VAR Text
  | DISY Prop Prop
  | CONJ Prop Prop
  | NEG Prop
  deriving(Eq)


