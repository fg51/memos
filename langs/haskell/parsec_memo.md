parsec3
====


## ParsecT

parser's type: ParsecT
ParsecT is monad 変換子

ParsecT s u m a

s 抽象化された入力の型。(この抽象化された入力をストリームという。)
u パース時に好きな状態を格納しておく容器の型。
m モナド変換子にとって基盤となるモナド。
a 出力の型


文字列から文字列を探して返すパーサ
Monad m => ParsecT String u m String

```haskell
import Control.Applicative hiding ((<|>))
import Text.Parsec

parser :: Monad m => ParsecT String u m String
parser = (++) <$> string "HT" <*> (string "TP" <|> string "ML")
```


with Test.Parsec.String

```haskell
import Control.Applicative hiding ((<|>))
import Text.Parsec
import Text.Parsec.String

parser :: Parser String
parser = (++) <$> string "HT" <*> (string "TP" <|> string "ML")
```


