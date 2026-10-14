//! Execute with scripts/test-wasm.cjs; no browser, WASI, or host imports.

#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    Box::into_raw(vec![0u8; len].into_boxed_slice()).cast()
}

/// # Safety
/// The buffer must be a live allocation from `alloc` of exactly `len` bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run(ptr: *mut u8, len: usize) {
    // SAFETY: the test runner passes the exact allocation back once.
    let font = unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len)) };
    let cases = [
        "flowchart LR\n A[Hello] --> B[World]",
        "sequenceDiagram\n Alice->>Bob: Hello",
        "classDiagram\n Animal <|-- Duck",
        "stateDiagram-v2\n [*] --> Ready",
        "pie title Pets\n \"Dogs\" : 60\n \"Cats\" : 40",
        "erDiagram\n CUSTOMER ||--o{ ORDER : places",
        "flowchart LR\n A[日本語] --> B[Hello 🌍]",
    ];
    // First prove default fallback rendering does not need a clock or filesystem.
    for source in cases {
        let result =
            mermaid_rs_renderer::render_with_detailed_timing(source, Default::default()).unwrap();
        assert!(result.svg.contains("<svg"));
        assert_eq!(result.parse_us, 0);
        assert_eq!(result.layout_us, 0);
        assert_eq!(result.render_us, 0);
    }
    assert!(mermaid_rs_renderer::set_text_measurement_font(vec![0; 8], 0).is_err());
    if !font.is_empty() {
        mermaid_rs_renderer::set_text_measurement_font(font.into_vec(), 0).unwrap();
        for source in cases {
            let first = mermaid_rs_renderer::render_strict(source, Default::default()).unwrap();
            let second = mermaid_rs_renderer::render_strict(source, Default::default()).unwrap();
            assert_eq!(first, second);
        }
    }
    assert!(mermaid_rs_renderer::render_strict("not a diagram", Default::default()).is_err());
}
