use rocket::State;
use rocket_contrib::JSON;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize)]
pub struct Execution {
    repo: String,
    init: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExecutionCreated {
    id: Uuid,
    execution: Execution,
}

#[post("/execution", format = "application/json", data = "<execution>")]
pub fn execution(
    execution: JSON<Execution>,
    ref_execs: State<Arc<Mutex<HashMap<Uuid, Execution>>>>
) -> JSON<ExecutionCreated> {
    let id = Uuid::new_v4();
    let execution =  execution.into_inner();
    let created = ExecutionCreated { execution: execution.clone(), id: id };
    ref_execs.lock().unwrap().insert(id, execution);
    return JSON(created);
}

#[get("/execution/<id>")]
pub fn get_execution(id: String, ref_execs: State<Arc<Mutex<HashMap<Uuid, Execution>>>>) -> JSON<Option<ExecutionCreated>> {
    let exec_id = Uuid::parse_str(&id).unwrap();
    let val = match ref_execs.lock().unwrap().get(&exec_id) {
        Some(exec) => Some(ExecutionCreated { id: exec_id, execution: exec.clone() }),
        None => None
    };
    JSON(val)
}

#[cfg(test)]
#[path = "./execution_test.rs"]
mod execution_test;
