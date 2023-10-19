use anyhow::{anyhow, Result};
use js_sys::JsString;
use js_sys::{Error, Function};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

use crate::models::task::Task;
use crate::models::task_pool::Pipeline;
use crate::{
    endpoints::{
        scaffold_project::scaffold_project,
        stream_code::{stream_code, CodeGenParams},
    },
    models::task_params::{TaskParams, TaskType},
    openai::params::OpenAIParams,
    JsError, WasmType,
};

use super::language::Language;
use super::{
    interfaces::{Interface, SchemaFile},
    task_pool::TaskPool,
};

// NOTE: We will need to perform the following improvements to the data model:
//
// 1. The extension itself will be interactive, and will not rely solely on an
// initial prompt but rather a sequence of prompts, or even a tree of prompts.
// there are different models we can use to model this. We can think of modeling
// as a chat app like Slack, in which each message can have a Thread or we can
// generalise it further to something more intricate.
//
// 2. This struct is storing static application data such as `scaffold`,
// `codebase`. we will need to find a way to make the application state dynamic
// such that it reflects the current state of the codebase at any given time. We
// should also consider if have the field `codebase` makes sense here, because
// we can also access the codebase via the Language Server on the client side.
//
/// Acts as a shared application data (i.e. shared state). It contains
/// information related to the initial prompt, the scaffold of the project, its
/// interfaces, and current jobs in the TODO pipeline among others (see `Jobs`).
#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub(crate) language: Option<Language>,
    /// Initial prompt containing the specifications of the project
    pub(crate) specs: Option<String>,
    /// JSON String containing the File System Scaffold
    /// Example:
    /// ```json
    /// {
    ///     "src": {
    ///       "config.rs": "Module for handling configuration variables",
    ///       "db.rs": "Module for establishing and managing database connections",
    ///       "handlers": {
    ///         "company.rs": "Module for handling company-related API endpoints",
    ///         "customer.rs": "Module for handling customer-related API endpoints",
    ///         "order.rs": "Module for handling order-related API endpoints",
    ///         "product.rs": "Module for handling product-related API endpoints"
    ///       },
    ///       "main.rs": "Main entry point of the API server",
    ///       "models": {
    ///         "company.rs": "Module defining the Company struct and its database operations",
    ///         "customer.rs": "Module defining the Customer struct and its database operations",
    ///         "order.rs": "Module defining the Order struct and its database operations",
    ///         "product.rs": "Module defining the Product struct and its database operations"
    ///       },
    ///       "routes.rs": "Module for defining API routes and their corresponding handlers",
    ///       "utils.rs": "Module for utility functions and helper methods"
    ///     }
    ///   }
    /// ```
    pub(crate) scaffold: Option<String>,
    /// Vector of strings containing the interface config files (e.g. SQL DLLs,
    /// etc.) The BTreeMap represents BTreeMap<Interface Name, Interface>
    pub(crate) interfaces: BTreeMap<String, Interface>,
    pub(crate) task_pool: TaskPool,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Record<string, Interface>")]
    pub type IInterfaces;

    #[wasm_bindgen(typescript_type = "Record<string, string>")]
    pub type ICodebase;

    #[wasm_bindgen(typescript_type = "Array<Task>")]
    pub type ITasksVec;
}

#[wasm_bindgen]
impl AppState {
    #[wasm_bindgen(constructor)]
    pub fn new(
        language: Option<Language>,
        specs: Option<String>,
        scaffold: Option<String>,
        interfaces: IInterfaces,
        task_pool: TaskPool,
    ) -> Result<AppState, JsValue> {
        let interfaces = BTreeMap::from_extern(interfaces)?;
        Ok(Self {
            language,
            specs,
            scaffold,
            interfaces,
            task_pool,
        })
    }

    #[wasm_bindgen(js_name = castToString)]
    pub fn cast_to_string(&self) -> Result<JsString, JsError> {
        let json = serde_json::to_string(self)
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        Ok(json.into())
    }

    #[wasm_bindgen(js_name = castFromString)]
    pub fn cast_from_string(json: String) -> Result<AppState, JsError> {
        let app_state = serde_json::from_str(&json)
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        Ok(app_state)
    }

    pub fn empty() -> Self {
        Self {
            language: None,
            specs: None,
            scaffold: None,
            interfaces: BTreeMap::new(),
            task_pool: TaskPool::empty(),
        }
    }

    #[wasm_bindgen(js_name = removeAllTodos)]
    pub fn remove_all_todos(&mut self) {
        self.task_pool.todo = Pipeline::empty();
    }

    #[wasm_bindgen(getter)]
    pub fn specs(&self) -> Option<JsString> {
        match &self.specs {
            Some(specs) => Some(specs.clone().into()),
            None => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn scaffold(&self) -> Option<JsString> {
        match &self.scaffold {
            Some(scaffold) => Some(scaffold.clone().into()),
            None => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn interfaces(&self) -> Result<IInterfaces, JsError> {
        BTreeMap::to_extern(self.interfaces.clone())
    }

    #[wasm_bindgen(getter, js_name = taskPool)]
    pub fn task_pool(&self) -> TaskPool {
        self.task_pool.clone()
    }

    #[wasm_bindgen(js_name = getTodoTasks)]
    pub fn get_todo_tasks(&self) -> Result<ITasksVec, JsError> {
        let tasks: Vec<_> = self
            .task_pool
            .todo
            .order
            .iter()
            .filter_map(|&id| self.task_pool.todo.tasks.get(&id))
            .cloned()
            .collect();

        Vec::to_extern(tasks)
    }

    #[wasm_bindgen(js_name = getDoneTasks)]
    pub fn get_done_tasks(&self) -> Result<ITasksVec, JsError> {
        let tasks: Vec<_> = self
            .task_pool
            .done
            .order
            .iter()
            .filter_map(|&id| self.task_pool.done.tasks.get(&id))
            .cloned()
            .collect();

        Vec::to_extern(tasks)
    }

    #[wasm_bindgen(js_name = popTodo)]
    pub fn pop_todo(&mut self, task_id: usize) -> Result<Task, JsError> {
        self.task_pool.pop_todo(task_id)
    }

    #[wasm_bindgen(js_name = addDone)]
    pub fn add_done(&mut self, task: Task) {
        self.task_pool.add_done(task)
    }

    #[wasm_bindgen(js_name = addBackTodo)]
    pub fn add_back_todo(&mut self, task: Task) {
        self.task_pool.add_back_todo(task)
    }

    #[wasm_bindgen(js_name = setInterfaces)]
    pub fn set_interfaces(
        &mut self,
        interfaces: IInterfaces,
    ) -> Result<(), JsError> {
        if !self.interfaces.is_empty() {
            return Err(anyhow!("Data model already exists"))
                .map_err(|e| Error::new(&e.to_string()).into());
        }

        let interfaces = BTreeMap::from_extern(interfaces)?;
        self.interfaces = interfaces;

        Ok(())
    }

    #[wasm_bindgen(js_name = setLanguage)]
    pub fn set_language(&mut self, language: Language) {
        self.language = Some(language);
    }

    #[wasm_bindgen(js_name = addSchema)]
    pub fn add_schema(
        &mut self,
        interface_name: String,
        schema_name: String,
        schema: SchemaFile,
    ) -> Result<(), JsError> {
        self.add_schema_(interface_name, schema_name, schema)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = removeSchema)]
    pub fn remove_schema(
        &mut self,
        interface_name: &str,
        schema_name: &str,
    ) -> Result<(), JsError> {
        self.remove_schema_(interface_name, schema_name)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = addInterface)]
    pub fn add_interface(
        &mut self,
        new_interface: Interface,
    ) -> Result<(), JsError> {
        self.add_interface_(new_interface)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = removeInterface)]
    pub fn remove_interface(
        &mut self,
        interface_name: &str,
    ) -> Result<(), JsError> {
        self.remove_interface_(interface_name)
            .map_err(|e| Error::new(&e.to_string()).into())
    }

    #[wasm_bindgen(js_name = scaffoldProject)]
    pub async fn scaffold_project(
        &mut self,
        ai_params: &OpenAIParams,
        task_params: TaskParams,
        request_callback: &Function,
    ) -> Result<(), JsError> {
        let task_params = task_params
            .scaffold_project_()
            .ok_or("No ScaffoldProject field. This error should not occur.")
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        self.specs = Some(task_params.specs.clone());

        let language = self.language.as_ref().ok_or_else(|| {
            JsError::from_str("Failed to retrieve a language")
        })?;

        let (scaffold_json, files) = scaffold_project(
            language,
            ai_params,
            task_params,
            request_callback,
        )
        .await
        .map_err(|e| JsError::from_str(&e.to_string()))?;

        // Add code writing jobs to the task pool
        for file in files.iter() {
            let filename = file.name.clone();
            let description = file.description.clone();

            let task_params = TaskParams::new_(
                TaskType::CodeGen,
                Box::new(CodeGenParams {
                    filename,
                    description,
                }),
            )
            .map_err(|e| JsError::from_str(&e.to_string()))?;

            self.task_pool
                .add_todo(&file.name, &file.description, task_params);
        }

        self.scaffold = Some(scaffold_json.to_string());

        Ok(())
    }

    #[wasm_bindgen(js_name = streamCode)]
    pub fn stream_code(
        &mut self,
        ai_params: &OpenAIParams,
        task_params: TaskParams,
        codebase: ICodebase,
    ) -> Result<String, JsError> {
        let task_params = task_params
            .stream_code_()
            .ok_or("No StreamCode field. This error should not occur.")
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        let codebase = BTreeMap::from_extern(codebase)?;

        let req_body = stream_code(self, ai_params, task_params, codebase)
            .map_err(|e| JsError::from_str(&e.to_string()))?;

        Ok(req_body)
    }
}

impl AppState {
    pub fn new_(
        language: Option<Language>,
        specs: Option<String>,
        scaffold: Option<String>,
        interfaces: BTreeMap<String, Interface>,
        task_pool: TaskPool,
    ) -> Self {
        Self {
            language,
            specs,
            scaffold,
            interfaces,
            task_pool,
        }
    }

    fn add_schema_(
        &mut self,
        interface_name: String,
        schema_name: String,
        schema: SchemaFile,
    ) -> Result<()> {
        if !self.interfaces.contains_key(&interface_name) {
            return Err(anyhow!("[ERROR] The interface does not exist. Please create an interface first."));
        }

        // Safe to call `expect` due to previous check
        let interface = self.interfaces.get_mut(&interface_name).expect(
            "Unable to locate the interface. This error should not occur.",
        );

        // Replaces the existing interface if any
        interface
            .insert_schema(schema_name, schema)
            .map_err(|e| anyhow!("{:?}", e))?;

        Ok(())
    }

    pub fn remove_schema_(
        &mut self,
        interface_name: &str,
        schema_name: &str,
    ) -> Result<()> {
        if !self.interfaces.contains_key(interface_name) {
            return Err(anyhow!("[ERROR] The interface does not exist."));
        }

        // Safe to unwrap due to previous check
        let interface = self.interfaces.get_mut(interface_name).expect(
            "Unable to locate the interface. This error should not occur.",
        );

        // Replaces the existing interface if any
        interface
            .remove_schema(schema_name)
            .map_err(|e| anyhow!("{:?}", e))?;

        Ok(())
    }

    pub fn add_interface_(&mut self, new_interface: Interface) -> Result<()> {
        let interface_name = new_interface.name();

        if self.interfaces.contains_key(&interface_name) {
            // TODO: We need proper error escalation and communication with the
            // client
            eprintln!("[ERROR] The interface already exists. Skipping.");

            return Err(anyhow!("Interface already exists"));
        }

        self.interfaces
            .insert(interface_name.to_string(), new_interface);

        Ok(())
    }

    pub fn remove_interface_(&mut self, interface_name: &str) -> Result<()> {
        if !self.interfaces.contains_key(interface_name) {
            // TODO: We need proper error escalation and communication with the
            // client
            eprintln!("[ERROR] The interface does not exist. Skipping.");

            return Err(anyhow!("Interface does not exist"));
        }

        self.interfaces.remove(interface_name);

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::VecDeque;

    use crate::{
        endpoints::scaffold_project::ScaffoldParams,
        models::{
            interfaces::{
                apis::{Api, ApiType},
                dbs::{Database, DbType},
            },
            language::LanguageType,
            task::Task,
            task_pool::Pipeline,
        },
    };

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    pub fn test_app_state_new() {
        let json = r#"{
            "specs":null,
            "scaffold":null,
            "interfaces":{
                "aaa":{
                    "interfaceType":"Database",
                    "inner":{
                        "database":{
                            "name":"aaa",
                            "dbType":"ClickHouse",
                            "customType":null,
                            "port":null,
                            "host":null,
                            "schemas":{}
                        }
                    },
                    "storage":null,
                    "api":null
                }
            },
            "taskPool":{
                "counter":0,
                "todo":{
                    "tasks":{},
                    "order":[]
                },
                "done":{
                    "tasks":{},
                    "order":[]
                }
            }
        }"#;

        let app_state = AppState::cast_from_string(json.to_string());

        if let Err(e) = app_state {
            panic!("Failed to create AppState: {:?}", e);
        }
    }

    #[wasm_bindgen_test]
    pub fn app_state_deserialize() {
        let mut interfaces = BTreeMap::new();
        interfaces.insert(
            String::from("MyDB"),
            Interface::new_db(Database::new_(
                String::from("MyDB"),
                DbType::MySql,
                BTreeMap::from([(
                    "MySchema".to_string(),
                    "schema".to_string(),
                )]),
            )),
        );

        interfaces.insert(
            String::from("MyApi"),
            Interface::new_api(Api::new_(
                String::from("MyApi"),
                ApiType::RestfulApi,
                BTreeMap::from([(
                    "MySchema".to_string(),
                    "schema".to_string(),
                )]),
            )),
        );

        let task_1 = Task::new(
            1,
            "Task1",
            "Description1",
            TaskParams::new_(
                TaskType::ScaffoldProject,
                Box::new(ScaffoldParams::new(String::from("specs"))),
            )
            .unwrap(),
        );
        let task_2 = Task::new(
            2,
            "Task2",
            "Description2",
            TaskParams::new_(
                TaskType::CodeGen,
                Box::new(CodeGenParams::new(
                    String::from("filename.rs"),
                    String::from("description"),
                )),
            )
            .unwrap(),
        );

        let todo = Pipeline::new_(
            BTreeMap::from([(2, task_2)]),
            VecDeque::from([3, 2]),
        );

        let done =
            Pipeline::new_(BTreeMap::from([(1, task_1)]), VecDeque::from([1]));

        let task_pool = TaskPool::new(
            3, // The number of tasks
            todo, done,
        );

        let app_state = AppState::new_(
            Some(Language::new(LanguageType::Rust)),
            Some(String::from("specs")),
            Some(String::from("scaffold")),
            interfaces,
            task_pool,
        );

        let actual = AppState::cast_to_string(&app_state)
            .unwrap()
            .as_string()
            .unwrap();

        let expected = String::from(
            r#"{"language":{"language":"Rust","custom":null},"specs":"specs","scaffold":"scaffold","interfaces":{"MyApi":{"interfaceType":"Api","inner":{"database":null,"storage":null,"api":{"name":"MyApi","apiType":"RestfulApi","customType":null,"port":null,"host":null,"schemas":{"MySchema":"schema"}}}},"MyDB":{"interfaceType":"Database","inner":{"database":{"name":"MyDB","dbType":"MySql","customType":null,"port":null,"host":null,"schemas":{"MySchema":"schema"}},"storage":null,"api":null}}},"taskPool":{"counter":3,"todo":{"tasks":{"2":{"id":2,"name":"Task2","description": "Description2","taskParams":{"taskType":"CodeGen","inner":{"scaffoldProject":null,"streamCode":{"filename":"filename.rs"}}},"status":"Todo"}},"order":[2]},"done":{"tasks":{"1":{"id":1,"name":"Task1","description": "Description1", "taskParams":{"taskType":"ScaffoldProject","inner":{"scaffoldProject":{"specs":"specs"},"streamCode":null}},"status":"Todo"}},"order":[1]}}}"#,
        );

        assert_eq!(actual, expected);

        let app_state = AppState::cast_from_string(actual);

        if let Err(e) = app_state {
            panic!("Failed to create AppState: {:?}", e);
        }
    }

    #[wasm_bindgen_test]
    pub fn app_state_deserialize_2() {
        let mut interfaces = BTreeMap::new();
        interfaces.insert(
            String::from("aaa"),
            Interface::new_db(Database::new_(
                String::from("aaa"),
                DbType::ClickHouse,
                BTreeMap::new(),
            )),
        );

        let todo = Pipeline::new_(BTreeMap::new(), VecDeque::from([]));
        let done = Pipeline::new_(BTreeMap::new(), VecDeque::from([]));

        let task_pool = TaskPool::new(
            0, // The number of tasks
            todo, done,
        );

        let app_state = AppState::new_(
            Some(Language::new(LanguageType::Rust)),
            None,
            None,
            interfaces,
            task_pool,
        );

        let actual = AppState::cast_to_string(&app_state)
            .unwrap()
            .as_string()
            .unwrap();

        let expected = String::from(
            r#"{"language":{"language":"Rust","custom":null},"specs":null,"scaffold":null,"interfaces":{"aaa":{"interfaceType":"Database","inner":{"database":{"name":"aaa","dbType":"ClickHouse","customType":null,"port":null,"host":null,"schemas":{}},"storage":null,"api":null}}},"taskPool":{"counter":0,"todo":{"tasks":{},"order":[]},"done":{"tasks":{},"order":[]}}}"#,
        );

        assert_eq!(actual, expected);

        let app_state = AppState::cast_from_string(expected);

        if let Err(e) = app_state {
            panic!("Failed to create AppState: {:?}", e);
        }
    }

    #[wasm_bindgen_test]
    pub fn app_state_deserialize_empty() {
        let app_state_x = AppState::empty();

        // Deserialized
        let actual =
            JsValue::from_str(&serde_json::to_string(&app_state_x).unwrap());

        let expected = JsValue::from_str(
            r#"{"language":null,"specs":null,"scaffold":null,"interfaces":{},"taskPool":{"counter":0,"todo":{"tasks":{},"order":[]},"done":{"tasks":{},"order":[]}}}"#,
        );

        assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    pub fn gracefully_handles_stringified_objects() {
        let expected = JsValue::from_str(
            r#"{"specs":null,"scaffold":null,"interfaces":"{\"aaa\":{\"
    interfaceType\":\"Database\",\"inner\":{\"database\":{\"name\":\"aaa\",\"
    dbType\":\"ClickHouse\",\"customType\":null,\"port\":null,\"host\":null,\
    "schemas\":{}},\"storage\":null,\"api\":null}}}","taskPool":{"counter":0,
    "todo":{"tasks":{},"order":[]},"done":{"tasks":{},"order":[]}}}"#,
        );

        if expected.as_string().is_none() {
            panic!("Upsie")
        }
    }
}
