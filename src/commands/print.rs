use crate::data::NoogleResponse;

pub fn execute(response: &NoogleResponse, filter: Option<&str>) {
    for doc in &response.data {
        for name in doc.all_names() {
            if let Some(namespace) = filter {
                let prefix = format!("{}.", namespace);
                if let Some(stripped) = name.strip_prefix(&prefix) {
                    println!("{}\t{}", stripped, namespace);
                }
            } else {
                println!("{}", name);
            }
        }
    }
}
