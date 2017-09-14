current filetype: cpp
linters: clang, clangtidy, cppcheck, cpplint, g++

g:ale_cpp_clang_options = '-std=c++14 -Wall'
g:ale_cpp_clangtidy_checks = ['*']
g:ale_cpp_clangtidy_options = ''
g:ale_cpp_cppcheck_options = '--enable=style'
g:ale_cpp_cpplint_options = ''

