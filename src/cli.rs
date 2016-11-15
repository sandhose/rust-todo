use clap::{App, Arg, SubCommand, AppSettings, ArgGroup};

pub fn build_cli() -> App<'static, 'static> {
    let filter_args = &[
        Arg::from_usage("-a, --all 'show all tasks (including done)'").group("done_filter"),
        Arg::from_usage("-d, --done 'show done tasks'").group("done_filter"),
        Arg::from_usage("-D, --todo 'show to do tasks (default)'").group("done_filter"),
    ];

    let task_arg = Arg::with_name("task")
        .required(true)
        .takes_value(true);

    App::new("todo")
		.about("Task management app")
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::GlobalVersion)
        .arg_from_usage("[file] -f --file=<FILE> 'task file to use (default: $XDG_CONFIG_HOME/rust-todo/tasks.toml)'")
        .subcommand(SubCommand::with_name("list")
            .aliases(&["ls", "l"])
            .about("List tasks")
            .subcommands(vec![
                SubCommand::with_name("all").alias("a").about("show all tasks (including done)"),
                SubCommand::with_name("done").alias("d").about("show done tasks"),
                SubCommand::with_name("todo").alias("t").about("show to do tasks"),
            ])
        )
        .subcommand(SubCommand::with_name("show")
            .aliases(&["s", "view", "v"])
            .about("View a single task")
            .arg(task_arg.clone())
        )
        .subcommand(SubCommand::with_name("remove")
            .aliases(&["del", "delete", "rem"])
            .about("Mark task as done")
            .arg(task_arg.clone())
        )
        .subcommand(SubCommand::with_name("done")
            .aliases(&["ok", "d"])
            .about("Mark task as done")
            .arg(task_arg.clone())
        )
        .subcommand(SubCommand::with_name("todo")
            .aliases(&["not done", "n"])
            .about("Mark task as to be done")
            .arg(task_arg)
        )
        .subcommand(SubCommand::with_name("add")
            .aliases(&["new", "n", "a"])
            .about("Add task")
            .arg(Arg::from_usage("[pr] -p, --priority=[priority] 'Set task priority'")
                .possible_values(&["c", "crit", "critical", "h", "high", "m", "medium", "l", "low"])
                .hide_possible_values(true)
            )
            .args_from_usage(r#"
                -c, --critical  'Set task priority to critical'
                -h, --high      'Set task priority to high'
                -m, --medium    'Set task priority to medium'
                -l, --low       'Set task priority to low'
                <name>          'Task name'
                [details]       'Task details'
            "#)
            .group(ArgGroup::with_name("priority")
                .args(&["pr", "critical", "high", "medium", "low"])
            )
        )
        .subcommand(SubCommand::with_name("test")
            .args(filter_args)
        )
}
