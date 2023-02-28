// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! Example of how to extend the the rust_xlsxwriter write() method using the
//! IntoExcelData trait to handle arbitrary user data that can be mapped to one
//! of the main Excel data types.

use rust_xlsxwriter::*;

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Make the first column wider for clarity.
    worksheet.set_column_width(0, 12)?;

    // Write user defined type instances that implement the IntoExcelData trait.
    worksheet.write(0, 0, UnixTime::new(0))?;
    worksheet.write(1, 0, UnixTime::new(946598400))?;
    worksheet.write(2, 0, UnixTime::new(1672531200))?;

    // Save the file to disk.
    workbook.save("write_generic.xlsx")?;

    Ok(())
}

// For this example we create a simple struct type to represent a Unix time.
// This is the number of elapsed seconds since the epoch of January 1970 (UTC).
// See https://en.wikipedia.org/wiki/Unix_time. This type isn't handled by
// default by rust_xlsxwriter.
pub struct UnixTime {
    seconds: u64,
}

impl UnixTime {
    pub fn new(seconds: u64) -> UnixTime {
        UnixTime { seconds }
    }
}

// Implement the IntoExcelData trait to map our new UnixTime struct into an
// Excel type.
//
// The relevant Excel type is f64 which is used to store dates and times (along
// with a number format). The Unix 1970 epoch equates to a date/number of
// 25569.0. For Unix times beyond that we divide by the number of seconds in the
// day (24 * 60 * 60) to get the Excel serial date.
//
// We must also supply a number format if one isn't specified since that is
// required for dates.
//
impl IntoExcelData for UnixTime {
    fn write<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: Option<&'a Format>,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        // Convert the Unix time to an Excel datetime.
        let datetime = 25569.0 + (self.seconds / (24 * 60 * 60)) as f64;

        match format {
            Some(format) => {
                // Write the date with the user supplied format.
                worksheet.write_number_with_format(row, col, datetime, format)
            }
            None => {
                // Create a default date format.
                let format = Format::new().set_num_format("yyyy-mm-dd");
                worksheet.write_number_with_format(row, col, datetime, &format)
            }
        }
    }
}
