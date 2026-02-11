% Dynamic assembly basics

Dynamic assembly, runtime assembly, or Just-in-time compilation is the technique of generating machine code at runtime. This is in contrast to the more standard ahead-of-time assembling, where all machine code for a program is generated ahead of its execution. This technique occupies a niche in between compiled code and interpreted code, where it is possible to compile code at runtime so that it executes several times faster than interpreted code, while having a much lower up-front cost than ahead-of-time compilation.

This page discusses some of the details one should be aware of when using this technique through `dynasm-rs` as well as some of the required terminology.

# Differences between ahead-of-time compilation and just-in-time compilation

The fundamental difference between these two techniques is that during ahead-of-time compilation, the entire code to be compiled is known. In just-in-time compilation it is instead normal for only parts of the program to be compiled at a time, often without full knowledge of the rest of the program. Furthermore, the target environment is already a running process, and thus it cannot freely choose at what address the compiled code will be located.

Due to these limitations, handling of references to instructions in the emitted code is significantly different in dynamic assemblers like `dynasm-rs`. As the amount of emitted code grows, the `dynasm-rs` runtime must be prepared to move the emitted code around in memory, and adjust any references that might change during this process.

# Label resolution

Just like in classical assemblers, references to locations in the emitted code are handled via labels (see the [common language reference](./langref_common.md) for all the different label types). These labels can be defined at any point in the to be emitted instruction stream, and code (both emitted before and after the label declaration) can reference these labels in order to create jumps to them or reference data encoded in the instruction stream.

Unlike classical assemblers, it is not possible to perform arbitrary math on these labels or manipulate them as values, as their exact values might not be known at the time these labels are encoded. The runtime does provide limited support for adding / subtracting arbitrary offsets from labels at the moment they are referenced.

# Relocations

When emitted code references a label in a jump or address load, the `dynasm-rs` runtime does not immediately encode it. Instead, it records a so-called relocation. This relocation contains information on the address of the relevant instruction, what it is targetting, how the resulting value should be encoded, and how the value should be adjusted if the runtime has to move the emitted code in memory.

When emitted code is committed, the runtime will resolve all currently outstanding relocations. At this point, all labels referenced in the code need to be defined, else this process will return an `Err(DynasmError)`.

## Managed Relocations

If a relocation needs to be adjusted when the runtime moves emitted code around in memory, it is stored as a managed relocation during the commit process. These relocations will have to be remembered until the assembling process is finalized, and therefore they do come at a performance and memory cost. Luckily, on modern architectures these can be mostly avoided by using position-independent code techniques.

# Position-independent code

Position independent code is code that will run identically, independent of where it is placed in its processes address space. As you can imagine, this is beneficial for preventing `dynasm-rs` from having to use managed relocations. Such code only uses relative offsets for performing jumps and loads/stores of static data. For a dynamic assembler such as `dynasm-rs`, the rules are even looser, as it is also possible to use absolute offsets to any objects that are already present in the address space of the containing process. These objects should then of course not be moved while the emitted code is used.

The exact details of how to create position-independent code depend on the instruction set architecture that is being emitted. One should ensure that only PC-relative instructions are used for any label references into the emitted code, and only absolute instructions are used for any references to static items in the processes address space. Any absolute encodings of references to items in the run-time emitted code, or relative jumps to absolute addresses will result in managed relocations having to be emitted.

# Code patching

If more complex mechanisms than relocations are needed for modifying previously emitted code, `dynasm-rs` supports the ability to modify previously emitted code directly, both before and after having committed it. Modifying uncommitted code is a cheap operation, while modifying committed code potentially requires changing memory permissions.

`dynasm-rs` guarantees that emitted instruction sizes and encodings are predictable at compile time. This ensures that it is possible to patch code in a consistent manner. During the patching process, any outstanding or managed relocations that touch modified code are automatically removed, and new ones can be recorded where necessary.

# Cache management

`dynasm-rs` supports a variety of use cases. One of these is progressive compilation, where execution of dynamically assembled code can happen before the full compilation process is finished. To handle these use cases, `dynasm-rs` provides an `Assembler` implementation that keeps track of what code is ready to be executed, and actual assembling happens to a different, internal buffer. When code is committed, execution has to be paused and the internal buffer is appended to the rest of the executable code. To enhance security, `dynasm-rs` ensures that memory is never writeable and executable at the same time.

Some architectures require additional handling during this process, or during modification of emitted code, as they have incoherent instruction caches. This means that changes made to memory are not directly visible to already cached instruction data. `dynasm-rs` internally ensures that the required cache control operations are executed on these architectures to bring these caches to coherency before execution resumes. `dynasm-rs` also ensures multi-thread coherency here, to code to be assembled in parallel with the execution of previously emitted code.
