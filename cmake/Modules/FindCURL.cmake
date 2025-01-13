mark_as_advanced(CURL_LIBRARY CURL_INCLUDE_DIR)

find_library(CURL_LIBRARY NAMES curl libcurl)
find_path(CURL_INCLUDE_DIR NAMES curl/curl.h)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(CURL DEFAULT_MSG CURL_LIBRARY CURL_INCLUDE_DIR)
