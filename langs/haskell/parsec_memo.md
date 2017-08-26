parsec3
====


## setup

```.cabal
build-depends: parsec
```

```haskell
import Text.Parsec

import Text.Parsec.String (Parser)
-- import Text.Parsec.Text (Parser)

import Control.Applicative hiding ((<|>))
```


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


## usage

### char :: Char -> Parser Char

特定の文字のみ受け付ける parser

```haskell
parseTest (char 'h') "h"    -- pass
parseTest (char 'h') "a"    -- error
```


### string :: String -> Parser String

特定の文字列のみ受け付ける parser

```haskell
parseTest (string 'abc') "abc"    -- pass
parseTest (string 'abc') "xyz"    -- error
```

### many :: Parser a -> Parser [a]

指定した parser を0回以上適用して返す parser (== reqexp's * )


### (<|>) :: Parser a -> Parser a -> Parser a

一つ目のパーサが失敗時に二つ目のパーサを実行する parser

```haskell
parseTest (string "hello" <|> string "world") "world" -- pass
```

二つ目のパーサが実行されるのは 一つ目のパーサが 入力を全く消費せずに 失敗した場合だけになります。 たとえば


#### caution

以下の場合, 前段の "abc" parser がheまで消費してしまい, heavenパーサが実行されません。
```haskell
parseTest (string "abc" <|> string "abx") "abx" -- error
```

use "try" (back-track)

```haskell
parseTest (try (string "hello") <|> string "heaven") "heaven" -- pass
```

LL(1)


### choice :: [Parser a] -> Parser a

リスト中のどれかが成功するまで解析を行う parser
