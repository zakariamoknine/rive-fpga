#[derive(Debug)]
pub enum FirmwareError {
    SdCardNotFound,
    SDCardNotSupported,
    SDCardInvalidGptHeader,

    UartInvalidMagicNumber,
    UartSizeOverflow,
    UartEntryOffsetOverflow,
}
