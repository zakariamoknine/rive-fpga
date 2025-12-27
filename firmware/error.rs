#[derive(Debug,PartialEq)]
pub enum FirmwareError {
    SdCardNotFound,
    SDCardNotSupported,
    SDCardInvalidGptHeader,

    UartInvalidMagicNumber,
    UartSizeOverflow,
    UartEntryOffsetOverflow,
}
