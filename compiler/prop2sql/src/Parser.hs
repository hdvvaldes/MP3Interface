{-# LANGUAGE OverloadedStrings #-}

module Parser
(
  Parser,
  parseInput,
) where 

import Text.Megaparsec (Parsec, MonadParsec (takeWhile1P, takeWhileP), empty, between, (<|>), optional)
import qualified Text.Megaparsec.Char.Lexer as L
import Data.Char (isAlpha, isAlphaNum)
import qualified Data.Text as T
import Domain (SQLTable, Prop(..))
import Data.Void (Void)
import Text.Megaparsec.Char (space1, string')

-- S -> (P) | P
-- P -> VB | neg S
-- B -> AND S | OR S | e*
-- V -> _identifier_

-- identifier : ( letter | _ ) (letter | num | _)+

type Parser = Parsec Void T.Text

sc :: Parser ()
sc = L.space space1 empty empty

symbol :: T.Text -> Parser T.Text
symbol = L.symbol sc

lexeme :: Parser a -> Parser a
lexeme = L.lexeme sc

parseInput :: Parser (SQLTable, Prop)
parseInput = do
  table <- parseID
  _ <- symbol ":"
  prop <- parseProp
  return (table,prop)

parseID :: Parser T.Text
parseID = lexeme $ do
    firstPart <- takeWhile1P (Just "letter or underscore") 
      (\c -> isAlpha c || c == '_')
    restPart  <- takeWhileP (Just "alphanumeric or underscore") 
      (\c -> isAlphaNum c || c == '_')
    return (firstPart <> restPart)

parseProp :: Parser Prop
parseProp = parens parseTerm <|> parseTerm

parseTerm :: Parser Prop
parseTerm = do
    left <- parseNeg <|> parseVar
    maybeOp <- lexeme $ optional (parseAND <|> parseOR)
    case maybeOp of 
      Nothing -> return left
      Just op -> op left <$> parseProp

parseVar :: Parser Prop
parseVar = VAR <$> parseID

parseNeg :: Parser Prop
parseNeg = string' "NEG" >> NEG <$> parseProp

parseAND :: Parser (Prop -> Prop -> Prop)
parseAND = string' "AND" >> return CONJ

parseOR :: Parser (Prop -> Prop -> Prop)
parseOR  = string' "OR" >> return DISY

parens :: Parser a -> Parser a
parens = between (symbol "(") (symbol ")")


