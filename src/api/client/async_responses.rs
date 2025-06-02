use super::PyResponse;
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseStats {
    pub durations: Vec<u64>,
    pub responses: Vec<u16>,
    pub total_duration: u64,
}

#[pyclass(name = "AsyncResponse")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyAsyncResponses {
    #[pyo3(get)]
    pub responses: Vec<PyResponse>,

    pub responses_stats: ResponseStats,
}

#[pymethods]
impl PyAsyncResponses {
    fn print_nth(&self, i: isize) -> PyResult<()> {
        if i < 0 {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Index cannot be negative",
            ));
        } else if i as usize >= self.responses.len() {
            return Err(pyo3::exceptions::PyIndexError::new_err(format!(
                "Index out of bounds: {} (size: {})",
                i,
                self.responses.len()
            )));
        }

        let response = self.responses.get(i as usize).ok_or_else(|| {
            pyo3::exceptions::PyIndexError::new_err(format!("No response found at index {}", i))
        })?;

        let headers: String = response
            .headers
            .iter()
            .map(|(k, v)| format!("({}: {})", k, v))
            .collect::<Vec<String>>()
            .join(",\n    ");

        println!("Response:");
        println!("  Status: {} {}", response.status_code, response.status);
        println!("  Duration: {:}", response.duration);
        println!("  Headers:\n    {}", headers);
        println!(
            "  Body:\n{}",
            response
                .body
                .as_ref()
                .map_or("None".to_string(), |b| b.clone())
        );

        Ok(())
    }

    fn amount(&self) -> usize {
        self.responses.len()
    }

    fn print_stats(&self, chunk: u64, cut_off: u32) {
        let total_responses: usize = self.responses_stats.responses.len();
        let total_duration_sum: u64 = self.responses_stats.durations.iter().sum();
        let average_duration = if total_responses > 0 {
            total_duration_sum as f64 / total_responses as f64
        } else {
            0.0
        };
        let slowest = self.responses_stats.durations.iter().max().unwrap_or(&0);
        let fastest = self.responses_stats.durations.iter().min().unwrap_or(&0);
        let req_per_sec = if average_duration > 0.0 {
            total_responses as f64 / (total_duration_sum as f64 / 1000.0)
        } else {
            0.0
        };

        // (from ms, to ms, amount, histogram_chars)
        const MAX_CHARS: usize = 20;
        let mut responses_by_time: Vec<(u64, u64, u32, u32)> = Vec::new();
        for duration in &self.responses_stats.durations {
            let from = (duration / chunk) * chunk;
            let to = from + chunk;
            if let Some(entry) = responses_by_time
                .iter_mut()
                .find(|(f, t, _, _)| *f == from && *t == to)
            {
                entry.2 += 1;
            } else {
                responses_by_time.push((from, to, 1, 0));
            }
        }
        responses_by_time
            .iter_mut()
            .for_each(|(_, _, amount, chars)| {
                *chars = if *amount > 0 {
                    (*amount as f64 / total_responses as f64 * MAX_CHARS as f64).floor() as u32 + 1
                } else {
                    0
                };
            });
        responses_by_time.sort_by(|a, b| a.0.cmp(&b.0));

        // (code, status, amount, histogram_chars)
        let mut status_codes: Vec<(u16, String, u64, u64)> = Vec::new();
        for response in &self.responses {
            if let Some(entry) = status_codes
                .iter_mut()
                .find(|(code, _, _, _)| *code == response.status_code)
            {
                entry.2 += 1;
            } else {
                status_codes.push((response.status_code, response.status.clone(), 1, 0));
            }
        }
        for (_, _, amount, chars) in &mut status_codes {
            *chars = if *amount > 0 {
                (*amount as f64 / total_responses as f64 * MAX_CHARS as f64).floor() as u64 + 1
            } else {
                0
            };
        }
        status_codes.sort_by(|a, b| a.0.cmp(&b.0));

        println!("Total Responses: {}", total_responses);
        println!("Total: {} ms", self.responses_stats.total_duration);
        println!("Average: {:.4} ms", average_duration);
        println!("Slowest: {} ms", slowest);
        println!("Fastest: {} ms", fastest);
        println!("Req/s: {:.2}", req_per_sec);
        println!();
        println!("Responses by time (cut off: {}%):", cut_off);
        for (from, to, amount, histogram_chars) in responses_by_time {
            if amount as f64 / total_responses as f64 * 100.0 < cut_off as f64 {
                continue;
            }

            println!(
                "{:>5}-{:<5} - {:<5} |{}",
                from,
                to,
                amount,
                "#".repeat(histogram_chars as usize)
            );
        }
        println!();
        println!("Status Codes:");
        for (_, status, amount, histogram_chars) in status_codes {
            println!(
                "  [{:>30}] - {:<5} |{}",
                status,
                amount,
                "#".repeat(histogram_chars as usize)
            );
        }
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyAsyncResponses>()?;

    Ok(())
}
