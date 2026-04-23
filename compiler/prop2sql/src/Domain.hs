module Domain(
  Prop,
  SQLTable
)where

import Data.Text as Text
type SQLTable = Text

data Prop
  = VAR Text
  | DISY Prop Prop
  | CONJ Prop Prop
  | NEG Prop
  deriving(Eq)


