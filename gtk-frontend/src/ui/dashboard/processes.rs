use crate::ui::common::table::Table;
use crate::ui::Refresh;
use gtk::prelude::*;
use sysinfo::{ProcessExt, SystemExt};

pub struct Processes {
    pub table: Table,
}

impl Processes {
    pub fn new() -> Self {
        let column_names = [
            "PID",
            "User ID",
            "Status",
            "CPU%",
            "MEM KiB",
            "Process Name",
        ];
        let column_types = [
            u32::static_type(),
            u32::static_type(),
            String::static_type(),
            String::static_type(),
            u32::static_type(),
            String::static_type(),
        ];

        let table = Table::new(&column_names, &column_types);

        Self { table }
    }
}

impl Refresh for Processes {
    fn refresh(&self, system: &sysinfo::System) {
        let processes_list = system.get_processes();
        self.table.get_store().clear();

        for (pid, process) in processes_list.iter() {
            if process.cmd().len() != 0 {
                self.table.get_store().insert_with_values(
                    None,
                    &[0, 1, 2, 3, 4, 5],
                    &[
                        pid,
                        // &extract_username(process.environ()),
                        &process.uid,
                        &format!("{:?}", process.status()),
                        &format!("{:.1}", process.cpu_usage()),
                        &process.memory(),
                        &process.cmd().join(" "),
                    ],
                );
            }
        }
    }
}

// fn extract_username(environ: &[String]) -> String {
//     println!("{:?}", environ);
//     let user = environ.iter().find(|line| {
//         if line.len() > 9 && &line[..=8] == "USERNAME=" {
//             return true;
//         }
//         false
//     });
//     let user = match user {
//         Some(user) => user.split("=").nth(1).unwrap(),
//         None => "Unknown",
//     };

//     String::from(user)
// }
