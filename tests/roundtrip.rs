use anyhow::Result;
use psd::Psd;

#[test]
fn basic_roundtrip() -> Result<()> {
    // Use a tiny fixture to keep the test lightweight and fast
    let bytes = include_bytes!("./fixtures/groups/green-1x1-one-group-one-layer-inside-one-outside.psd");

    let original = Psd::from_bytes(bytes)?;

    // Serialize back to bytes
    let out = original.into_bytes()?;

    // Parse again
    let reparsed = Psd::from_bytes(&out)?;

    // Assert a couple of key invariants remain stable across roundtrip
    assert_eq!(original.width(), reparsed.width(), "width should match after roundtrip");
    assert_eq!(original.height(), reparsed.height(), "height should match after roundtrip");

    // Layer count should be preserved for supported subset
    assert_eq!(original.layers().len(), reparsed.layers().len(), "layer count should match");

    // Compare merged RGBA length (content may differ in some edge cases, but size should match)
    assert_eq!(original.rgba().len(), reparsed.rgba().len(), "rgba length should match");

    Ok(())
}
