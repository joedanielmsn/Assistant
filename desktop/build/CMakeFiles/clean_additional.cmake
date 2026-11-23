# Additional clean files
cmake_minimum_required(VERSION 3.16)

if("${CONFIG}" STREQUAL "" OR "${CONFIG}" STREQUAL "Debug")
  file(REMOVE_RECURSE
  "CMakeFiles/local-assistant_autogen.dir/AutogenUsed.txt"
  "CMakeFiles/local-assistant_autogen.dir/ParseCache.txt"
  "local-assistant_autogen"
  )
endif()
