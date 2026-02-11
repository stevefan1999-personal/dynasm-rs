use dynasmrt::dynasm;
use dynasmrt::{DynasmError, DynasmLabelApi, relocations::SimpleRelocation};

// tests for all the different relocation behaviours
// a bunch of things have to be duplicated for the different usize bitsize probabilities.

// first of all, tests for [relative/absolute] encodings of [relative/absolute] addresses, 32 and 64-bit
#[cfg(target_pointer_width="32")]
#[test]
fn static_relative_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0xAABB_CCDDusize);
    let dynamic_label = ops.new_dynamic_label();
    dynasm!(ops
        ;backward_local_label:
        ; .rel8 >forward_local_label
        ; .rel8 <backward_local_label
        ; .rel8 ->global_label
        ; .rel8 =>dynamic_label
        ; .rel16 >forward_local_label
        ; .rel16 <backward_local_label
        ; .rel16 ->global_label
        ; .rel16 =>dynamic_label
        ; .rel32 >forward_local_label
        ; .rel32 <backward_local_label
        ; .rel32 ->global_label
        ; .rel32 =>dynamic_label
        ; .rel64 >forward_local_label
        ; .rel64 <backward_local_label
        ; .rel64 ->global_label
        ; .rel64 =>dynamic_label
        ;forward_local_label:
        ;->global_label:
        ;=>dynamic_label
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "3C,FF,3A,39,38,00,FA,FF,34,00,32,00,30,00,00,00,F0,FF,FF,FF,28,00,00,00,24,00,00,00,20,00,00,00,00,00,00,00,DC,FF,FF,FF,FF,FF,FF,FF,10,00,00,00,00,00,00,00,08,00,00,00,00,00,00,00",
               "relative relocationsto relative addresses");
}

#[cfg(target_pointer_width="64")]
#[test]
fn static_relative_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0xAABB_CCDD_EEFF_0011usize);
    let dynamic_label = ops.new_dynamic_label();
    dynasm!(ops
        ;backward_local_label:
        ; .rel8 >forward_local_label
        ; .rel8 <backward_local_label
        ; .rel8 ->global_label
        ; .rel8 =>dynamic_label
        ; .rel16 >forward_local_label
        ; .rel16 <backward_local_label
        ; .rel16 ->global_label
        ; .rel16 =>dynamic_label
        ; .rel32 >forward_local_label
        ; .rel32 <backward_local_label
        ; .rel32 ->global_label
        ; .rel32 =>dynamic_label
        ; .rel64 >forward_local_label
        ; .rel64 <backward_local_label
        ; .rel64 ->global_label
        ; .rel64 =>dynamic_label
        ;forward_local_label:
        ;->global_label:
        ;=>dynamic_label
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "3C,FF,3A,39,38,00,FA,FF,34,00,32,00,30,00,00,00,F0,FF,FF,FF,28,00,00,00,24,00,00,00,20,00,00,00,00,00,00,00,DC,FF,FF,FF,FF,FF,FF,FF,10,00,00,00,00,00,00,00,08,00,00,00,00,00,00,00",
               "relative relocations to relative addresses");
}

#[cfg(target_pointer_width="32")]
#[test]
fn static_absolute_to_relative_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0xAABB_CC00usize);
    let dynamic_label = ops.new_dynamic_label();
    dynasm!(ops
        ;backward_local_label:
        ; .abs32 >forward_local_label
        ; .abs32 <backward_local_label
        ; .abs32 ->global_label
        ; .abs32 =>dynamic_label
        ;forward_local_label:
        ;->global_label:
        ;=>dynamic_label
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "10,CC,BB,AA,00,CC,BB,AA,10,CC,BB,AA,10,CC,BB,AA",
               "absolute relocations to relative addresses");
}

#[cfg(target_pointer_width="64")]
#[test]
fn static_absolute_to_relative_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0xAABB_CCDD_EEFF_0000usize);
    let dynamic_label = ops.new_dynamic_label();
    dynasm!(ops
        ;backward_local_label:
        ; .abs64 >forward_local_label
        ; .abs64 <backward_local_label
        ; .abs64 ->global_label
        ; .abs64 =>dynamic_label
        ;forward_local_label:
        ;->global_label:
        ;=>dynamic_label
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "20,00,FF,EE,DD,CC,BB,AA,00,00,FF,EE,DD,CC,BB,AA,20,00,FF,EE,DD,CC,BB,AA,20,00,FF,EE,DD,CC,BB,AA",
               "absolute relocations to relative addresses, 64 bits");
}

#[cfg(target_pointer_width="32")]
#[test]
fn static_relative_to_absolute_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0xAABB_0000usize);
    dynasm!(ops
        ; .rel8 extern 0xAABB_0040
        ; .rel8 extern 0xAABA_FFC0
        ; .rel16 extern 0xAABB_0040
        ; .rel16 extern 0xAABA_FFC0
        ; .rel32 extern 0xAABB_0040
        ; .rel32 extern 0xAABA_FFC0
        ; .rel64 extern 0xAABB_0040
        ; .rel64 extern 0xAABA_FFC0
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "40,BF,3E,00,BC,FF,3A,00,00,00,B6,FF,FF,FF,32,00,00,00,00,00,00,00,AA,FF,FF,FF,FF,FF,FF,FF",
               "relative relocations to absolute addresses");
}

#[cfg(target_pointer_width="64")]
#[test]
fn static_relative_to_absolute_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0xAABB_CCDD_EEFF_0000usize);
    dynasm!(ops
        ; .rel8 extern 0xAABB_CCDD_EEFF_0040
        ; .rel8 extern 0xAABB_CCDD_EEFE_FFC0
        ; .rel16 extern 0xAABB_CCDD_EEFF_0040
        ; .rel16 extern 0xAABB_CCDD_EEFE_FFC0
        ; .rel32 extern 0xAABB_CCDD_EEFF_0040
        ; .rel32 extern 0xAABB_CCDD_EEFE_FFC0
        ; .rel64 extern 0xAABB_CCDD_EEFF_0040
        ; .rel64 extern 0xAABB_CCDD_EEFE_FFC0
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "40,BF,3E,00,BC,FF,3A,00,00,00,B6,FF,FF,FF,32,00,00,00,00,00,00,00,AA,FF,FF,FF,FF,FF,FF,FF",
               "relative relocations to absolute addresses");
}

#[cfg(target_pointer_width="32")]
#[test]
fn static_absolute_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0xABCD_EF01usize);
    dynasm!(ops
        ; .abs32 extern 0x1122_3344
        ; .abs32 extern 0xAABB_CCDD
        ; .abs64 extern 0x1122_3344
        ; .abs64 extern 0xAABB_CCDD
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "44,33,22,11,DD,CC,BB,AA,44,33,22,11,00,00,00,00,DD,CC,BB,AA,FF,FF,FF,FF",
               "absolute relocations in 32-bit address space");
}

#[cfg(target_pointer_width="64")]
#[test]
fn static_absolute_relocations() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0123_4567_89AB_CDEFusize);
    dynasm!(ops
        ; .abs32 extern 0x0000_0000_1122_3344
        ; .abs32 extern 0xFFFF_FFFF_AABB_CCDD
        ; .abs64 extern 0x0000_0000_1122_3344
        ; .abs64 extern 0xFFFF_FFFF_AABB_CCDD
        ; .abs64 extern 0x1122_3344_5566_7788
        ; .abs64 extern 0xFFEE_DDCC_BBAA_9900
    );
    let buf = ops.finalize().unwrap();
    let hex: Vec<String> = buf.iter().map(|x| format!("{:02X}", *x)).collect();
    let hex: String = hex.join(",");
    assert_eq!(hex,
               "44,33,22,11,DD,CC,BB,AA,44,33,22,11,00,00,00,00,DD,CC,BB,AA,FF,FF,FF,FF,88,77,66,55,44,33,22,11,00,99,AA,BB,CC,DD,EE,FF",
               "absolute relocations in 64-bit address space");
}


// and finally some test cases to validate that we're correctly failing to encode relocations that are too big
#[cfg(target_pointer_width="64")]
#[test]
fn too_big_absolute_relocation() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .abs32 extern 0x8000_0000
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }

    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .abs32 extern 0xFFFF_FFFF_7FFF_FFFF
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }
}

#[test]
fn too_big_relative_relocation() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .rel8 extern 0x80
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }

    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .rel8 extern (-0x81isize) as usize
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }

    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .rel16 extern 0x8000
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }

    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .rel16 extern (-0x8001isize) as usize
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }
}

#[cfg(target_pointer_width="64")]
#[test]
fn too_big_relative_relocation_64() {
    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .rel32 extern 0x8000_0000
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }

    let mut ops = dynasmrt::VecAssembler::<SimpleRelocation>::new(0x0usize);
    dynasm!(ops
        ; .rel32 extern (-0x8000_0001isize) as usize
    );
    match ops.finalize() {
        Err(DynasmError::ImpossibleRelocation(_)) => (),
        x => panic!("{:?}", x)
    }
}
