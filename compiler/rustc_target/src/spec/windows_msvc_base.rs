use crate::spec::TargetOptions;

pub fn opts() -> TargetOptions {
    let base = super::msvc_base::opts();

    TargetOptions {
        os: "windows".to_string(),
        env: "msvc".to_string(),
        vendor: "pc".to_string(),
        dynamic_linking: true,
        dll_prefix: String::new(),
        dll_suffix: ".dll".to_string(),
        exe_suffix: ".exe".to_string(),
        staticlib_prefix: String::new(),
        staticlib_suffix: ".lib".to_string(),
        families: vec!["windows".to_string()],
        crt_static_allows_dylibs: true,
        crt_static_respected: true,
        requires_uwtable: true,
        // Currently we don't pass the /NODEFAULTLIB flag to the linker on MSVC
        // as there's been trouble in the past of linking the C++ standard
        // library required by LLVM. This likely needs to happen one day, but
        // in general Windows is also a more controlled environment than
        // Unix, so it's not necessarily as critical that this be implemented.
        //
        // Note that there are also some licensing worries about statically
        // linking some libraries which require a specific agreement, so it may
        // not ever be possible for us to pass this flag.
        no_default_libraries: false,
        // When reading this function you might ask "why is this inlined
        // everywhere other than Windows?", and that's a very reasonable
        // question to ask. The short story is that it segfaults rustc if
        // this function is inlined. The longer story is that Windows looks
        // to not support `extern` references to thread locals across DLL
        // boundaries. This appears to at least not be supported in the ABI
        // that LLVM implements.
        //
        // Because of this we never inline on Windows, but we do inline on
        // other platforms (where external references to thread locals
        // across DLLs are supported). A better fix for this would be to
        // inline this function on Windows, but only for "statically linked"
        // components. For example if two separately compiled rlibs end up
        // getting linked into a DLL then it's fine to inline this function
        // across that boundary. It's only not fine to inline this function
        // across a DLL boundary. Unfortunately rustc doesn't currently
        // have this sort of logic available in an attribute, and it's not
        // clear that rustc is even equipped to answer this (it's more of a
        // Cargo question kinda). This means that, unfortunately, Windows
        // gets the pessimistic path for now where it's never inlined.
        //
        // The issue of "should enable on Windows sometimes" is #84933
        extern_thread_local_references: false,

        ..base
    }
}
