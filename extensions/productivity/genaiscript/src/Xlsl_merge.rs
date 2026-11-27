//! Conservative XLSL workbook merge support.
//!
//! Each source worksheet becomes a separate worksheet in the output workbook.
//! Cell values are copied; formatting, charts, macros, named ranges, and formulas
//! are not preserved. Formula cells are represented by their cached values.

use calamine::{open_workbook, Data, Reader, Xlsx};
use rust_xlsxwriter::Workbook;
use std::{collections::HashSet, convert::TryFrom, path::PathBuf};

const MAX_ROWS: u32 = 1_048_576;
const MAX_COLUMNS: u32 = 16_384;
const MAX_SHEET_NAME_CHARS: usize = 31;

pub(super) fn merge_xlsx_files(
    inputs: Vec<PathBuf>,
    output: PathBuf,
    context: &super::ExecutionContext,
) -> Result<super::MergeReport, super::ExtensionError> {
    let mut output_workbook = Workbook::new();
    let mut used_names = HashSet::new();
    let mut worksheet_count = 0usize;

    for (input_index, input) in inputs.iter().enumerate() {
        context.check_cancelled().map_err(super::execution_error)?;

        let mut source: Xlsx<_> = open_workbook(input).map_err(super::execution_error)?;
        let sheet_names = source.sheet_names().to_owned();
        if sheet_names.is_empty() {
            return Err(super::ExtensionError::InvalidInput(format!(
                "XLSX workbook contains no readable worksheets: {}",
                input.display()
            )));
        }

        let source_prefix = input
            .file_stem()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .unwrap_or("source");

        for source_sheet_name in sheet_names {
            context.check_cancelled().map_err(super::execution_error)?;

            let range = source
                .worksheet_range(&source_sheet_name)
                .map_err(super::execution_error)?;
            let output_sheet_name = unique_sheet_name(
                &format!("{source_prefix} {}", source_sheet_name),
                input_index + 1,
                &mut used_names,
            );

            let worksheet = output_workbook.add_worksheet();
            worksheet
                .set_name(&output_sheet_name)
                .map_err(super::execution_error)?;

            let (start_row, start_column) = range.start().unwrap_or((0, 0));
            for (row_offset, cells) in range.rows().enumerate() {
                context.check_cancelled().map_err(super::execution_error)?;
                let row = start_row
                    .checked_add(u32::try_from(row_offset).map_err(super::execution_error)?)
                    .ok_or_else(|| {
                        super::ExtensionError::InvalidInput(
                            "XLSX worksheet row index overflow".into(),
                        )
                    })?;
                if row >= MAX_ROWS {
                    return Err(super::ExtensionError::InvalidInput(format!(
                        "worksheet '{}' exceeds the XLSX row limit",
                        source_sheet_name
                    )));
                }

                for (column_offset, cell) in cells.iter().enumerate() {
                    let column = start_column
                        .checked_add(u32::try_from(column_offset).map_err(super::execution_error)?)
                        .ok_or_else(|| {
                            super::ExtensionError::InvalidInput(
                                "XLSX worksheet column index overflow".into(),
                            )
                        })?;
                    if column >= MAX_COLUMNS {
                        return Err(super::ExtensionError::InvalidInput(format!(
                            "worksheet '{}' exceeds the XLSX column limit",
                            source_sheet_name
                        )));
                    }
                    let column = u16::try_from(column).map_err(super::execution_error)?;

                    match cell {
                        Data::Empty => {}
                        Data::Int(value)
                            if (-9_007_199_254_740_992i64..=9_007_199_254_740_992i64)
                                .contains(value) =>
                        {
                            worksheet
                                .write_number(row, column, *value as f64)
                                .map_err(super::execution_error)?;
                        }
                        Data::Int(value) => {
                            worksheet
                                .write_string(row, column, value.to_string())
                                .map_err(super::execution_error)?;
                        }
                        Data::Float(value) if value.is_finite() => {
                            worksheet
                                .write_number(row, column, *value)
                                .map_err(super::execution_error)?;
                        }
                        Data::Float(value) => {
                            worksheet
                                .write_string(row, column, value.to_string())
                                .map_err(super::execution_error)?;
                        }
                        Data::String(value) => {
                            worksheet
                                .write_string(row, column, value)
                                .map_err(super::execution_error)?;
                        }
                        Data::Bool(value) => {
                            worksheet
                                .write_boolean(row, column, *value)
                                .map_err(super::execution_error)?;
                        }
                        other => {
                            worksheet
                                .write_string(row, column, other.to_string())
                                .map_err(super::execution_error)?;
                        }
                    }
                }
            }
            worksheet_count += 1;
        }
    }

    if worksheet_count == 0 {
        return Err(super::ExtensionError::InvalidInput(
            "no readable worksheets were found in the XLSX inputs".into(),
        ));
    }

    context.check_cancelled().map_err(super::execution_error)?;
    let bytes = output_workbook
        .save_to_buffer()
        .map_err(super::execution_error)?;
    super::write_new_output(&output, &bytes)?;

    Ok(super::MergeReport {
        output: output.display().to_string(),
        inputs: inputs
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        bytes_written: bytes.len() as u64,
    })
}

fn unique_sheet_name(candidate: &str, source_index: usize, used: &mut HashSet<String>) -> String {
    let sanitized = sanitize_sheet_name(candidate);
    let mut attempt = 1usize;

    loop {
        let suffix = if attempt == 1 {
            String::new()
        } else {
            format!(" ({attempt})")
        };
        let max_base_chars = MAX_SHEET_NAME_CHARS.saturating_sub(suffix.chars().count());
        let mut name: String = sanitized.chars().take(max_base_chars).collect();
        name.push_str(&suffix);

        if name.is_empty() {
            name = format!("Sheet{source_index}{suffix}");
        }
        let key = name.to_lowercase();
        if used.insert(key) {
            return name;
        }
        attempt += 1;
    }
}

fn sanitize_sheet_name(candidate: &str) -> String {
    let mut name: String = candidate
        .chars()
        .map(|character| {
            if matches!(character, '[' | ']' | ':' | '*' | '?' | '/' | '\\') {
                '_'
            } else {
                character
            }
        })
        .collect();
    name = name.trim().trim_matches('\'').to_owned();
    if name.is_empty() {
        name = "Sheet".into();
    }
    name.chars().take(MAX_SHEET_NAME_CHARS).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::{Data, Reader, Xlsx};
    use std::path::Path;
    use tempfile::tempdir;

    fn write_source(path: &Path, sheet_name: &str, label: &str, number: f64) {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(sheet_name).unwrap();
        worksheet.write_string(0, 0, label).unwrap();
        worksheet.write_number(0, 1, number).unwrap();
        worksheet.write_boolean(1, 0, true).unwrap();
        workbook.save(path).unwrap();
    }

    #[test]
    fn xlsx_merge_appends_sheets_and_preserves_source_workbooks() {
        let directory = tempdir().unwrap();
        let first = directory.path().join("first.xlsx");
        let second = directory.path().join("second.xlsx");
        let output = directory.path().join("combined.xlsx");
        write_source(&first, "Data", "first workbook", 12.5);
        write_source(&second, "Data", "second workbook", 42.0);

        super::merge_xlsx_files(
            vec![first.clone(), second.clone()],
            output.clone(),
            &super::super::ExecutionContext::default(),
        )
        .unwrap();

        let mut merged: Xlsx<_> = open_workbook(&output).unwrap();
        let names = merged.sheet_names().to_owned();
        assert_eq!(names.len(), 2);
        assert_ne!(names[0].to_lowercase(), names[1].to_lowercase());

        let first_sheet = merged.worksheet_range(&names[0]).unwrap();
        assert_eq!(
            first_sheet.get_value((0, 0)),
            Some(&Data::String("first workbook".into()))
        );
        assert_eq!(first_sheet.get_value((0, 1)), Some(&Data::Float(12.5)));

        let second_sheet = merged.worksheet_range(&names[1]).unwrap();
        assert_eq!(
            second_sheet.get_value((0, 0)),
            Some(&Data::String("second workbook".into()))
        );
        assert_eq!(second_sheet.get_value((1, 0)), Some(&Data::Bool(true)));
        assert!(first.exists() && second.exists());
    }
}
