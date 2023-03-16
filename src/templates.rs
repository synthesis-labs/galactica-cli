use askama::Template;

#[derive(Template)]
#[template(path = "git-commit-hook.txt")]
struct GitCommitHookTemplate<'a> {
    editor: &'a str,
    tty:&'a str,
}

pub fn render_git_commit_hook(editor: &str, tty: &str) -> String {
    let git_commit_hook = GitCommitHookTemplate { editor,tty  };
    format!("{}", git_commit_hook.render().unwrap())
}
