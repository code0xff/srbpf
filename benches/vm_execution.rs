// Copyright 2020 Solana Maintainers <maintainers@solana.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(test)]

extern crate srbpf;
extern crate test;

use srbpf::{
    elf::Executable, program::BuiltinProgram, verifier::RequisiteVerifier, vm::TestContextObject,
};
use std::{fs::File, io::Read, sync::Arc};
use test::Bencher;
use test_utils::create_vm;

#[bench]
fn bench_init_interpreter_start(bencher: &mut Bencher) {
    let mut file = File::open("tests/elfs/rodata_section_sbpfv1.so").unwrap();
    let mut elf = Vec::new();
    file.read_to_end(&mut elf).unwrap();
    let executable =
        Executable::<TestContextObject>::from_elf(&elf, Arc::new(BuiltinProgram::new_mock()))
            .unwrap();
    executable.verify::<RequisiteVerifier>().unwrap();
    let mut context_object = TestContextObject::default();
    create_vm!(
        vm,
        &executable,
        &mut context_object,
        stack,
        heap,
        Vec::new(),
        None
    );
    bencher.iter(|| {
        vm.context_object_pointer.remaining = 37;
        vm.execute_program(&executable, true).1.unwrap()
    });
}
