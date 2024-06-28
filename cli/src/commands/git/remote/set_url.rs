// Copyright 2024 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use jj_lib::git;
use jj_lib::repo::Repo;

use crate::cli_util::CommandHelper;
use crate::command_error::CommandError;
use crate::git_util::get_git_repo;
use crate::ui::Ui;

/// Set the URL of a Git remote
#[derive(clap::Args, Clone, Debug)]
pub struct GitRemoteSetUrlArgs {
    /// The remote's name
    remote: String,
    /// The desired url for `remote`
    url: String,
}

pub fn cmd_git_remote_set_url(
    ui: &mut Ui,
    command: &CommandHelper,
    args: &GitRemoteSetUrlArgs,
) -> Result<(), CommandError> {
    let workspace_command = command.workspace_helper(ui)?;
    let repo = workspace_command.repo();
    let git_repo = get_git_repo(repo.store())?;
    git::set_remote_url(&git_repo, &args.remote, &args.url)?;
    Ok(())
}
