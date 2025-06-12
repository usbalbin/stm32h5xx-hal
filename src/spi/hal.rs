use embedded_hal::spi::{
    Error as HalError, ErrorKind, ErrorType, Operation, SpiBus, SpiDevice,
};

use super::{
    config::{communication_mode, Yes},
    CommunicationMode, Error, Instance, Spi, TransferWordsNonBlocking, Word,
};

impl HalError for Error {
    fn kind(&self) -> ErrorKind {
        match self {
            Error::Overrun => ErrorKind::Overrun,
            Error::Underrun => ErrorKind::Other,
            Error::ModeFault => ErrorKind::ModeFault,
            Error::Crc => ErrorKind::Other,
            Error::TransferAlreadyComplete => ErrorKind::Other,
            Error::TransactionAlreadyStarted => ErrorKind::Other,
            Error::BufferTooBig { max_size: _ } => ErrorKind::Other,
            Error::InvalidOperation => ErrorKind::Other,
        }
    }
}

impl<SPI, MODE: CommunicationMode, W: Word> ErrorType for Spi<SPI, MODE, W> {
    type Error = Error;
}

impl<SPI: Instance, W: Word> SpiBus<W>
    for Spi<SPI, communication_mode::FullDuplex, W>
where
    super::Inner<SPI, communication_mode::FullDuplex, W>:
        TransferWordsNonBlocking<W>,
{
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        Spi::read(self, words)
    }

    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        Spi::write(self, words)
    }

    fn transfer(
        &mut self,
        read: &mut [W],
        write: &[W],
    ) -> Result<(), Self::Error> {
        Spi::transfer(self, read, write)
    }

    fn transfer_in_place(
        &mut self,
        words: &mut [W],
    ) -> Result<(), Self::Error> {
        Spi::transfer_inplace(self, words)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // This is handled within each of the above functions
        Ok(())
    }
}

impl<SPI: Instance, W: Word> SpiBus<W>
    for Spi<SPI, communication_mode::HalfDuplex, W>
where
    super::Inner<SPI, communication_mode::HalfDuplex, W>:
        TransferWordsNonBlocking<W>,
{
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        Spi::read(self, words)
    }

    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        Spi::write(self, words)
    }

    fn transfer(
        &mut self,
        _read: &mut [W],
        _write: &[W],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn transfer_in_place(
        &mut self,
        _words: &mut [W],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // This is handled within each of the above functions
        Ok(())
    }
}

impl<SPI: Instance, W: Word> SpiBus<W>
    for Spi<SPI, communication_mode::SimplexReceiver, W>
where
    super::Inner<SPI, communication_mode::SimplexReceiver, W>:
        TransferWordsNonBlocking<W>,
{
    fn read(&mut self, words: &mut [W]) -> Result<(), Self::Error> {
        Spi::read(self, words)
    }

    fn write(&mut self, _words: &[W]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn transfer(
        &mut self,
        _read: &mut [W],
        _write: &[W],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn transfer_in_place(
        &mut self,
        _words: &mut [W],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // This is handled within each of the above functions
        Ok(())
    }
}

impl<SPI: Instance, W: Word> SpiBus<W>
    for Spi<SPI, communication_mode::SimplexTransmitter, W>
where
    super::Inner<SPI, communication_mode::SimplexTransmitter, W>:
        TransferWordsNonBlocking<W>,
{
    fn read(&mut self, _words: &mut [W]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn write(&mut self, words: &[W]) -> Result<(), Self::Error> {
        Spi::write(self, words)
    }

    fn transfer(
        &mut self,
        _read: &mut [W],
        _write: &[W],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn transfer_in_place(
        &mut self,
        _words: &mut [W],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // This is handled within each of the above functions
        Ok(())
    }
}

trait OperationExt {
    fn len(&self) -> usize;
}

impl<W> OperationExt for Operation<'_, W> {
    fn len(&self) -> usize {
        match self {
            Operation::Read(words) => words.len(),
            Operation::Write(words) => words.len(),
            Operation::Transfer(read, write) => {
                core::cmp::max(read.len(), write.len())
            }
            Operation::TransferInPlace(words) => words.len(),
            Operation::DelayNs(_) => 0,
        }
    }
}

impl<SPI: Instance, W: Word> Spi<SPI, communication_mode::FullDuplex, W>
where
    super::Inner<SPI, communication_mode::FullDuplex, W>:
        TransferWordsNonBlocking<W>,
{
    #[inline(always)]
    fn perform_operation(
        &mut self,
        operation: &mut Operation<'_, W>,
    ) -> Result<(), Error> {
        match operation {
            Operation::Read(words) => self.inner.read_partial(words),
            Operation::Write(words) => self.inner.write_partial(words),
            Operation::Transfer(read, write) => {
                self.inner.transfer_partial(read, write)
            }
            Operation::TransferInPlace(words) => {
                self.inner.transfer_inplace_partial(words)
            }
            Operation::DelayNs(_) => {
                unimplemented!()
            }
        }
    }
}

impl<SPI: Instance, W: Word> SpiDevice<W>
    for Spi<SPI, communication_mode::FullDuplex, W>
where
    super::Inner<SPI, communication_mode::FullDuplex, W>:
        TransferWordsNonBlocking<W>,
{
    fn transaction(
        &mut self,
        operations: &mut [Operation<'_, W>],
    ) -> Result<(), Self::Error> {
        let len = operations.iter().fold(0, |acc, op| acc + op.len());
        if len == 0 {
            return Ok(());
        }
        self.setup_transaction(len);

        for operation in operations {
            match self.perform_operation(operation) {
                Ok(()) => {}
                Err(error) => {
                    self.abort_transaction();
                    return Err(error);
                }
            }
        }

        self.end_transaction();
        Ok(())
    }
}
