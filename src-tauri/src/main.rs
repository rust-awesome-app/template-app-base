// #![allow(unused)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use crate::ipc::{
	create_project, create_task, delete_project, delete_task, get_project, get_task,
	list_projects, list_tasks, update_project, update_task,
};
use crate::prelude::*;
use model::{ProjectForCreate, TaskForCreate};
use std::sync::Arc;
use store::Store;
mod ctx;
mod error;
mod event;
mod ipc;
mod model;
mod prelude;
mod store;
mod utils;

// --- re-exports
// pub use error::Error;

#[tokio::main]
async fn main() -> Result<()> {
	// let store: Arc<StoreManager> = Arc::new(StoreManager::default());
	let store = Store::new().await?;
	let store = Arc::new(store);

	// for dev only
	seed_store(store.clone()).await?;

	tauri::Builder::default()
		.manage(store)
		.invoke_handler(tauri::generate_handler![
			// Project
			get_project,
			create_project,
			update_project,
			delete_project,
			list_projects,
			// Task
			get_task,
			create_task,
			update_task,
			delete_task,
			list_tasks,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");

	Ok(())
}

/// Only use while developing. Convenient when to seed the store on start of the application.
async fn seed_store(store: Arc<Store>) -> Result<()> {
	let ps = ["A", "B"]
		.into_iter()
		.map(|k| {
			(
				k,
				ProjectForCreate {
					name: format!("Project {k}"),
				},
			)
		})
		.collect::<Vec<_>>();

	for (k, project) in ps.into_iter() {
		let project_id =
			store.exec_create::<ProjectForCreate>("project", project.into()).await?;

		for i in 1..=200 {
			let done = if i % 2 == 0 { true } else { false };
			let task = TaskForCreate {
				project_id: project_id.clone(),
				title: format!("Task {k}.{i}"),
				desc: None,
				done: Some(done),
			};

			store.exec_create::<TaskForCreate>("task", task.into()).await?;
		}
	}

	Ok(())
}
