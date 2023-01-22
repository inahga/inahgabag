use std::{any::Any, error::Error};

type FnResult = Result<(), Box<dyn Error>>;
type TaskResult<'a> = Result<Task<'a>, Box<dyn Error>>;

struct Task<'a> {
    name: &'a str,
    completed: bool,
}

impl<'a> Task<'a> {
    fn then<F>(&self, name: &'a str, f: F) -> TaskResult
    where
        F: FnOnce() -> FnResult,
    {
        f()?;
        Ok(Task::<'a> {
            name,
            completed: true,
        })
    }
}

fn run<F>(name: &str, f: F) -> TaskResult
where
    F: FnOnce() -> FnResult,
{
    f()?;
    Ok(Task {
        name,
        completed: true,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // stateful task runner
    // run arbitrary functions, but they're reentrant
    // state needs to be serializable
    // run(|| foobar).run(|| foobar)

    run("foo", || {
        println!("foo");
        Ok(())
    })?;

    run("foo", || {
        println!("foo");
        Ok(())
    })?
    .then("bar", || {
        println!("bar");
        Ok(())
    })?;

    Ok(())
}
