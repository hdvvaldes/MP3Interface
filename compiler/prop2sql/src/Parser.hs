{-# LANGUAGE OverloadedStrings #-}

module Parser
(
  Parser,
  parseInput,
) where 

import Text.Megaparsec (Parsec, MonadParsec (takeWhile1P, takeWhileP), empty, between, (<|>))
import qualified Text.Megaparsec.Char.Lexer as L
import Data.Char (isAlpha, isAlphaNum)
import qualified Data.Text as T
import Domain (SQLTable, Prop(DISY, VAR))
import Data.Void (Void)
import Text.Megaparsec.Char (space1, string)

type Parser = Parsec Void T.Text

parseInput :: Parser (SQLTable, Prop)
parseInput = do
  table <- parseID
  _ <- symbol ":"
  prop <- parseProp
  return (table,prop)

sc :: Parser ()
sc = L.space space1 empty empty

symbol :: T.Text -> Parser T.Text
symbol = L.symbol sc

parseID :: Parser T.Text
parseID = do
    firstPart <- takeWhile1P (Just "letter or underscore") 
      (\c -> isAlpha c || c == '_')
    restPart  <- takeWhileP (Just "alphanumeric or underscore") 
      (\c -> isAlphaNum c || c == '_')
    return (firstPart <> restPart)

parseProp :: Parser Prop
parseProp = parens parseProp <|> parseVar

parens :: Parser a -> Parser a
parens = between (string "(") (string ")")

parseVar :: Parser Prop
parseVar = VAR <$> parseID

-- | Return a constructor for the parsed Prop
identifyOp :: Parser Prop
identifyOp = do 
    some <- foldMap symbol ["AND", "OR"]
    return $ VAR "some"

