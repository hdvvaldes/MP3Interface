module Domain where

data TokenType 
  = LP
  | RP
  | VAR
  | DISY
  | CONJ
  | NEG
  deriving(Eq)

type Lexeme = String

data Token = Token 
  { ttype :: TokenType, 
    lexeme :: Lexeme
  } deriving(Eq)

