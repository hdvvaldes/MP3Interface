module Controller (
    App,
    UserInterface(..),
    runApp
) where

import Control.Monad.Trans.Reader (ReaderT, runReaderT, ask)
import Control.Monad.IO.Class (liftIO)
import System.Environment (getArgs)
import Compiler (prop2sql)
import qualified Data.Text as T

data UserInterface = TUI

-- 2. Fixed the App type to take a return type 'a'
type App a = ReaderT UserInterface IO a

-- 3. Created the core logic inside the App monad
printUserArgs :: App ()
printUserArgs = do
    args <- liftIO getArgs
    let t_args = map T.pack args
    liftIO $ print (prop2sql t_args)

-- 4. runApp kicks off the ReaderT execution
runApp :: UserInterface -> IO ()
runApp = runReaderT printUserArgs
