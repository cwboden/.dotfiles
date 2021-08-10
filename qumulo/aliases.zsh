# Check Run
alias b='build'
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
alias clean-cache='qonstruct/cache_tool.py trim --entry-ctime 3_days_ago ~/cache/'
alias clean-all='qpkg sweep; rm -rf build/tmp/; ./tools/rm_merge_remnants.sh; clean-cache'
alias hello='fetch-all && clean-all && tags'
