use anyhow::Result;
use indexmap::IndexMap;
use openapiv3::{
    Components, OpenAPI, Operation, PathItem, Paths, ReferenceOr, Responses,
    Server,
};

pub trait PreProcess {
    fn pre_process(&mut self) -> Result<()>;
}

impl PreProcess for OpenAPI {
    fn pre_process(&mut self) -> Result<()> {
        // Components
        if let Some(components) = &mut self.components {
            components.pre_process()?;
        }

        // Paths
        self.paths.pre_process()?;

        self.servers
            .iter_mut()
            .try_for_each(|server| server.pre_process())?;

        // Nuke extensions
        // Note: These can have references as well - just FYI in case we use this in the future
        self.extensions = IndexMap::new();

        // Nuke docs
        self.external_docs = None;

        Ok(())
    }
}

// TODO: This is not working properly for some unknown reason...
// Use utils::resolve_references for the timebeing...
impl PreProcess for Components {
    fn pre_process(&mut self) -> Result<()> {
        // Nuke Responses
        self.responses = IndexMap::new();

        // TODO: Missing normalising all other fields

        Ok(())
    }
}

impl PreProcess for Paths {
    fn pre_process(&mut self) -> Result<()> {
        self.paths.iter_mut().try_for_each(|(_, path)| {
            if let ReferenceOr::Item(path) = path {
                return path.pre_process();
            }

            Ok(())
        })?;

        self.extensions = IndexMap::new();

        Ok(())
    }
}

impl PreProcess for PathItem {
    fn pre_process(&mut self) -> Result<()> {
        if let Some(ref mut description) = &mut self.description {
            if description.chars().count() > 300 {
                // Take only the first `max_len` characters.
                *description = description.chars().take(300).collect();
            }
        }

        if let Some(get) = &mut self.get {
            get.pre_process()?;
        }

        if let Some(put) = &mut self.put {
            put.pre_process()?;
        }

        if let Some(post) = &mut self.post {
            post.pre_process()?;
        }

        if let Some(delete) = &mut self.delete {
            delete.pre_process()?;
        }

        if let Some(options) = &mut self.options {
            options.pre_process()?;
        }

        if let Some(head) = &mut self.head {
            head.pre_process()?;
        }

        if let Some(patch) = &mut self.patch {
            patch.pre_process()?;
        }

        if let Some(trace) = &mut self.trace {
            trace.pre_process()?;
        }

        self.servers
            .iter_mut()
            .try_for_each(|server| server.pre_process())?;

        Ok(())
    }
}

impl PreProcess for Operation {
    fn pre_process(&mut self) -> Result<()> {
        if let Some(ref mut description) = &mut self.description {
            if description.chars().count() > 300 {
                // Take only the first `max_len` characters.
                *description = description.chars().take(300).collect();
            }
        }

        if let Some(ref mut operation_id) = &mut self.operation_id {
            *operation_id = operation_id.replace("/", "--");
        }

        // Nuke responses
        self.responses = Responses {
            default: None,
            responses: IndexMap::new(),
            extensions: IndexMap::new(),
        };

        Ok(())
    }
}

impl PreProcess for Server {
    fn pre_process(&mut self) -> Result<()> {
        if self.url.starts_with("http://") {
            self.url = self.url.replacen("http://", "https://", 1);
        }

        Ok(())
    }
}
