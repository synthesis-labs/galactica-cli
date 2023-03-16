use askama::Template;

#[derive(Template)]
#[template(path = "git-commit-hook.txt")]
struct GitCommitHookTemplate<'a> {
    editor: &'a str,
}

pub fn render_git_commit_hook(editor: &str) -> String {
    let git_commit_hook = GitCommitHookTemplate { editor };
    format!("{}", git_commit_hook.render().unwrap())
}
