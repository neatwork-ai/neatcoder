use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, ops::Deref};

use crate::Sample;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Tasks(BTreeMap<usize, Task>);

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Task {
    /// Task to be performed
    task: String,
    /// Specialist assigned to perform the task
    role: Option<String>,
}

impl Tasks {
    // TODO: Add fallback logic in case if fails to split
    pub fn parse_str(text: &str, col_delimiter: &str) -> Result<Self> {
        let result_map: BTreeMap<usize, Task> = text
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let parts: Vec<&str> = line.splitn(2, col_delimiter).collect();
                match parts.as_slice() {
                    [task, role] => Some((
                        index,
                        Task {
                            task: task
                                .trim_start_matches(char::is_whitespace)
                                .trim_start_matches("- ")
                                .to_string(),
                            role: Some(role.trim().to_string()),
                        },
                    )),
                    [task] => Some((
                        index,
                        Task {
                            task: task.trim_start_matches("- ").to_string(),
                            role: None,
                        },
                    )),
                    _ => None,
                }
            })
            .collect();

        // Print the resulting HashMap
        for (index, task) in &result_map {
            println!(
                "Index: {}, Task: {}, Role: {:?}",
                index, task.task, task.role
            );
        }

        Ok(Tasks(result_map))
    }
}

impl Sample for Tasks {
    fn sample() -> Self {
        let sample = vec![
            (
                0,
                Task {
                    task: "Task A to be performed".to_string(),
                    role: Some("Specialist assigned to perform the task".to_string()),
                },
            ),
            (
                1,
                Task {
                    task: "Task B to be performed".to_string(),
                    role: Some("Specialist assigned to perform the task".to_string()),
                },
            ),
            (
                2,
                Task {
                    task: "Task C to be performed".to_string(),
                    role: Some("Specialist assigned to perform the task".to_string()),
                },
            ),
        ];

        Self(sample.into_iter().collect())
    }

    fn sample_json() -> Result<String> {
        Ok(serde_json::to_string(&Self::sample())?)
    }
}

impl Deref for Tasks {
    type Target = BTreeMap<usize, Task>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correct_string() -> Result<()> {
        let text = r#"
            - Conduct market research to identify target audience and demand for prompt templates --> Marketing team
            - Develop a user-friendly website or platform for Promptify --> Development team
            - Design an appealing and intuitive user interface for the platform --> Design team
            - Create a database to store and manage prompt templates --> Database team
            - Implement a search and filtering system to easily find prompt templates --> Development team
            - Develop a secure payment system for purchasing prompt templates --> Development and Finance team
            - Establish partnerships with prompt template creators to source high-quality templates --> Business development team
            - Implement a rating and review system for users to provide feedback on prompt templates --> Development team
            - Set up a customer support system to assist users with any issues or inquiries --> Customer support team
            - Develop a marketing strategy to promote Promptify and attract users --> Marketing team
            - Analyze user data and feedback to continuously improve the platform --> Data analysis team
            - Regularly update and maintain the platform to ensure it is secure and functioning properly --> Development and IT team
        "#.trim_start_matches(char::is_whitespace);

        let actual = Tasks::parse_str(text, " --> ")?;

        let entries = vec![
        (0, Task {task: "Conduct market research to identify target audience and demand for prompt templates".to_string(), role: Some("Marketing team".to_string())}),
        (1, Task {task: "Develop a user-friendly website or platform for Promptify".to_string(), role: Some("Development team".to_string())}),
        (2, Task {task: "Design an appealing and intuitive user interface for the platform".to_string(), role: Some("Design team".to_string())}),
        (3, Task {task: "Create a database to store and manage prompt templates".to_string(), role: Some("Database team".to_string())}),
        (4, Task {task: "Implement a search and filtering system to easily find prompt templates".to_string(), role: Some("Development team".to_string())}),
        (5, Task {task: "Develop a secure payment system for purchasing prompt templates".to_string(), role: Some("Development and Finance team".to_string())}),
        (6, Task {task: "Establish partnerships with prompt template creators to source high-quality templates".to_string(), role: Some("Business development team".to_string())}),
        (7, Task {task:  "Implement a rating and review system for users to provide feedback on prompt templates".to_string(), role: Some("Development team".to_string())}),
        (8, Task {task: "Set up a customer support system to assist users with any issues or inquiries".to_string(), role: Some("Customer support team".to_string())}),
        (9, Task {task: "Develop a marketing strategy to promote Promptify and attract users".to_string(), role: Some( "Marketing team".to_string())}),
        (10, Task {task: "Analyze user data and feedback to continuously improve the platform".to_string(), role: Some("Data analysis team".to_string())}),
        (11, Task {task: "Regularly update and maintain the platform to ensure it is secure and functioning properly".to_string(), role: Some("Development and IT team".to_string())}),
    ];

        let expected: Tasks = Tasks(entries.into_iter().collect());

        assert_eq!(actual[&0], expected[&0]);
        assert_eq!(actual[&1], expected[&1]);
        assert_eq!(actual[&2], expected[&2]);
        assert_eq!(actual[&3], expected[&3]);
        assert_eq!(actual[&4], expected[&4]);
        assert_eq!(actual[&5], expected[&5]);
        assert_eq!(actual[&6], expected[&6]);
        assert_eq!(actual[&7], expected[&7]);
        assert_eq!(actual[&8], expected[&8]);
        assert_eq!(actual[&9], expected[&9]);
        assert_eq!(actual[&10], expected[&10]);
        assert_eq!(actual[&11], expected[&11]);

        Ok(())
    }
}
