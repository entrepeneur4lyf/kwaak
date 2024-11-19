use anyhow::Result;
use async_trait::async_trait;
use std::{
    io::{BufWriter, Cursor, Read as _},
    path::Path,
};
use swiftide::{
    query::StreamExt as _,
    traits::{Command, ToolExecutor},
};
use tokio::io::AsyncReadExt as _;
use tracing::{error, info};

use bollard::{
    container::{
        Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
        StopContainerOptions,
    },
    exec::{CreateExecOptions, StartExecResults},
    image::{BuildImageOptions, CreateImageOptions},
    Docker,
};
use ignore::{gitignore::GitignoreBuilder, WalkBuilder};
use tokio_tar::{Builder, Header};

use crate::repository::{self, Repository};

/// Starts up a docker container from the dockerfile configured in the repository
///
/// - Build a docker image with bollard and start it up
/// - Ensure current context (ie the code) is present in the container
/// - implement the Workspace trait for this
///
#[derive(Clone)]
pub struct DockerExecutor {
    container_id: String,
    docker: Docker,
    // docker file?
    // handle?
}

#[async_trait]
impl ToolExecutor for DockerExecutor {
    async fn exec_cmd(&self, cmd: &Command) -> Result<swiftide::traits::Output> {
        let Command::Shell(cmd) = cmd else {
            anyhow::bail!("Command not implemented")
        };

        let exec = self
            .docker
            .create_exec(
                &self.container_id,
                CreateExecOptions {
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    cmd: Some(cmd.split_whitespace().collect::<Vec<_>>()),
                    ..Default::default()
                },
            )
            .await?
            .id;

        let mut response = String::new();

        if let StartExecResults::Attached { mut output, .. } =
            self.docker.start_exec(&exec, None).await?
        {
            while let Some(Ok(msg)) = output.next().await {
                response.push_str(&msg.to_string());
            }
        } else {
            todo!();
        }
        Ok(response.into())
    }
}

impl DockerExecutor {
    /// Starts a docker container from the dockerfile configured in the repository
    ///
    /// Method is a bit loaded, tar'in the full context (respecting ignore)
    /// and then booting up the container
    ///
    /// TODO: Ensure that drop can be handled on panic
    /// TODO: Should not build and run container in constructor
    pub async fn from_repository(repository: &Repository) -> Result<DockerExecutor> {
        let docker = Docker::connect_with_socket_defaults().unwrap();

        // TODO: Handle dockerfile not being named `Dockerfile` or missing
        // let dockerfile_path = &repository.config().docker.dockerfile;
        let context_path = &repository.config().docker.context;

        let context = build_context_as_tar(context_path).await?;

        // Step 2: Build the Docker image using the in-memory tar archive
        let image_name = format!("kwaak-{}", repository.config().project_name);
        let build_options = BuildImageOptions {
            t: image_name.as_str(),
            rm: true,
            ..Default::default()
        };

        let mut build_stream = docker.build_image(build_options, None, Some(context.into()));

        while let Some(log) = build_stream.next().await {
            match log {
                Ok(output) => {
                    if let Some(stream) = output.stream {
                        info!("{}", stream);
                    }
                }
                Err(e) => error!("Error during build: {:?}", e),
            }
        }

        // Step 3: Start a container with the built image
        let config = Config {
            image: Some(image_name.as_str()),
            host_config: Some(bollard::models::HostConfig {
                auto_remove: Some(true),
                ..Default::default()
            }),
            ..Default::default()
        };

        let create_options = CreateContainerOptions {
            name: image_name.as_str(),
            ..Default::default()
        };

        let container_id = docker
            .create_container(Some(create_options), config)
            .await?
            .id;

        docker
            .start_container(&image_name, None::<StartContainerOptions<String>>)
            .await?;

        Ok(DockerExecutor {
            container_id,
            // TODO: should we clone the docker client here?
            docker: docker.clone(),
        })
    }
}

impl Drop for DockerExecutor {
    fn drop(&mut self) {
        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                self.docker
                    .stop_container(
                        &self.container_id,
                        Some(StopContainerOptions {
                            ..Default::default()
                        }),
                    )
                    .await
            })
        });

        if let Err(e) = result {
            tracing::error!(error = %e, "Could not stop container");
        }
    }
}

async fn build_context_as_tar(context_path: &Path) -> Result<Vec<u8>> {
    let buffer = Vec::new();

    let mut tar = Builder::new(buffer);
    // Load .dockerignore and .gitignore rules
    let mut ignore_builder = GitignoreBuilder::new(context_path);
    let dockerignore_path = context_path.join(".dockerignore");
    if dockerignore_path.exists() {
        ignore_builder.add(&dockerignore_path);
    }
    let gitignore_path = context_path.join(".gitignore");
    if gitignore_path.exists() {
        ignore_builder.add(&gitignore_path);
    }
    let matcher = ignore_builder.build()?;

    // Walk the directory and add files that are not ignored
    for entry in WalkBuilder::new(context_path).build() {
        let entry = entry?;
        let path = entry.path();
        if matcher.matched(path, path.is_dir()).is_ignore() {
            continue; // Skip ignored files
        }

        if path.is_file() {
            let relative_path = path.strip_prefix(context_path)?;
            let mut file = tokio::fs::File::open(path).await?;
            let mut buffer_content = Vec::new();
            file.read_to_end(&mut buffer_content).await?;

            let mut header = Header::new_gnu();
            header.set_size(buffer_content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append_data(&mut header, relative_path, &*buffer_content)
                .await?;
        }
    }

    let result = tar.into_inner().await?;

    Ok(result.clone())
}
