use core::convert::TryInto;
use embedded_hal::digital::v2::{InputPin, OutputPin, PinState};

pub struct Matrix<C, R, const CS: usize, const RS: usize>
where
    C: OutputPin,
    R: InputPin,
{
    cols: [C; CS],
    rows: [R; RS],
}

impl<C: OutputPin, R: InputPin, const CS: usize, const RS: usize> Matrix<C, R, CS, RS>
where
    C: OutputPin,
    R: InputPin,
{
    /// Given a list of GPIO pins, return a new Matrix object
    pub fn new<E>(
        cols: [C; CS],
        rows: [R; RS],
    ) -> Result<Self, E>
    where
        C: OutputPin<Error = E>,
        R: InputPin<Error = E>,
    {
        let mut res = Self {
            cols,
            rows,
        };
        res.clear()?;
        Ok(res)
    }

    /// Set every output pin to LOW (inputs are pull-up)
    pub fn clear<E>(&mut self) -> Result<(), E>
    where
        C: OutputPin<Error = E>,
        R: InputPin<Error = E>,
    {
        for c in self.cols.iter_mut() {
            c.set_low()?;
        }
        Ok(())
    }

    /// Set the selected column to LOW, and set everything else to HIGH
    fn select_column<E>(&mut self, col: usize)
    where
        C: OutputPin<Error = E>,
        R: InputPin<Error = E>,
    {
        for bit in 0..self.cols.len() {
            let state: u8 = ((col & (0b1 << bit)) >> bit).try_into().unwrap();
            if state == 0 {
                let _ = self.cols[bit].set_state(PinState::Low);
            } else if state == 1 {
                let _ = self.cols[bit].set_state(PinState::High);
            }
        }
    }

    /// Return the list of every key pressed or not in the bitmap format.
    /// Actual scanning is done here.
    pub fn get_raw<E>(&mut self) -> Result<[[bool; CS]; RS], E>
    where
        C: OutputPin<Error = E>,
        R: InputPin<Error = E>,
    {
        let mut keys = [[false; CS]; RS];

        for current_col in 0..CS {
            self.select_column(current_col);
            cortex_m::asm::delay(5000);
            for (ri, row) in (&mut self.rows).iter_mut().enumerate() {
                keys[ri][current_col] = row.is_low()?;
            }
        }
        Ok(keys)
    }
}
