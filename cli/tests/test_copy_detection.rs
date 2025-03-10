// Copyright 2022 The Jujutsu Authors
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

use crate::common::TestEnvironment;

#[test]
fn test_simple_rename() {
    let test_env = TestEnvironment::default();
    test_env.run_jj_in(".", ["git", "init", "repo"]).success();
    let repo_path = test_env.env_root().join("repo");

    test_env.run_jj_in(&repo_path, ["new"]).success();
    std::fs::write(repo_path.join("original"), "original").unwrap();
    std::fs::write(repo_path.join("something"), "something").unwrap();
    test_env
        .run_jj_in(&repo_path, ["commit", "-mfirst"])
        .success();
    std::fs::remove_file(repo_path.join("original")).unwrap();
    std::fs::write(repo_path.join("modified"), "original").unwrap();
    std::fs::write(repo_path.join("something"), "changed").unwrap();
    insta::assert_snapshot!(
        test_env.run_jj_in(&repo_path, ["debug", "copy-detection"]).normalize_backslash(), @r"
    original -> modified
    [EOF]
    ");
}
