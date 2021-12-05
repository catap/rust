// revisions: CHECK-BASE CHECK-OPT
// compile-flags: -C no-prepopulate-passes -C opt-level=0 -Z mir-opt-level=1
//[CHECK-BASE] compile-flags: -Z opt-switch-monomorphizing=off
//[CHECK-OPT] compile-flags: -Z opt-switch-monomorphizing=on

#![crate_type = "lib"]
#![feature(core_intrinsics)]
#![feature(never_type)]

use std::intrinsics::discriminant_value;
use std::num::NonZeroUsize;

pub enum BothEmpty {
    Left(!),
    Right(!),
}

// CHECK-LABEL: @match_both_empty
#[no_mangle]
pub fn match_both_empty(e: BothEmpty) -> u8 {
    // CHECK-BASE-NOT: unreachable
    // CHECK-BASE: switch i[[TY:[0-9]+]] undef, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT:    i[[TY]] 0
    // CHECK-BASE-NEXT:    i[[TY]] 1
    // CHECK-BASE-NEXT:    i[[TY]] 2
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NOT: switch
    // CHECK-OPT: unreachable
    match discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}

pub enum EmptyOrBool {
    Empty(!),
    Bool(bool),
}

// CHECK-LABEL: @match_empty_or_bool
#[no_mangle]
pub fn match_empty_or_bool(e: EmptyOrBool) -> u8 {
    // CHECK: %[[REG:[0-9]+]] = zext i1 %0 to i8
    // CHECK-NEXT: store i8 %[[REG]], i8* %e
    // CHECK-BASE-NEXT: switch i[[TY:[0-9]+]] 1, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0
    // CHECK-BASE-NEXT: i[[TY]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 2
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: br label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 1, i8* %1
    match discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}

pub enum BoolOrEmpty {
    Bool(bool),
    Empty(!),
}

// CHECK-LABEL: @match_bool_or_empty
#[no_mangle]
pub fn match_bool_or_empty(e: BoolOrEmpty) -> u8 {
    // CHECK: %[[REG:[0-9]+]] = zext i1 %0 to i8
    // CHECK-NEXT: store i8 %[[REG]], i8* %e
    // CHECK-BASE-NEXT: switch i[[TY:[0-9]+]] 0, label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 1
    // CHECK-BASE-NEXT: i[[TY]] 2
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: br label %[[L:[a-zA-Z0-9_]+]]
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 0, i8* %1
    match discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}

pub enum UninhabitedUsizeOrBool {
    Usize(usize, !),
    Bool(bool),
}

// CHECK-LABEL: @match_uninhabited_usize_or_bool
#[no_mangle]
pub fn match_uninhabited_usize_or_bool(e: UninhabitedUsizeOrBool) -> u8 {
    // CHECK: %[[TMP:[0-9]+]] = load i8, i8* %{{[0-9]+}}
    // CHECK-NEXT: %_[[REG:[0-9]+]] = zext i8 %[[TMP]] to i[[SIZE:[0-9]+]]
    // CHECK-BASE-NEXT: switch i[[TY:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0
    // CHECK-BASE-NEXT: i[[TY]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 2
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 1
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 1, i8* %1
    match discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}

pub enum BoolOrUninhabitedUsize {
    Bool(bool),
    Usize(usize, !),
}

// CHECK-LABEL: @match_bool_or_uninhabited_nonzero_usize
#[no_mangle]
pub fn match_bool_or_uninhabited_nonzero_usize(e: BoolOrUninhabitedUsize) -> u8 {
    // CHECK: %[[TMP:[0-9]+]] = load i8, i8* %{{[0-9]+}}
    // CHECK-NEXT: %_[[REG:[0-9]+]] = zext i8 %[[TMP]] to i[[SIZE:[0-9]+]]
    // CHECK-BASE-NEXT: switch i[[TY:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %[[L:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 1
    // CHECK-BASE-NEXT: i[[TY]] 2
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 0
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[L:[a-zA-Z0-9_]+]]
    // CHECK: [[L]]:
    // CHECK-NEXT: store i8 0, i8* %1
    match discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}

pub enum UninhabitedNonZeroUsizeOrUnit {
    Usize(NonZeroUsize, !),
    Unit,
}

// CHECK-LABEL: @match_uninhabited_non_zero_usize_or_unit
#[no_mangle]
pub fn match_uninhabited_non_zero_usize_or_unit(e: UninhabitedNonZeroUsizeOrUnit) -> u8 {
    // CHECK: %[[TMP1:[0-9]+]] = load i[[SIZE:[0-9]+]], i[[SIZE]]* %{{[0-9]+}}
    // CHECK-NEXT: %[[TMP2:[0-9]+]] = icmp eq i[[SIZE]] %[[TMP1]], 0
    // CHECK-NEXT: %_[[REG:[0-9]+]] = select i1 %[[TMP2]], i[[SIZE]] 1, i[[SIZE]] 0
    // CHECK-BASE-NEXT: switch i[[TY:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0
    // CHECK-BASE-NEXT: i[[TY]] 1, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 2
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 1
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 1, i8* %1
    match discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}

pub enum UnitOrUninhabitedNonZeroUsize {
    Unit,
    Usize(NonZeroUsize, !),
}

// CHECK-LABEL: @match_unit_or_uninhabited_non_zero_usize
#[no_mangle]
pub fn match_unit_or_uninhabited_non_zero_usize(e: UnitOrUninhabitedNonZeroUsize) -> u8 {
    // CHECK: %[[TMP1:[0-9]+]] = load i[[SIZE:[0-9]+]], i[[SIZE]]* %{{[0-9]+}}
    // CHECK-NEXT: %[[TMP2:[0-9]+]] = icmp eq i[[SIZE]] %[[TMP1]], 0
    // CHECK-NEXT: %_[[REG:[0-9]+]] = select i1 %[[TMP2]], i[[SIZE]] 0, i[[SIZE]] 1
    // CHECK-BASE-NEXT: switch i[[TY:[0-9]+]] %_[[REG]], label %{{[a-zA-Z0-9_]+}} [
    // CHECK-BASE-NEXT: i[[TY]] 0, label %[[R:[a-zA-Z0-9_]+]]
    // CHECK-BASE-NEXT: i[[TY]] 1
    // CHECK-BASE-NEXT: i[[TY]] 2
    // CHECK-BASE-NEXT: ]
    // CHECK-OPT-NEXT: %[[D:[0-9]+]] = icmp eq i[[SIZE]] %_[[REG]], 0
    // CHECK-OPT-NEXT: br i1 %[[D]], label %[[R:[a-zA-Z0-9_]+]]
    // CHECK: [[R]]:
    // CHECK-NEXT: store i8 0, i8* %1
    match discriminant_value(&e) {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => 3,
    }
}
