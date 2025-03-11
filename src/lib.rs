use core::fmt::Error;
use kartoffel::*;

/// Wrapper around the kartoffel radar functionality
///
/// This is desgined to try and make reading the radar a bit more consistent.
/// The main problem I had with the radar was the fact it would give different results for 9x9 reads compared to 5x5,
/// this is solved by now giving the most recent info from the radar for whatever tile you request
/// You should use the time values to estimate how 'reliable' the information is
///
pub struct Radar {
    /// the tick that the last scan went off
    pub recent_scan_time: u32,
    /// is the size of the last scan e.g. 3,5,7,9
    pub recent_scan_type: usize,
    /// is an array of which scan type and when it took place for quick searching for each of the relevent sizes e.g. 0->3, 1->5, 2->7, 3->9
    pub scan_time: [(usize, u32); 4],
}

impl Radar {
    fn new() -> Self {
        Radar {
            scan_time: [(3, 0), (5, 0), (7, 0), (9, 0)],
            recent_scan_type: 3,
            recent_scan_time: 0,
        }
    }

    /// Gets the character and time it was checked from the radar
    ///
    /// The radar is bot-centric meaning at(0,0) will always be the bot
    /// and at(-1,0) will always be the square 1 to the left of the bot which means that if you turn (-1,0) will be a different square!
    ///
    /// # Returns
    /// We don't just return the underlying character, but also when the radar scan went off that read that character
    /// this is to give us a good idea of our 'certainty' around a value, you will need to compare it to the current time
    /// with the function `kartoffel::timer_ticks()`
    /// e.g.
    /// ## Example
    /// ```no_run
    /// # use kartoffel::*;
    /// # use radar::Radar;
    ///
    /// let radar = Radar::new();
    ///
    /// // some code here scanning the environment
    ///
    /// let time = timer_ticks();
    /// let (scan_location,scan_time) = radar.at(-1,1);
    /// if time - scan_time < 20_000 {
    ///  // this data is quite fresh
    /// }
    /// ```
    ///
    /// It's worth noting that the times for the radar cooldown are:
    /// 3: 10_000, 5: 15_000, 7: 22_000, 9: 30_000,
    /// with each one having a +- of 10, 15, 25, 30 % respectively
    fn at(&self, x: i8, y: i8) -> Option<(char, u32)> {
        // which scans can we use
        let a_x = x.unsigned_abs();
        let a_y = y.unsigned_abs();
        let mut bigger = (if a_x > a_y { a_x } else { a_y }) as usize;
        if bigger == 0 {
            bigger = 1
        }
        if bigger > 4 {
            return None;
        }
        let (scan_size, scanned_at) = self.scan_time[bigger - 1];
        Some((radar_read(scan_size, x, y, 0) as u8 as char, scanned_at))
    }

    /// Scans in an area
    ///
    /// # Important
    ///
    /// `size` MUST be either 3, 5, 7, or 9.
    ///
    /// # Errors
    ///
    /// This function will return an error if the radar isn't ready
    /// or
    /// the given size is not 3, 5, 7, or 9.
    fn scan(&mut self, size: usize) -> Result<(), Error> {
        if !self.ready() {
            return Err(Error);
        }
        match size {
            3 => {
                radar_scan(3);
                let time = timer_ticks();
                self.scan_time[0] = (3, time);
                self.recent_scan_time = time;
                self.recent_scan_type = 3;
                Ok(())
            }
            5 => {
                radar_scan(5);
                let time = timer_ticks();
                self.scan_time[0] = (5, time);
                self.scan_time[1] = (5, time);
                self.recent_scan_type = 5;
                self.recent_scan_time = time;
                Ok(())
            }
            7 => {
                radar_scan(7);
                let time = timer_ticks();
                self.scan_time[0] = (7, time);
                self.scan_time[1] = (7, time);
                self.scan_time[2] = (7, time);
                self.recent_scan_type = 7;
                self.recent_scan_time = time;
                Ok(())
            }
            9 => {
                radar_scan(9);
                let time = timer_ticks();
                self.scan_time = [(9, time); 4];
                self.recent_scan_type = 9;
                self.recent_scan_time = time;
                Ok(())
            }
            _ => Err(Error),
        }
    }

    /// Returns the time to next possible scan of this [`Radar`].
    /// There is a certian amount of error within this (check the documentation for [`at`][`Radar::at`].)
    fn time_to_next_scan(&self) -> u32 {
        let time = timer_ticks();
        let v = match self.recent_scan_type {
            3 => 10_000,
            5 => 15_000,
            7 => 22_000,
            9 => 30_000,
            _ => 0,
        };
        v - time - self.recent_scan_time
    }

    /// Is the radar ready?
    ///
    /// It's just a wrapper for `kartoffel::is_radar_ready()`
    fn ready(&self) -> bool {
        is_radar_ready()
    }

    /// Wait until radar is ready
    ///
    /// It's just a wrapper for `kartoffel::radar_wait()`
    fn wait(&self) {
        while !self.ready() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_to_next_scan() {
        todo!();
    }
}
