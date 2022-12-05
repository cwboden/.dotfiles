filetype plugin indent on

"  ____  _             _
" |  _ \| |_   _  __ _(_)_ __  ___
" | |_) | | | | |/ _` | | '_ \/ __|
" |  __/| | |_| | (_| | | | | \__ \
" |_|   |_|\__,_|\__, |_|_| |_|___/
"                |___/
" FIGlet: Plugins

call plug#begin()

" Appearance
Plug 'itchyny/lightline.vim'
Plug 'morhetz/gruvbox'
Plug 'ryanoasis/vim-devicons'
Plug 'shinchu/lightline-gruvbox.vim'

" Keybindings and Navigation
Plug 'ervandew/supertab'
Plug 'christoomey/vim-tmux-navigator'

" Language Server Protocol Configuration
Plug 'hrsh7th/cmp-buffer'
Plug 'hrsh7th/cmp-cmdline'
Plug 'hrsh7th/cmp-nvim-lsp'
Plug 'hrsh7th/cmp-path'
Plug 'hrsh7th/cmp-vsnip'
Plug 'hrsh7th/nvim-cmp'
Plug 'hrsh7th/vim-vsnip'
Plug 'neovim/nvim-lspconfig'
Plug 'neovim/nvim-lspconfig'
Plug 'simrat39/rust-tools.nvim'
Plug 'onsails/lspkind.nvim'

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

"   ____                           _
"  / ___| ___ _ __   ___ _ __ __ _| |
" | |  _ / _ \ '_ \ / _ \ '__/ _` | |
" | |_| |  __/ | | |  __/ | | (_| | |
"  \____|\___|_| |_|\___|_|  \__,_|_|
"
"   ____             __ _                       _   _
"  / ___|___  _ __  / _(_) __ _ _   _ _ __ __ _| |_(_) ___  _ __
" | |   / _ \| '_ \| |_| |/ _` | | | | '__/ _` | __| |/ _ \| '_ \
" | |__| (_) | | | |  _| | (_| | |_| | | | (_| | |_| | (_) | | | |
"  \____\___/|_| |_|_| |_|\__, |\__,_|_|  \__,_|\__|_|\___/|_| |_|
"                         |___/
" FIGlet: General Configuration

" Cursor Placement
set nowrap
set so=8

" Indentation
set tabstop=4
set shiftwidth=4
set softtabstop=4
set expandtab
set shiftround
set modelines=1

" Folding
set foldmethod=indent
set foldnestmax=10
set foldenable
set foldlevelstart=10

" Noises and Notifications
set noerrorbells

" Searching
set ignorecase
set smartcase
set showmatch
set wildmode=list:longest,list:full

" Windows and Tabs
set splitbelow
set splitright

"     _
"    / \   _ __  _ __   ___  __ _ _ __ __ _ _ __   ___ ___
"   / _ \ | '_ \| '_ \ / _ \/ _` | '__/ _` | '_ \ / __/ _ \
"  / ___ \| |_) | |_) |  __/ (_| | | | (_| | | | | (_|  __/
" /_/   \_\ .__/| .__/ \___|\__,_|_|  \__,_|_| |_|\___\___|
"         |_|   |_|
" FIGlet: Appearance

" Automatic switching of relative numbers
augroup numbertoggle
  autocmd!
  autocmd BufEnter,FocusGained,InsertLeave,WinEnter * if &nu | set rnu | endif
  autocmd BufLeave,FocusLost,InsertEnter,WinLeave * if &nu | set nornu | endif
augroup END

" Use 256-style terminal colors if the GUI isn't supported
if !has('gui_running')
  set t_Co=256
endif

" Avoids freezing the terminal when the color scheme is not yet installed
try
    colorscheme gruvbox
catch /^Vim\%((\a\+)\)\=:E185/
    colorscheme elflord
endtry

syntax on
syntax enable

set colorcolumn=100
set listchars=tab:→\ ,space:·,nbsp:␣,trail:•,eol:¶,precedes:«,extends:»
set number
set relativenumber
set cursorline
set noshowmode
set novisualbell

"  _  __            ____  _           _ _
" | |/ /___ _   _  | __ )(_)_ __   __| (_)_ __   __ _ ___
" | ' // _ \ | | | |  _ \| | '_ \ / _` | | '_ \ / _` / __|
" | . \  __/ |_| | | |_) | | | | | (_| | | | | | (_| \__ \
" |_|\_\___|\__, | |____/|_|_| |_|\__,_|_|_| |_|\__, |___/
"           |___/                               |___/
" FIGlet: Key Bindings

let mapleader = "\\"
let maplocalleader = "\\"

imap jk <Esc>
imap jj <Esc>

nnoremap j gj
nnoremap k gk
nnoremap ; :

nmap <silent> // :nohlsearch<CR>

set backspace=eol,start,indent

"  _____ _ _              ___     ____         __  __
" |  ___(_) | ___  ___   ( _ )   | __ ) _   _ / _|/ _| ___ _ __ ___
" | |_  | | |/ _ \/ __|  / _ \/\ |  _ \| | | | |_| |_ / _ \ '__/ __|
" |  _| | | |  __/\__ \ | (_>  < | |_) | |_| |  _|  _|  __/ |  \__ \
" |_|   |_|_|\___||___/  \___/\/ |____/ \__,_|_| |_|  \___|_|  |___/
" FIGlet: Files & Buffers

let $LANG='en'
set langmenu=en

set autowrite
set nobackup
set noswapfile
set undofile

" Ignore specific files
set wildignore=*.o,*~,*.pyc
if has("win16") || has("win32")
  set wildignore+=.git\*,.hg\*,.svn\*
else
  set wildignore+=*/.git/*,*/.hg/*,*/.svn/*,*/.DS_Store
endif

" Trigger `autoread` when files changes on disk
" Relies on vim-tmux-focus-events if using vim inside tmux.
" https://unix.stackexchange.com/questions/149209/refresh-changed-content-of-file-opened-in-vim/383044#383044
" https://vi.stackexchange.com/questions/13692/prevent-focusgained-autocmd-running-in-command-line-editing-mode
autocmd FocusGained,BufEnter,CursorHold,CursorHoldI * if mode() != 'c' | checktime | endif

" Notification after file change
" https://vi.stackexchange.com/questions/13091/autocmd-event-for-autoread
autocmd FileChangedShellPost *
  \ echohl WarningMsg | echo "File changed on disk. Buffer reloaded." | echohl None

"  _____ _ _           ____                  _  __ _
" |  ___(_) | ___     / ___| _ __   ___  ___(_)/ _(_) ___
" | |_  | | |/ _ \____\___ \| '_ \ / _ \/ __| | |_| |/ __|
" |  _| | | |  __/_____|__) | |_) |  __/ (__| |  _| | (__
" |_|   |_|_|\___|    |____/| .__/ \___|\___|_|_| |_|\___|
"                           |_|
"   ____                                          _
"  / ___|___  _ __ ___  _ __ ___   __ _ _ __   __| |___
" | |   / _ \| '_ ` _ \| '_ ` _ \ / _` | '_ \ / _` / __|
" | |__| (_) | | | | | | | | | | | (_| | | | | (_| \__ \
"  \____\___/|_| |_| |_|_| |_| |_|\__,_|_| |_|\__,_|___/
" FIGlet: File-Specific Commands

" C
autocmd FileType c setlocal colorcolumn=100
autocmd FileType c setlocal textwidth=100
autocmd FileType h setlocal colorcolumn=100
autocmd FileType h setlocal textwidth=100

" Markdown
autocmd FileType markdown setlocal colorcolumn=80
autocmd FileType markdown setlocal textwidth=80

" Python
autocmd FileType python setlocal colorcolumn=100
autocmd FileType python setlocal textwidth=100
autocmd FileType python setlocal includeexpr=substitute(substitute(v:fname,'qinternal.','','g'),'\\.','/','g')

" Rust (rustfmt defaults)
autocmd FileType rust setlocal colorcolumn=100
autocmd FileType rust setlocal textwidth=100

"   ____          _
"  / ___|   _ ___| |_ ___  _ __ ___
" | |  | | | / __| __/ _ \| '_ ` _ \
" | |__| |_| \__ \ || (_) | | | | | |
"  \____\__,_|___/\__\___/|_| |_| |_|
"
"  _____                 _   _
" |  ___|   _ _ __   ___| |_(_) ___  _ __  ___
" | |_ | | | | '_ \ / __| __| |/ _ \| '_ \/ __|
" |  _|| |_| | | | | (__| |_| | (_) | | | \__ \
" |_|   \__,_|_| |_|\___|\__|_|\___/|_| |_|___/
" FIGlet: Custom Functions

" Automatically remove extra whitespace
autocmd BufWritePre * :%s/\s\+$//e

" Toggle Between Test Files
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

"  ____  _             _
" |  _ \| |_   _  __ _(_)_ __
" | |_) | | | | |/ _` | | '_ \
" |  __/| | |_| | (_| | | | | |
" |_|   |_|\__,_|\__, |_|_| |_|
"                |___/
"   ____             __ _                       _   _
"  / ___|___  _ __  / _(_) __ _ _   _ _ __ __ _| |_(_) ___  _ __
" | |   / _ \| '_ \| |_| |/ _` | | | | '__/ _` | __| |/ _ \| '_ \
" | |__| (_) | | | |  _| | (_| | |_| | | | (_| | |_| | (_) | | | |
"  \____\___/|_| |_|_| |_|\__, |\__,_|_|  \__,_|\__|_|\___/|_| |_|
"                         |___/
" FIGlet: Plugin Configuration

" Arg Wrap
let g:argwrap_padded_braces = '[{'
let g:argwrap_closing_brace = 0

nnoremap <silent> <leader>a :ArgWrap<CR>

" Black (Python)
let g:black_fast = 1
let g:black_quiet = 1

" C++ Enhanced Highlight
let g:cpp_class_scope_highlight = 1
let g:cpp_member_variable_highlight = 1
let g:cpp_class_decl_highlight = 1

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

" NERD Tree
let NerdTreeMinimalUI = 1
let NERDTreeIgnore = ['\.pyc$', '\.o$', '\.pdf$']
let NERDTreeAutoDeleteBuffer = 1
let NERDTreeDirArrows = 1

"   Close Vim if the only open pane is NERDTree
autocmd bufenter * if (winnr("$") == 1 && exists("b:NERDTree") && b:NERDTree.isTabTree()) | q | endif

"   Start NERDTree when Vim is started without file arguments.
autocmd StdinReadPre * let s:std_in=1
autocmd VimEnter * if argc() == 0 && !exists('s:std_in') | NERDTree | endif

map <C-n> :NERDTreeToggle %<CR>
map <C-f> :NERDTreeFind<CR>

" NERD Commenter
let g:NERDSpaceDelims = 1
let g:NERDCompactSexyComs = 1
let g:NERDDefaultAlign = 'left'
let g:NERDCommentEmptyLines = 1
let g:NERDTrimTrailingWhitespace = 1

" Rust Tools
set completeopt=menu,menuone,noselect

lua <<EOF

  -- Set up nvim-cmp.
  local cmp = require('cmp')
  local lspkind = require('lspkind')

  cmp.setup({
    snippet = {
      -- REQUIRED - you must specify a snippet engine
      expand = function(args)
        vim.fn["vsnip#anonymous"](args.body)
      end,
    },
    mapping = cmp.mapping.preset.insert({
      ['<C-b>'] = cmp.mapping.scroll_docs(-4),
      ['<C-f>'] = cmp.mapping.scroll_docs(4),
      ['<C-Space>'] = cmp.mapping.complete(),
      ['<C-e>'] = cmp.mapping.abort(),
      ['<CR>'] = cmp.mapping.confirm({ select = true }), -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
    }),
    sources = cmp.config.sources({
      { name = 'nvim_lsp' },
      { name = 'vsnip' },
    }, {
      { name = 'buffer' },
    }),
    formatting = {
      format = lspkind.cmp_format({
        mode = 'symbol', -- show only symbol annotations
        maxwidth = 60, -- prevent the popup from showing more than provided characters (e.g 50 will not show more than 50 characters)
        ellipsis_char = '...', -- when popup menu exceed maxwidth, the truncated part would show ellipsis_char instead (must define maxwidth first)
      })
    }
  })

  -- Set configuration for specific filetype.
  cmp.setup.filetype('gitcommit', {
    sources = cmp.config.sources({
      { name = 'cmp_git' }, -- You can specify the `cmp_git` source if you were installed it.
    }, {
      { name = 'buffer' },
    })
  })

  -- Use buffer source for `/` and `?` (if you enabled `native_menu`, this won't work anymore).
  cmp.setup.cmdline({ '/', '?' }, {
    mapping = cmp.mapping.preset.cmdline(),
    sources = {
      { name = 'buffer' }
    }
  })

  -- Use cmdline & path source for ':' (if you enabled `native_menu`, this won't work anymore).
  cmp.setup.cmdline(':', {
    mapping = cmp.mapping.preset.cmdline(),
    sources = cmp.config.sources({
      { name = 'path' }
    }, {
      { name = 'cmdline' }
    })
  })

  -- Set up lspconfig.
  local capabilities = require('cmp_nvim_lsp').default_capabilities()
  require('lspconfig')['rust_analyzer'].setup {
    capabilities = capabilities
  }
  require('lspconfig')['jedi_language_server'].setup {
    capabilities = capabilities
  }

EOF

" Rust Formatter
let g:rustfmt_autosave_if_config_present = 1
