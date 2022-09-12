add_custom_target(bins)
add_dependencies(bins main)

add_executable(main main.c)

target_compile_options(main PUBLIC -O2 -Wall -Wextra)

target_compile_features(main PUBLIC c_std_11)

# target_link_libraries(main sub-sh7136 mylib1 remote m)
