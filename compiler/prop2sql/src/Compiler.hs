module Compiler(
  prop2sql
)where

import Text.Megaparsec (Parsec, many)
import Data.Text as T
import Data.Void (Void)
import Domain (SQLTable, Prop)
import Text.Megaparsec.Char
import Control.Applicative((<|>))

type Parser = Parsec Void Text

prop2sql :: Text -> Text
prop2sql prop = undefined

parse :: Text
parse = undefined

parseTable :: Parser SQLTable
parseTable = do
  firstChar <- letterChar <|> char '_'
  restChars <- many (alphaNumChar <|> char '_')
  return $ T.pack (firstChar : restChars)

parseProp :: Parser Prop
parseProp = undefined
  
