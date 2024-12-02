use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub enum ReportStatus {
    Valid,
    Invalid,
}

#[derive(Debug, Clone)]
pub enum LevelStatus {
    Increasing,
    Decreasing,
}

pub fn solve_part_one() {
    let input_filepath = "inputs/day2/data.txt";
    let file = File::open(input_filepath).unwrap();
    let reader = BufReader::new(file);

    let mut reports = Vec::new();

    for line in reader.lines() {
        let mut report = vec![];
        for level in line.unwrap().split(' ') {
            let level = level.parse::<i64>().unwrap();
            report.push(level);
        }
        reports.push(report);
    }

    let mut valid_reports = 0;
    for report in reports {
        let mut report_status = ReportStatus::Valid;
        let mut level_status = None;

        for level in 1..report.len() {
            let prev_value = report[level - 1];
            let current_value = report[level];
            let diff = prev_value - current_value;

            if (1..=3).contains(&diff) || (-3..=-1).contains(&diff) {
                match &level_status {
                    Some(value) => {
                        if diff < 0 && !matches!(value, LevelStatus::Decreasing) {
                            report_status = ReportStatus::Invalid;
                            break;
                        } else if diff > 0 && !matches!(value, LevelStatus::Increasing) {
                            report_status = ReportStatus::Invalid;
                            break;
                        }
                    }
                    None => {
                        if diff >= 1 {
                            level_status = Some(LevelStatus::Increasing);
                        } else {
                            level_status = Some(LevelStatus::Decreasing);
                        }
                    }
                }
            } else {
                report_status = ReportStatus::Invalid;
                break;
            }
        }

        if matches!(report_status, ReportStatus::Valid) {
            valid_reports += 1;
        }
    }

    println!("Result day 2/1: {:?}", valid_reports)
}

pub fn solve_part_two() {
    let input_filepath = "inputs/day2/data.txt";
    let file = File::open(input_filepath).unwrap();
    let reader = BufReader::new(file);

    let mut reports = Vec::new();

    for line in reader.lines() {
        let mut report = vec![];
        for level in line.unwrap().split(' ') {
            let level = level.parse::<i64>().unwrap();
            report.push(level);
        }
        reports.push(report);
    }

    let mut maybe_invalid = Vec::new();

    let mut valid_reports = 0;
    for report in reports {
        let mut report_status = ReportStatus::Valid;
        let mut level_status = None;

        for index in 1..report.len() {
            let prev_value = report[index - 1];
            let current_value = report[index];
            let diff = prev_value - current_value;

            if (1..=3).contains(&diff) || (-3..=-1).contains(&diff) {
                match &level_status {
                    Some(value) => {
                        if diff < 0 && !matches!(value, LevelStatus::Decreasing) {
                            report_status = ReportStatus::Invalid;
                            maybe_invalid.push((index - 1, report));
                            break;
                        } else if diff > 0 && !matches!(value, LevelStatus::Increasing) {
                            report_status = ReportStatus::Invalid;
                            maybe_invalid.push((index - 1, report));
                            break;
                        }
                    }
                    None => {
                        if diff >= 1 {
                            level_status = Some(LevelStatus::Increasing);
                        } else {
                            level_status = Some(LevelStatus::Decreasing);
                        }
                    }
                }
            } else {
                report_status = ReportStatus::Invalid;
                maybe_invalid.push((index - 1, report));
                break;
            }
        }

        if matches!(report_status, ReportStatus::Valid) {
            valid_reports += 1;
        }
    }

    for (anchor, origin_report) in maybe_invalid {
        let indexes = if anchor == 0 {
            vec![anchor, anchor + 1]
        } else {
            vec![0, anchor, anchor + 1]
        };
        println!("org: {:?}", origin_report);
        for index in indexes {
            let mut report = origin_report.clone();
            report.remove(index);

            let mut report_status = ReportStatus::Valid;
            let mut level_status = None;

            for index in 1..report.len() {
                let prev_value = report[index - 1];
                let current_value = report[index];
                println!("{} - {}", prev_value, current_value);
                let diff = prev_value - current_value;

                if (1..=3).contains(&diff) || (-3..=-1).contains(&diff) {
                    match &level_status {
                        Some(value) => {
                            if diff < 0 && !matches!(value, LevelStatus::Decreasing) {
                                report_status = ReportStatus::Invalid;
                                break;
                            } else if diff > 0 && !matches!(value, LevelStatus::Increasing) {
                                report_status = ReportStatus::Invalid;
                                break;
                            }
                        }
                        None => {
                            if diff >= 1 {
                                level_status = Some(LevelStatus::Increasing);
                            } else {
                                level_status = Some(LevelStatus::Decreasing);
                            }
                        }
                    }
                } else {
                    report_status = ReportStatus::Invalid;
                    break;
                }
            }

            if matches!(report_status, ReportStatus::Valid) {
                valid_reports += 1;
                break;
            }
        }
    }

    println!("Result day 2/2: {:?}", valid_reports)
}
