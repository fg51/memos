set encoding=utf-8
scriptencoding utf-8
set fileencoding=utf-8 " 保存時の文字コード
set fileencodings=ucs-boms,utf-8,euc-jp,cp932 " 読み込み時の文字コードの自動判別. 左側が優先される
set fileformats=unix,dos,mac " 改行コードの自動判別. 左側が優先される
set ambiwidth=double " □や○文字が崩れる問題を解決

""""""""""""""""""""""""""""""
" 各種オプションの設定
""""""""""""""""""""""""""""""
" タグファイルの指定(でもタグジャンプは使ったことがない)
set tags=~/.tags

" スワップファイルは使わない(ときどき面倒な警告が出るだけで役に立ったことがない)
set noswapfile

" バックアップファイル出力無効
set nobackup

" undoファイルは作成しない
set noundofile

" カーソルが何行目の何列目に置かれているかを表示する
set ruler

" コマンドラインに使われる画面上の行数
set cmdheight=2

" エディタウィンドウの末尾から2行目にステータスラインを常時表示させる
set laststatus=2

" ステータス行に表示させる情報の指定(どこからかコピペしたので細かい意味はわかっていない)
set statusline=%<%f\ %m%r%h%w%{'['.(&fenc!=''?&fenc:&enc).']['.&ff.']'}%=%l,%c%V%8P

" ステータス行に現在のgitブランチを表示する
" set statusline+=%{fugitive#statusline()}
" ウインドウのタイトルバーにファイルのパス情報等を表示する
set title

" コマンドラインモードで<Tab>キーによるファイル名補完を有効にする
set wildmenu

" 入力中のコマンドを表示する
set showcmd

" バックアップする
set backup
set writebackup

" 小文字のみで検索したときに大文字小文字を無視する
set smartcase

" 検索結果をハイライト表示する
set hlsearch

" ESCキー２度押しでハイライトの切り替え
nnoremap <silent><Esc><Esc> :<C-u>set nohlsearch!<CR>

" タブ入力を複数の空白入力に置き換える
set expandtab

" 検索ワードの最初の文字を入力した時点で検索を開始する
set incsearch

" 保存されていないファイルがあるときでも別のファイルを開けるようにする
set hidden

" 不可視文字を表示する
set list

" タブと行の続きを可視化する
set listchars=tab:>\ ,extends:<

" 行番号を表示する
set number

" 対応する括弧やブレースを表示する
set showmatch
set matchtime=1
set matchpairs& matchpairs+=<:>
source $VIMRUNTIME/macros/matchit.vim " Vimの「％」を拡張する

" 改行時に前の行のインデントを継続する
set autoindent

" 改行時に入力された行の末尾に合わせて次の行のインデントを増減する
set smartindent

" タブ文字の表示幅
set tabstop=2

" Vimが挿入するインデントの幅
set shiftwidth=2

" 行頭の余白内で Tab を打ち込むと、'shiftwidth' の数だけインデントする
set smarttab

" カーソルを行頭、行末で止まらないようにする
set whichwrap=b,s,h,l,<,>,[,]

" 構文毎に文字色を変化させる
syntax on

" クラッシュ防止（http://superuser.com/questions/810622/vim-crashes-freezes-on-specific-files-mac-osx-mavericks）
set synmaxcol=200

" バックスペースを有効化
set backspace=indent,eol,start

" 全角文字の表示
set ambiwidth=double

" ビープ音を視覚表示
set visualbell
nnoremap j gj
nnoremap k gk
nnoremap <down> gj
nnoremap <up> gk
