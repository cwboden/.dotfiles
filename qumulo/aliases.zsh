# Check Run
alias cr='./check_run.py -j 8'
alias crq='./check_run.py -Q'
alias bcr='./check_run.py -b -j 8'
alias bch=bcr
alias crd='./check_run.py -b --flavor debug -j 8'
alias crr='./check_run.py -b --flavor release -j 8'
alias bcrq='./check_run.py -bQ'
alias red-green='./tools/red_green.py'

# Build
alias tags='build all_tests; build tags cscope'

# Utility
alias fetch-all='ssha; hg qpop -a; hg fetch; ./prebuild'
alias hello='fetch-all; tags'

