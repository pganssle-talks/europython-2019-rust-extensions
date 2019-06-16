use pyo3::prelude::*;
use pyo3::types::{PyDate, PyDateAccess};
use pyo3::wrap_pyfunction;

static SECONDS_BEFORE_1970: i64 = 62135596800;
static DAYS_BEFORE_MONTH: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

fn is_leap(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

/// Calculate epoch time from a PyDate object
fn to_timestamp(date: &PyDate) -> i64 {
    let year = date.get_year() - 1;
    let month = date.get_month() - 1;
    let day = date.get_day();

    let mut days = 365 * year + (year / 4) - (year / 100) + (year / 400);

    days += DAYS_BEFORE_MONTH[month as usize];
    if is_leap(year) && month > 2 || (month == 2 && day == 29) {
        days += 1;
    }

    days += day as i32;
    let seconds: i64 = (days as i64) * 86400;
    seconds - SECONDS_BEFORE_1970
}

#[pyfunction]
fn seconds_before<'p>(py: Python<'p>, d: &PyDate, seconds: i64) -> PyResult<&'p PyDate> {
    let timestamp = to_timestamp(&d);

    PyDate::from_timestamp(py, timestamp - seconds)
}

#[pymodule]
fn date_ex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(seconds_before))?;

    Ok(())
}
