" Vim Syntax for tr-lang
" Language: tr-lang
" Maintainer: Kerem Göksu <superkerem13@gmail.com>
" Latest Revision: 26 December 2021

" Usage:
" move this file into .vim/syntax/tr-lang.vim
" for automatic recognition of .trl files as tr-lang
" add this to your vimrc
" au BufRead,BufNewFile *.trl set filetype=tr-lang

" Note For Neovim Users:
" first find your runtime
" in neovim:
" :echo $VIMRUNTIME
" instead of .vim/syntax/tr-lang.vim
" put your file in $VIMRUNTIME/syntax/tr-lang.vim
" for automatic recognition you need to put the following line in ~/.config/nvim/init.vim
" au BufRead,BufNewFile *.trl set filetype=tr-lang

if exists("b:current_syntax")
    finish
endif

" Keywords
syn keyword trlKeyword at ver de ise son iken yoksa kpy tks üst veya ve dön girdi işlev yükle

" Booleans
syn keyword trlBoolean doğru yanlış

" Numbers
syn match trlNumber '\v(\d+\.\d*|\d+)'

" Types that come after `@`
syn match trlType '@\@<=[^\t\r \n\"\'\:\?\=<>\!\/\%\*\@,\d-][^\t\r \n\"\'\:\?\=<>\!\/\%\*\@,]*'

" Some of the single character operators no minus(-)
syn match trlSingleCharOp '\v[\@\+\*\%\>=!\<]'
syn match trlMinus '\v(\s|^)@<=-'

" Comments
"" Line Comment
syn region trlComment start="#" end="$"
"" Block Comment
syn region trlComment start="-\*" end="\*-"

" Strings
"" Double-Quoted String
syn region trlString start=/\v"/ skip=/\v\\[tnr"'\\\n\t]/ end=/\v"/
"" Single-Quoted String
syn region trlString start=/\v'/ skip=/\v\\[tnr"'\\\n\t]/ end=/\v'/

hi def link trlBoolean      Boolean
hi def link trlKeyword      Keyword
hi def link trlComment      Comment
hi def link trlNumber       Number
hi def link trlString       String
hi def link trlSingleCharOp Operator
hi def link trlType         Type
hi def link trlMinus        Operator

let b:current_syntax = "tr-lang"
