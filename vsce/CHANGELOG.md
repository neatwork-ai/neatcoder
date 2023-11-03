# Change Log

All notable changes to the "neatcoder" extension will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [0.2.0] - 03/11/2023

### Added
- Embedded portable OpenAI Chat in VS Code with
- Added Chat View for creating and managing chats
- `createChat` and `openChat` commands
- MixPanel analytics
- Added retry button in `Tasks Completed`
- Added support for multiple OpenAI Models (Currently GPT3.5 and GPT4)
- Added `Run All Tasks` command and button in `Task Manager` view

### Changed
- Renamed `Task Pool` to `Task Manager` view
- Renamed `Audit Trail` to `Tasks Completed` view
- Renamed `Interfaces` to `Data Models and APIs` view
- Fixed error escalation/handling accross all functionalities

## [0.1.0] - 05/10/2023

### Changed
- `.neat` folder is now lazily created
- Fixed incorrect taskpool state rollback when tasks fail
- Foldable task pool
- Improved performance of scaffolding task
- Guaranteed that only one object gets streamed per file

## [0.1.0-alpha] - 18/09/2023

### Added
- Initial release with basic scaffolding for the Neatcoder project.
- Rust crate with functionalities for managing application state, including the `AppData` struct which holds key information about the project structure and current tasks.
- Various endpoints implemented in the Rust crate for project scaffolding, execution plan management, and code streaming.
- Client-side TypeScript files including the main extension entry point (`extension.ts`), utility functions (`utils.ts`), and HTTP client functionalities (`httpClient.ts`).
- Logging functionalities implemented in `logger.ts` to facilitate debugging and monitoring of the extension.
- A structure for managing application state on the client side, implemented in `appDataManager.ts`.
- Initial structure for unit testing in the `test/` directory.
- Basic setup for providers, commands, handlers, and watchers in respective directories to facilitate extension functionalities like code completion, command execution, event handling, and file monitoring.

### Future Improvements
- Enhancements to the data model as noted in `models/state.rs` to facilitate interactive features and dynamic state management.
- Further development of functionalities within the providers, commands, handlers, and watchers directories to extend the features of the extension.

### Note
- This version is an alpha release and may contain bugs and incomplete features. It serves as a foundation for further development and improvements.
