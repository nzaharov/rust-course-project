use reqwest::blocking;

const URL: &'static str = "http://localhost:8080/api/sysinfo";

pub fn get_sys_list() -> Result<Vec<String>, &'static str> {
    let list = blocking::get(URL)
        .map_err(|_| "Get failed")?
        .json::<Vec<String>>()
        .map_err(|_| "Parse failed")?;

    Ok(list)
}
