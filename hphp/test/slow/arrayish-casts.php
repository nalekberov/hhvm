<?hh
// Copyright (c) Facebook, Inc. and its affiliates. All Rights Reserved.

class A {
  public static function meth() {}
}

function foo() {}

function varrayCast($x) {
  return varray($x);
}

function darrayCast($x) {
  return darray($x);
}

function dictCast($x) {
  return dict($x);
}

function vecCast($x) {
  return vec($x);
}

function keysetCast($x) {
  return keyset($x);
}

function getFun() {
  return __hhvm_intrinsics\launder_value(foo<>);
}

function getMeth() {
  return __hhvm_intrinsics\launder_value(A::meth<>);
}

<<__EntryPoint>>
function main(): void {
  $tests = vec[
    () ==> var_dump(darrayCast(getFun())),
    () ==> var_dump(varrayCast(getFun())),
    () ==> var_dump(dictCast(getFun())),
    () ==> var_dump(vecCast(getFun())),
    () ==> var_dump(keysetCast(getFun())),

    () ==> var_dump(darrayCast(getMeth())),
    () ==> var_dump(varrayCast(getMeth())),
    () ==> var_dump(dictCast(getMeth())),
    () ==> var_dump(vecCast(getMeth())),
    () ==> var_dump(keysetCast(getMeth())),
  ];

  $success = false;
  $count = apc_fetch('test-count', inout $success);
  if (!$success) $count = 0;
  apc_store('test-count', $count + 1);
  try {
    $tests[$count]();
  } catch (Exception $e) {
    echo "Exception: " . $e->getMessage() . "\n";
  }
}
