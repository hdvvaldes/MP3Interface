{-# LANGUAGE OverloadedStrings #-}

module Parser
(
  Parser
) where 

import Text.Megaparsec (Parsec, MonadParsec (takeWhile1P, takeWhileP), (<|>))
import qualified Text.Megaparsec.Char.Lexer as L
import Data.Char (isAlpha, isAlphaNum)
import Data.Text as T
import Domain (SQLTable, Prop(DISY, VAR))
import Data.Void (Void)
import Text.Megaparsec.Char (space1)

type Parser = Parsec Void Text

parse :: Text
parse = undefined

parseInput :: Parser (SQLTable, Prop)
parseInput = do
  table <- parseTable
  _ <- symbol ":"
  prop <- parseProp
  return (table,prop)

sc :: Parser ()
sc = L.space space1 empty empty

symbol :: T.Text -> Parser T.Text
symbol = L.symbol sc

parseTable :: Parser SQLTable
parseTable = do
    firstPart <- takeWhile1P (Just "letter or underscore") 
      (\c -> isAlpha c || c == '_')
    restPart  <- takeWhileP (Just "alphanumeric or underscore") 
      (\c -> isAlphaNum c || c == '_')
    return (firstPart <> restPart)

parseProp :: Parser Prop
parseProp = do 
    left <- parseTerm
    right <- parseTerm
    return $ DISY left right

-- | Return a constructor for the parsed Prop
identifyOp :: Parser Prop
identifyOp = do 
    some <- foldMap symbol ["AND", "OR"]
    return $ VAR "some"
    
parseTerm :: Parser Prop
parseTerm = undefined

parseDisy :: Parser (Prop -> Prop -> Prop)
parseDisy = do 
    some <- symbol "OR" <|> symbol "->" 
    return DISY


parseConj :: Parser Prop
parseConj = do
    left <- parseTerm
    _ <- symbol "AND"
    right <- parseTerm
    return $ CONJ left right

  

