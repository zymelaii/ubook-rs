use crate::BackendCore;
use async_mutex::Mutex;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Default)]
pub struct BackendManager {
    backends: HashMap<String, Arc<Mutex<Box<dyn BackendCore>>>>,
    active_id: Option<String>,
    enabled_set: HashSet<String>,
}

impl BackendManager {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BackendManager {
    pub fn add_backend(&mut self, backend: Box<dyn BackendCore>) -> bool {
        let backend_id = backend.backend_id();
        if self.backends.contains_key(backend_id) {
            false
        } else {
            #[allow(clippy::arc_with_non_send_sync)]
            self.backends
                .insert(backend_id.into(), Arc::new(Mutex::new(backend)));
            true
        }
    }

    pub fn remove_backend(&mut self, backend_id: &str) -> bool {
        self.backends.remove(backend_id).is_some()
    }

    pub fn get_backend_list(&self) -> Vec<String> {
        self.backends.keys().map(Clone::clone).collect()
    }

    pub fn try_get_instance_of(
        &self,
        backend_id: &str,
    ) -> crate::Result<Arc<Mutex<Box<dyn BackendCore>>>> {
        if let Some(instance) = self.backends.get(backend_id) {
            Ok(instance.clone())
        } else {
            anyhow::bail!("unknonw backend `{backend_id}`")
        }
    }

    pub fn active_backend_id(&self) -> Option<String> {
        self.active_id.clone()
    }

    pub fn try_get_active_backend(&self) -> crate::Result<Arc<Mutex<Box<dyn BackendCore>>>> {
        let id = self.active_id.clone().unwrap_or_default();
        self.try_get_instance_of(id.as_str())
    }

    pub fn is_available(&self, backend_id: &str) -> bool {
        self.backends.contains_key(backend_id)
    }

    pub fn is_enabled(&self, backend_id: &str) -> bool {
        self.enabled_set.contains(backend_id)
    }

    pub fn enable(&mut self, backend_id: &str) -> crate::Result<()> {
        if self.backends.contains_key(backend_id) {
            self.enabled_set.insert(backend_id.into());
            Ok(())
        } else {
            anyhow::bail!("invalid backend `{backend_id}`")
        }
    }

    pub fn disable(&mut self, backend_id: &str) -> crate::Result<()> {
        if self.backends.contains_key(backend_id) {
            self.enabled_set.remove(backend_id);
            if let Some(true) = self.active_id.as_ref().map(|id| id == backend_id) {
                self.active_id = None;
            }
            Ok(())
        } else {
            anyhow::bail!("invalid backend `{backend_id}`")
        }
    }

    pub fn activate(&mut self, backend_id: &str) -> crate::Result<()> {
        self.enable(backend_id)?;
        self.active_id = Some(backend_id.into());
        Ok(())
    }
}
