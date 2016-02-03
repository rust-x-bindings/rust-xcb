#!/bin/bash

function test_success {
    "$@" > /dev/null 2>&1
    local status=$?
    if [ $status -ne 0 ]
    then
        echo "$@"
        "$@"
        echo -e "\e[38;5;166mSuccess test failed:\e[0m" "$@"
    else
        echo -e "\e[38;5;76mSuccess test succeed:\e[0m" "$@"
    fi
}

function test_failure {
    "$@" > /dev/null 2>&1
    local status=$?
    if [ $status -eq 0 ]
    then
        echo "$@"
        "$@"
        echo -e "\e[38;5;166mFailure test failed:\e[0m" "$@"
    else
        echo -e "\e[38;5;76mFailure test succeed:\e[0m" "$@"
    fi
}

test_success cargo build --example connect
test_success cargo build --example ffi_screen_info
test_success cargo build --example screen_info
test_success cargo build --example basic_window
test_success cargo build --example ffi_randr_crtc_info --features randr
test_success cargo build --example randr_crtc_info --features randr
test_success cargo build --example randr_screen_info --features randr
test_success cargo build --example ffi_xkb_init --features xkb
test_success cargo build --example xkb_init --features xkb

test_failure cargo build --example must_fail_borrow_check__reply --features randr
test_failure cargo build --example must_fail_borrow_check__setup
