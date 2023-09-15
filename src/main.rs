use serde::Deserialize;

fn main() {
    let input_filepath = "input/job_schedule.json";
    let file_data = std::fs::read_to_string(input_filepath)
        .expect("Should have been able to read the file");
    let jobs: JobSchedules = serde_json::from_str(&file_data).expect("Could not parse JSON file");

    if jobs.jobs.len() == 0 {
        println!("JSON file is empty.");
        return;
    }

    let (min_hour, max_hour) = parse_min_and_max_hour(&jobs);
    let mut load = vec![];
    for hour in min_hour..=max_hour {
        for minute in 0..=59 {
            let unit = parse_unit_usage_at(minute, hour, &jobs);
            println!("load for {:02}:{:02} => {}", hour, minute, unit);
            load.push(unit);
        }
    }

    println!("Max Unit at any time: {}", load.iter().max().unwrap());
}

fn parse_min_and_max_hour(jobs: &JobSchedules) -> (u64, u64) {
    let mut max_hour = 0;
    let mut min_hour = 24;

    for job in jobs.jobs.iter() {
        if job.start_hour > job.end_hour {
            panic!("Job finishes before starting!");
        }

        if job.start_hour < min_hour {
            min_hour = job.start_hour;
        }
        if job.end_hour > max_hour {
            max_hour = job.end_hour;
        }
    }

    (min_hour, max_hour)
}

fn parse_unit_usage_at(minute: u64, hour: u64, jobs: &JobSchedules) -> u64 {
    let mut load = 0;

    for job in jobs.jobs.iter() {
        let started_after = hour > job.start_hour || (job.start_hour == hour && minute >= job.start_minute);
        let not_finished = job.end_hour > hour || (job.end_hour == hour && job.end_minute >= minute);

        if started_after && not_finished {
            load += job.load;
        }
    }

    load
}


#[derive(Deserialize, Debug)]
struct JobSchedules {
    jobs: Vec<JobSchedule>
}

#[derive(Deserialize, Debug)]
struct JobSchedule {
    load: u64,
    start_hour: u64,
    start_minute: u64,
    end_hour: u64,
    end_minute: u64,
}

#[cfg(test)]
mod tests {
    use super::{JobSchedules, JobSchedule, parse_unit_usage_at};

    #[test]
    fn correct_unit_count() {
        let job_schedule = JobSchedules {
            jobs: vec![
                JobSchedule {
                    load: 11,
                    start_hour: 9,
                    start_minute: 10,
                    end_hour: 10,
                    end_minute: 10,
                }
            ]
        };
        assert_eq!(parse_unit_usage_at(10, 9, &job_schedule), 11);
        assert_eq!(parse_unit_usage_at(11, 10, &job_schedule), 0);

        let job_schedule2 = JobSchedules {
            jobs: vec![
                JobSchedule {
                    load: 1,
                    start_hour: 11,
                    start_minute: 58,
                    end_hour: 16,
                    end_minute: 3,
                }
            ]
        };
        assert_eq!(parse_unit_usage_at(1, 16, &job_schedule2), 1);
    }

    #[test]
    fn correct_unit_count_multiple() {
        let job_schedule = JobSchedules {
            jobs: vec![
                JobSchedule {
                    load: 2,
                    start_hour: 9,
                    start_minute: 10,
                    end_hour: 10,
                    end_minute: 10,
                },
                JobSchedule {
                    load: 1,
                    start_hour: 9,
                    start_minute: 10,
                    end_hour: 10,
                    end_minute: 10,
                }
            ]
        };
        assert_eq!(parse_unit_usage_at(10, 9, &job_schedule), 3);
        assert_eq!(parse_unit_usage_at(11, 10, &job_schedule), 0);
    }
}