export TELEBAR_EXAMPLE_CONFIG_FILE=${PWD}/examples/Config.toml
export TELEBAR_EXAMPLE_SOCKET_ID=7
export TELEBAR_EXAMPLE_SOCKET_PATH=${XDG_RUNTIME_DIR}/${TELEBAR_SOCKET_ID_EXAMPLE}_telebar_socket

export TELEBAR_TEST_CONFIG_FILE=${PWD}/functional_tests/Config.toml
export TELEBAR_TEST_SOCKET_ID=5
export TELEBAR_TEST_SOCKET_PATH=${XDG_RUNTIME_DIR}/${TELEBAR_TEST_SOCKET_ID}_telebar_socket
export TELEBAR_TEST_BUILD_DIR=${PWD}/functional_tests/test_build
export TELEBAR_TEST_BINARY=${TELEBAR_TEST_BUILD_DIR}/release/main
export TELEBAR_TEST_SHELL_LIB=${PWD}/functional_tests/lib.sh
