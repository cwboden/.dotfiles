filetype plugin indent on

" Vim-Plug Options
call plug#begin()

" Vim-Plug management for other apps
Plug 'FooSoft/vim-argwrap'

" Appearance
Plug 'itchyny/lightline.vim'
Plug 'morhetz/gruvbox'
Plug 'ryanoasis/vim-devicons'
Plug 'shinchu/lightline-gruvbox.vim'

" Keybindings and Navigation
Plug 'ervandew/supertab'
Plug 'christoomey/vim-tmux-navigator'

" Syntax Highlighting
Plug 'MaxMEllon/vim-jsx-pretty'
Plug 'leafgarland/typescript-vim'
Plug 'mtdl9/vim-log-highlighting'
Plug 'octol/vim-cpp-enhanced-highlight'
Plug 'peitalin/vim-jsx-typescript'
Plug 'peitalin/vim-jsx-typescript'
Plug 'ron-rs/ron.vim'
Plug 'rust-lang/rust.vim'
Plug 'sheerun/vim-polyglot'

" Search Utilities
Plug 'jremmen/vim-ripgrep'
Plug 'junegunn/fzf'
Plug 'preservim/nerdtree'

" Style Utilities
Plug 'FooSoft/vim-argwrap'
Plug 'fadein/vim-FIGlet'
Plug 'preservim/nerdcommenter'
Plug 'psf/black'

" Version Control Utilities
Plug 'Xuyuanp/nerdtree-git-plugin'
Plug 'airblade/vim-gitgutter'
Plug 'tpope/vim-fugitive'

" Finish the call, all plugins must be before this
call plug#end()

"

" Language and Encoding
let $LANG='en'
set langmenu=en
"

" General Configuration
set noshowmode

set number
set relativenumber
set cursorline

set nowrap
set so=8

set wildmode=list:longest,list:full

" Ignore specific files
set wildignore=*.o,*~,*.pyc
if has("win16") || has("win32")
  set wildignore+=.git\*,.hg\*,.svn\*
else
  set wildignore+=*/.git/*,*/.hg/*,*/.svn/*,*/.DS_Store
endif

" Automatic switching of relative numbers
augroup numbertoggle
  autocmd!
  autocmd BufEnter,FocusGained,InsertLeave,WinEnter * if &nu | set rnu | endif
  autocmd BufLeave,FocusLost,InsertEnter,WinLeave * if &nu | set nornu | endif
augroup END

" Trigger `autoread` when files changes on disk
" Relies on vim-tmux-focus-events if using vim inside tmux.
" https://unix.stackexchange.com/questions/149209/refresh-changed-content-of-file-opened-in-vim/383044#383044
" https://vi.stackexchange.com/questions/13692/prevent-focusgained-autocmd-running-in-command-line-editing-mode
autocmd FocusGained,BufEnter,CursorHold,CursorHoldI * if mode() != 'c' | checktime | endif
" Notification after file change
" https://vi.stackexchange.com/questions/13091/autocmd-event-for-autoread
autocmd FileChangedShellPost *
  \ echohl WarningMsg | echo "File changed on disk. Buffer reloaded." | echohl None
"

" Swap Files and Backups
set autowrite
set nobackup
set noswapfile

set undofile

" Automatically remove extra whitespace
autocmd BufWritePre * :%s/\s\+$//e
"

" Appearance
if !has('gui_running')
  set t_Co=256
endif

syntax on
syntax enable


" Avoids freezing the terminal when plugins are not yet installed
try
    colorscheme gruvbox
catch /^Vim\%((\a\+)\)\=:E185/
    colorscheme elflord
endtry

set colorcolumn=100

set listchars=tab:→\ ,space:·,nbsp:␣,trail:•,eol:¶,precedes:«,extends:»
"

" Indentation
set tabstop=4
set shiftwidth=4
set softtabstop=4

set expandtab
set shiftround
filetype plugin indent on

set backspace=eol,start,indent

set modelines=1
"

" Folding
set foldmethod=indent
set foldnestmax=10
set foldenable
set foldlevelstart=10
"

" Noises and Notifications
set noerrorbells
set novisualbell
"

" Searching
set ignorecase
set smartcase

set showmatch
"

" Windows and Tabs
set splitbelow
set splitright
"

" Keybindings
let mapleader = "\\"
let maplocalleader = "\\"

nnoremap j gj
nnoremap k gk
nnoremap ; :

nmap <silent> // :nohlsearch<CR>

inoremap <S-Tab> <C-V><Tab>

imap jk <Esc>
imap jj <Esc>
"

" Custom Functions
"   Toggle Between Test Files
function GoToTestFile()
    let test_file = expand('%:r') . '_test.' . expand('%:e')
    execute "e " . fnameescape(l:test_file)
endfunc
function GoFromTestFile()
    let source_file = substitute(expand('%:r'), '_test$', '.' . expand('%:e'), '')
    execute "e " . fnameescape(l:source_file)
endfunc
function ToggleTestFile()
    let base = expand('%:r')
    if base =~ '.*_test'
        call GoFromTestFile()
    else
        call GoToTestFile()
    end
endfunc

command T call ToggleTestFile()
"

"   Toggle Between Rust/C Files
function GoToRustFile()
    let rust_file = expand('%:r') . '.rs'
    execute "e " . fnameescape(l:rust_file)
endfunc
function GoFromRustFile()
    let c_file = expand('%:r') . '.c'
    execute "e " . fnameescape(l:c_file)
endfunc
function ToggleRustFile()
    let base = expand('%:r')
    if base =~ '.rs'
        call GoFromRustFile()
    else
        call GoToRustFile()
    end
endfunc

command R call ToggleRustFile()
"
"
"   Toggle Between Source / Header / Rust
function GoToHeaderFile()
    let header_file = expand('%:r') . '.h'
    execute "e " . fnameescape(l:header_file)
endfunc
command H call GoToHeaderFile()

function GoToSourceFile()
    let source_file = expand('%:r') . '.c'
    execute "e " . fnameescape(l:source_file)
endfunc
command C call GoToSourceFile()
"

"   Jump to Generated Code
function JumpToGeneratedCode()
    let generated_file = 'build/debug/' . expand('%') . '.gen.h'
    execute "e " . fnameescape(l:generated_file)
endfunc

command Gen call JumpToGeneratedCode()
"
"

" Lightline
let g:lightline = {
    \   'colorscheme': 'gruvbox',
    \   'active': {
    \       'left': [ [ 'mode', 'paste' ],
    \           [ 'readonly', 'absolutepath', 'modified' ] ],
    \   },
    \   'inactive': {
    \       'left': [ [ 'filename', 'modified' ] ],
    \   }
    \ }
"

" NERD Tree
let NerdTreeMinimalUI = 1
let NERDTreeIgnore = ['\.pyc$', '\.o$', '\.pdf$']
let NERDTreeAutoDeleteBuffer = 1
let NERDTreeDirArrows = 1

autocmd bufenter * if (winnr("$") == 1 && exists("b:NERDTree") && b:NERDTree.isTabTree()) | q | endif

" Start NERDTree when Vim is started without file arguments.
autocmd StdinReadPre * let s:std_in=1
autocmd VimEnter * if argc() == 0 && !exists('s:std_in') | NERDTree | endif

map <C-n> :NERDTreeToggle %<CR>
map <C-f> :NERDTreeFind<CR>
"

" NERD Commenter
let g:NERDSpaceDelims = 1
let g:NERDCompactSexyComs = 1
let g:NERDDefaultAlign = 'left'
let g:NERDCommentEmptyLines = 1
let g:NERDTrimTrailingWhitespace = 1
"

" C++ Enhanced Highlight
let g:cpp_class_scope_highlight = 1
let g:cpp_member_variable_highlight = 1
let g:cpp_class_decl_highlight = 1
"

" Arg Wrap
let g:argwrap_padded_braces = '[{'
let g:argwrap_closing_brace = 0

nnoremap <silent> <leader>a :ArgWrap<CR>
"

" Rust Formatter
let g:rustfmt_autosave_if_config_present = 1
"

" File-Specific Commands
" Python
autocmd FileType python setlocal colorcolumn=100
autocmd FileType python setlocal textwidth=100
autocmd FileType python setlocal includeexpr=substitute(substitute(v:fname,'qinternal.','','g'),'\\.','/','g')

" Rust (rustfmt defaults)
autocmd FileType rust setlocal colorcolumn=100
autocmd FileType rust setlocal textwidth=100

" C
autocmd FileType c setlocal colorcolumn=100
autocmd FileType c setlocal textwidth=100
autocmd FileType h setlocal colorcolumn=100
autocmd FileType h setlocal textwidth=100

" Markdown
autocmd FileType markdown setlocal colorcolumn=80
autocmd FileType markdown setlocal textwidth=80
"

" ALE
" Disabled for Qumulo
" let g:ale_linters = {'rust': ['analyzer']}
"

" vim-lsp
" Use Rust Analyzer
if executable('rust-analyzer')
    au User lsp_setup call lsp#register_server({
        \   'name': 'Rust Language Server',
        \   'cmd': {server_info->['rust-analyzer']},
        \   'whitelist': ['rust'],
        \ })
endif

" Some vim-lsp shortcuts
function! s:on_lsp_buffer_enabled() abort
    setlocal omnifunc=lsp#complete
    " setlocal signcolumn=yes
    if exists('+tagfunc') | setlocal tagfunc=lsp#tagfunc | endif
    nmap <buffer> gd <plug>(lsp-definition)
    nmap <buffer> gs <plug>(lsp-document-symbol-search)
    nmap <buffer> gS <plug>(lsp-workspace-symbol-search)
    nmap <buffer> gr <plug>(lsp-references)
    nmap <buffer> gi <plug>(lsp-implementation)
    " Disabled since it interferes with tab switching.
    " nmap <buffer> gt <plug>(lsp-type-definition)
    nmap <buffer> <leader>rn <plug>(lsp-rename)
    nmap <buffer> [g <plug>(lsp-previous-diagnostic)
    nmap <buffer> ]g <plug>(lsp-next-diagnostic)
    nmap <buffer> K <plug>(lsp-hover)
    inoremap <buffer> <expr><c-f> lsp#scroll(+4)
    inoremap <buffer> <expr><c-d> lsp#scroll(-4)
endfunction

augroup lsp_install
    au!
    " call s:on_lsp_buffer_enabled only for languages that has the server registered.
    autocmd User lsp_buffer_enabled call s:on_lsp_buffer_enabled()
augroup END

"

" Python Black
let g:black_fast = 1
let g:black_quiet = 1
"

" vim:foldmethod=marker:foldlevel=0