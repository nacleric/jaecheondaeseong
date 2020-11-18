use super::*;
#[test]
fn delete_glyph__backward_delete_an_empty_buffer_does_nothing() {
    // Given
    let expected_opt_glyph = None;
    let expected_buffer = Buffer::new();
    let mut sut = Buffer::new();

    // When
    let res = sut.delete_glyph(Direction::Backward);

    // Then
    assert_eq!(res, expected_opt_glyph);
    assert_eq!(sut, expected_buffer);
}

#[test]
fn delete_glyph__forward_delete_an_empty_buffer_does_nothing() {
    // Given
    let expected_opt_glyph = None;
    let expected_buffer = Buffer::new();
    let mut sut = Buffer::new();

    // When
    let res = sut.delete_glyph(Direction::Forward);

    // Then
    assert_eq!(res, expected_opt_glyph);
    assert_eq!(sut, expected_buffer);
}

// Note: invalid position error because their is no space in front of the index to be able to delete. Maybe pad the buffer?
#[test]
fn delete_glyph__backward_delete_sole_glyph_returns_empty_buffer() {
    // Given
    let glyph = 'a';
    let expected_opt_glyph = Some(glyph);
    let expected_buffer = Buffer::new();
    let mut sut = Buffer::new();
    sut.insert_glyph(glyph);
    let cursor_pos = 1;
    sut.set_pos(Position::new(cursor_pos, 0)).unwrap();

    // When
    let res = sut.delete_glyph(Direction::Backward);

    // Then
    assert_eq!(res, expected_opt_glyph);
    assert_eq!(sut, expected_buffer);
}

// Note: Deletion isn't happening
#[test]
fn delete_glyph__forward_delete_sole_glyph_returns_empty_buffer() -> Result<()> {
    // Given
    let glyph = 'a';
    let expected_opt_glyph = Some(glyph);
    let expected_buffer = Buffer::new();
    let mut sut = Buffer::new();
    sut.insert_glyph(glyph);
    sut.set_pos(Position::new(0, 0))?;

    // When
    let res = sut.delete_glyph(Direction::Forward);

    // Then
    assert_eq!(res, expected_opt_glyph);
    assert_eq!(sut, expected_buffer);
    Ok(())
}

// Doesn't work yet
#[test]
fn delete_glyph__backward_delete_from_data_model_removes_the_expected_glyph(
) -> Result<()> {
    // Given
    let bad_sentence = String::from("greenb sleeping mask");
    let expected_buffer = String::from("green sleeping mask");
    let expected_opt_glyph = Some('b');
    let glyph_pos = 6;
    let mut sut = Buffer::new();
    sut.insert_glyphs(bad_sentence.chars());
    sut.set_pos(Position::new(glyph_pos, 0))?;

    // When
    let res = sut.delete_glyph(Direction::Backward);

    // Then
    assert_eq!(res, expected_opt_glyph);
    assert_eq!(sut.contents(), expected_buffer.as_bytes());
    Ok(())
}

#[test]
fn delete_glyph__forward_delete_from_data_model_removes_the_expected_glyph() -> Result<()> {
    // Given
    let bad_sentence = String::from("greenb sleeping mask");
    let expected_buffer = String::from("green sleeping mask");
    let expected_opt_glyph = Some('b');
    let mut sut = Buffer::new();
    sut.insert_glyphs(bad_sentence.chars());
    let cursor_pos = 5;
    sut.set_pos(Position::new(cursor_pos, 0))?;

    // When
    let res = sut.delete_glyph(Direction::Forward);

    // Then
    assert_eq!(res, expected_opt_glyph);
    assert_eq!(sut.contents(), expected_buffer.as_bytes());
    Ok(())
}