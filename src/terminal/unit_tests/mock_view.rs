use super::*;
use crate::terminal::mock_terminal::MockTerminalView;
use crate::terminal::unit_tests::dummy_adapter::TtyControlDummyAdapter;

// Ask Brad about Constructor dependency injection
#[test]
fn new__clears_entire_view() {
    // Given
    let expected_view_state = [[None; WIDTH]; HEIGHT];
    // fake_view (mock, fake, spy, stub)
    let mock_view = MockTerminalView::new();
    let mut sut = Terminal::new(mock_view, TtyControlDummyAdapter::new());

    // When
    sut.clear().unwrap();

    // Then
    assert_eq!(sut.view_buffer().get_data(), expected_view_state);
}

#[test]
fn write_to_buffer__writes_grapheme_into_viewbuffer() {
    // Given
    let expected_view_state: [[Option<char>; WIDTH]; HEIGHT] = [[Some('a'); WIDTH]; HEIGHT];
    let mock_view = MockTerminalView::new();
    let mut sut = Terminal::new(mock_view, TtyControlDummyAdapter::new());

    // When
    sut.write_to_buffer('a');

    // Then
    assert_eq!(sut.view_buffer().get_data(), expected_view_state);
}

// #[test]
// fn print_to_screen__renders_graphemes_to_mockview() {
//     // Given
//     let expected_view_state: [[Option<char>; WIDTH]; HEIGHT] = [[Some('a'); WIDTH]; HEIGHT];
//     let mock_view = MockTerminalView::new();
//     let mut sut = Terminal::new(mock_view, TtyControlDummyAdapter::new());

//     // When
//     sut.print_to_screen('a');

//     // Then
//     assert_eq!(sut.view_buffer().get_data(), expected_view_state);
// }
